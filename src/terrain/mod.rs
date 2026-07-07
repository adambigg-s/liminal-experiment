pub mod debugging_biome;
pub mod empty;
pub mod maze;
pub mod parkour;
pub mod pillars;
pub mod pitfalls;
pub mod superliminal;

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

pub trait BiomeTrait
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
pub struct TerrainConfig
{
     pub biome_noise: NoiseLayer,
     pub weird_noise: NoiseLayer,
     pub feature_noise: NoiseLayer,
}

#[derive(bon::Builder, Debug)]
pub struct TerrainGenerator
{
     pub noise: noise::Perlin,
     pub config: TerrainConfig,
     pub seed: u32,
}

impl TerrainGenerator
{
     pub fn new(seed: u32) -> Self
     {
          let noise = noise::Perlin::new(seed);
          let config = TerrainConfig::builder()
               .biome_noise(
                    NoiseLayer::builder()
                         .offset(glam::DVec3::splat(0.0))
                         .freq(glam::dvec3(0.5, 0.015, 0.5))
                         .build(),
               )
               .weird_noise(
                    NoiseLayer::builder()
                         .offset(glam::DVec3::splat(250.0))
                         .freq(glam::dvec3(0.5, 0.1, 0.5))
                         .build(),
               )
               .feature_noise(
                    NoiseLayer::builder()
                         .offset(glam::DVec3::splat(-8000.0))
                         .freq(glam::dvec3(0.3, 0.3, 0.3))
                         .build(),
               )
               .build();

          Self {
               noise,
               config,
               seed,
          }
     }

     pub fn classify(&self, biome: f64, weird: f64) -> Box<dyn BiomeTrait>
     {
          match (biome, weird)
          {
               | (b, w) if b < 0.3 && w > 0.8 => Box::new(superliminal::SuperLiminal),
               | (b, w) if b > 0.4 && w < 0.3 => Box::new(pillars::Pillars),
               | (b, _) if b > 0.3 => Box::new(maze::Maze),
               // | (b, _) if b > 0.3 => Box::new(debugging_biome::DebuggingBiome),
               | (_, w) if w > 0.55 => Box::new(pitfalls::Pitfalls),
               | (b, _) if b < 0.25 => Box::new(empty::Empty),
               | _ => Box::new(parkour::Parkour),
               // | _ => Box::new(debugging_biome::DebuggingBiome),
          }
     }

     pub fn form_chunk(&self, chunk: &mut chunk::Chunk) -> delta::BlockDeltas
     {
          let mut outgoing_deltas = delta::BlockDeltas::new();

          let coord = chunk.offset().as_dvec3();
          let biome = self.config.biome_noise.sample(self.noise, coord);
          let weird = self.config.weird_noise.sample(self.noise, coord);

          let biome = self.classify(biome, weird);
          biome.generate(chunk, &self.noise, &self.config, &mut outgoing_deltas);

          outgoing_deltas
     }
}
