pub mod dark_maze;
pub mod debugging_biome;
pub mod empty;
pub mod empty_dark;
pub mod maze;
pub mod parkour;
pub mod pillars;
pub mod pitfalls;
pub mod superliminal;

use std::fmt::Debug;

use crate::world::chunk;
use crate::world::delta;

#[derive(bon::Builder, Debug)]
pub struct NoiseLayer
{
     pub freq: glam::DVec3,
     pub offset: glam::DVec3,
}

impl NoiseLayer
{
     pub fn sample<Noise>(&self, noise: Noise, point: glam::DVec3) -> f64
     where
          Noise: noise::NoiseFn<f64, 2> + noise::NoiseFn<f64, 3>,
     {
          let sample_point = point * self.freq + self.offset;
          (noise.get(sample_point.to_array()) + 1.0) * 0.5
     }
}

pub trait BiomeGeneration
where
     Self: Debug + Send + Sync,
{
     fn generate(
          &self,
          chunk: &mut chunk::Chunk,
          noise: &noise::Perlin,
          config: &TerrainConfig,
          deltas: &mut delta::BlockDeltas,
     );
}

#[derive(bon::Builder, Debug)]
pub struct BiomePoint
{
     biome_center: f64,
     weird_center: f64,
     #[builder(default = 1.0)]
     weight: f64,
     generator: Box<dyn BiomeGeneration>,
}

#[derive(bon::Builder, Debug)]
pub struct TerrainConfig
{
     pub biome_noise: NoiseLayer,
     pub weird_noise: NoiseLayer,
     pub feature_noise: NoiseLayer,
     pub random_noise: NoiseLayer,
}

#[derive(bon::Builder, Debug)]
pub struct TerrainGenerator
{
     pub noise: noise::Perlin,
     pub config: TerrainConfig,
     pub seed: u32,
     pub biome_map: Vec<BiomePoint>,
}

impl TerrainGenerator
{
     pub fn new(seed: u32) -> Self
     {
          let noise = noise::Perlin::new(seed);
          let config = TerrainConfig::builder()
               .biome_noise(
                    NoiseLayer::builder()
                         .offset(glam::DVec3::splat(0.9207135))
                         .freq(glam::dvec3(0.25, 0.025, 0.25))
                         .build(),
               )
               .weird_noise(
                    NoiseLayer::builder()
                         .offset(glam::DVec3::splat(-90.18973095))
                         .freq(glam::dvec3(0.5, 0.1, 0.5))
                         .build(),
               )
               .feature_noise(
                    NoiseLayer::builder()
                         .offset(glam::DVec3::splat(-8000.09238427))
                         .freq(glam::dvec3(0.3, 0.3, 0.3))
                         .build(),
               )
               .random_noise(
                    NoiseLayer::builder()
                         .offset(glam::DVec3::splat(-20202.234234234))
                         .freq(glam::dvec3(999.999, 999.999, 999.999))
                         .build(),
               )
               .build();

          let biome_map = vec![
               BiomePoint::builder()
                    .biome_center(0.8)
                    .weird_center(0.4)
                    .generator(Box::new(parkour::Parkour))
                    .build(),
               BiomePoint::builder()
                    .biome_center(0.5)
                    .weird_center(0.5)
                    .weight(1.5)
                    .generator(Box::new(maze::Maze))
                    // .generator(Box::new(debugging_biome::DebuggingBiome))
                    .build(),
               BiomePoint::builder()
                    .biome_center(0.55)
                    .weird_center(0.55)
                    .weight(0.5)
                    .generator(Box::new(dark_maze::DarkMaze))
                    // .generator(Box::new(debugging_biome::DebuggingBiome))
                    .build(),
               BiomePoint::builder()
                    .biome_center(0.65)
                    .weird_center(0.65)
                    .weight(0.5)
                    .generator(Box::new(empty_dark::EmptyDark))
                    // .generator(Box::new(debugging_biome::DebuggingBiome))
                    .build(),
               BiomePoint::builder()
                    .biome_center(0.5)
                    .weird_center(0.1)
                    .generator(Box::new(pillars::Pillars))
                    .build(),
               BiomePoint::builder()
                    .biome_center(0.6)
                    .weird_center(0.9)
                    .weight(1.5)
                    .generator(Box::new(pitfalls::Pitfalls))
                    .build(),
               BiomePoint::builder()
                    .biome_center(0.1)
                    .weird_center(0.9)
                    .generator(Box::new(superliminal::SuperLiminal))
                    .build(),
          ];

          Self {
               noise,
               config,
               seed,
               biome_map,
          }
     }

     pub fn classify(&self, biome: f64, weird: f64) -> &dyn BiomeGeneration
     {
          let sample_point = glam::dvec2(biome, weird);

          let mut closest = &self.biome_map[0].generator;
          let mut min_distance = f64::MAX;

          for point in &self.biome_map
          {
               let ideal = glam::dvec2(point.biome_center, point.weird_center);
               let distance = sample_point.distance_squared(ideal);
               let biased = distance / point.weight;

               if biased < min_distance
               {
                    closest = &point.generator;
                    min_distance = distance;
               }
          }

          closest.as_ref()
     }

     pub fn form_chunk(&self, chunk: &mut chunk::Chunk) -> delta::BlockDeltas
     {
          let mut outgoing_deltas = delta::BlockDeltas::new();

          let coord = chunk.offset().as_dvec3();
          let biome = self.config.biome_noise.sample(self.noise, coord);
          let weird = self.config.weird_noise.sample(self.noise, coord);

          if (-1 ..= 1).contains(&chunk.offset().x) && (-1 ..= 1).contains(&chunk.offset().z)
          {
               empty_dark::EmptyDark.generate(chunk, &self.noise, &self.config, &mut outgoing_deltas);
               // debugging_biome::DebuggingBiome.generate(chunk, &self.noise, &self.config, &mut outgoing_deltas);
          }
          else
          {
               let biome = self.classify(biome, weird);
               biome.generate(chunk, &self.noise, &self.config, &mut outgoing_deltas);
          }

          outgoing_deltas
     }
}
