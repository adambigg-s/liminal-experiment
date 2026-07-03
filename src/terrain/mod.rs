use std::arch::x86_64::_MM_FROUND_TO_NEG_INF;

use crate::engine::neighbors;
use crate::world::block;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Biome
{
     Maze,
     Pillars,
     Pitfalls,
     Empty,
     Parkour,
     SuperLiminal,
}

impl Biome
{
     pub fn classify(biome: f64, weird: f64) -> Self
     {
          match (biome, weird)
          {
               | (b, w) if b < 0.3 && w > 0.9 => Self::SuperLiminal,
               | (b, w) if b > 0.6 && w < 0.3 => Self::Pillars,
               | (b, _) if b > 0.3 => Self::Maze,
               | (_, w) if w > 0.8 => Self::Parkour,
               | (b, _) if b < 0.1 => Self::Empty,
               | _ => Self::Pitfalls,
          }
     }

     pub fn generate(
          &self,
          chunk: &mut chunk::Chunk,
          noise: &noise::Perlin,
          config: &TerrainConfig,
          deltas: &mut delta::BlockDeltas,
     )
     {
          match self
          {
               | Biome::Maze => self.make_maze(chunk, noise, config, deltas),
               | Biome::Pillars => self.make_pillars(chunk, config),
               | Biome::Pitfalls => self.make_pitfalls(chunk, config, deltas),
               | Biome::Empty => self.make_empty(chunk),
               | Biome::Parkour => self.make_parkour(chunk, noise, config),
               | Biome::SuperLiminal => self.make_superliminal(chunk),
          }
     }

     pub fn make_maze(
          &self,
          chunk: &mut chunk::Chunk,
          noise: &noise::Perlin,
          config: &TerrainConfig,
          deltas: &mut delta::BlockDeltas,
     )
     {
          let size = chunk.size();
          for z in 0 .. size.z
          {
               for x in 0 .. size.x
               {
                    let coord = glam::ivec3(x, size.y - 1, z);
                    *chunk.get_mut(coord) = block::Block::Plain;

                    let coord = glam::ivec3(x, size.y - 2, z);
                    *chunk.get_mut(coord) = block::Block::Plain;
                    if x % 5 == 0 && z % 5 == 0 && config.feature_noise.sample(noise, coord.as_dvec3()) > 0.5
                    {
                         *chunk.get_mut(coord) = block::Block::Light
                    }

                    let coord = glam::ivec3(x, 0, z);
                    if config.feature_noise.sample(noise, coord.as_dvec3()) > 0.9
                    {
                         *chunk.get_mut(coord) = block::Block::AlmondWater;
                    }
               }
          }
     }

     pub fn make_empty(&self, chunk: &mut chunk::Chunk)
     {
          let size = chunk.size();
          for z in 0 .. size.z
          {
               for y in 0 .. size.y
               {
                    for x in 0 .. size.x
                    {
                         let coord = glam::ivec3(x, y, z);
                         *chunk.get_mut(coord) = block::Block::Plain;
                    }
               }
          }
     }

     pub fn make_superliminal(&self, chunk: &mut chunk::Chunk)
     {
          let size = chunk.size();
          for z in 0 .. size.z
          {
               for y in 0 .. size.y
               {
                    for x in 0 .. size.x
                    {
                         let coord = glam::ivec3(x, y, z);
                         if neighbors::von_neumann3().iter().any(|&(dx, dy, dz)| {
                              let neighbor_coord = coord + glam::ivec3(dx, dy, dz);
                              !chunk.check_index(neighbor_coord)
                         }) && rand::random_bool(0.025)
                         {
                              *chunk.get_mut(coord) = if rand::random_bool(0.01)
                              {
                                   block::Block::Light
                              }
                              else if rand::random_bool(0.05)
                              {
                                   block::Block::AlmondWater
                              }
                              else if rand::random_bool(0.5)
                              {
                                   block::Block::Plain
                              }
                              else if rand::random_bool(0.5)
                              {
                                   block::Block::Distressed1
                              }
                              else
                              {
                                   block::Block::Corrupt1
                              }
                         }
                    }
               }
          }
     }

     fn make_pitfalls(&self, chunk: &mut chunk::Chunk, _: &TerrainConfig, deltas: &mut delta::BlockDeltas)
     {
          let size = chunk.size();
          for z in 0 .. size.z
          {
               for x in 0 .. size.x
               {
                    if x % 8 != 0 && z % 8 != 0
                    {
                         continue;
                    }

                    let coord = glam::ivec3(x, 0, z);
                    if x % 5 == 0 && z % 5 == 0
                    {
                         *chunk.get_mut(coord) = block::Block::Light
                    }
                    else
                    {
                         *chunk.get_mut(coord) = block::Block::Plain
                    }

                    for y in 1 .. size.y
                    {
                         let coord = glam::ivec3(x, y, z);
                         *chunk.get_mut(coord) = block::Block::Plain;
                    }
               }
          }
     }

     fn make_parkour(&self, chunk: &mut chunk::Chunk, noise: &noise::Perlin, config: &TerrainConfig)
     {
          let size = chunk.size();
          for z in 0 .. size.z
          {
               for y in 0 .. size.y
               {
                    for x in 0 .. size.x
                    {
                         let coord = glam::ivec3(x, y, z);

                         if config.feature_noise.sample(noise, coord.as_dvec3()) > 0.75
                         {
                              *chunk.get_mut(coord) = block::Block::Corrupt1;
                         }
                    }
               }
          }
     }

     fn make_pillars(&self, chunk: &mut chunk::Chunk, config: &TerrainConfig)
     {
          let size = chunk.size();
          for z in 0 .. size.z
          {
               for x in 0 .. size.x
               {
                    let coord = glam::ivec3(x, size.y - 1, z);
                    *chunk.get_mut(coord) = block::Block::Plain;

                    let coord = glam::ivec3(x, size.y - 2, z);
                    *chunk.get_mut(coord) = block::Block::Plain;
                    if x % 8 == 0 && z % 8 == 0
                    {
                         *chunk.get_mut(coord) = block::Block::Light;
                    }

                    if !(x % 5 == 0 && z % 5 == 0)
                    {
                         continue;
                    }

                    for y in 0 .. size.y - 2
                    {
                         let coord = glam::ivec3(x, y, z);
                         *chunk.get_mut(coord) = block::Block::Distressed1;
                    }
               }
          }
     }
}

#[derive(bon::Builder, Debug)]
pub struct TerrainConfig
{
     // pub biome_freq: glam::DVec3,
     // pub special_freq: glam::DVec3,
     // pub biome_offset: glam::DVec3,
     // pub special_offset: glam::DVec3,
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
                         .freq(glam::dvec3(0.5, 0.05, 0.5))
                         .build(),
               )
               .weird_noise(
                    NoiseLayer::builder()
                         .offset(glam::DVec3::splat(250.0))
                         .freq(glam::dvec3(0.5, 0.5, 0.5))
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

     pub fn form_chunk(&self, chunk: &mut chunk::Chunk) -> delta::BlockDeltas
     {
          let mut outgoing_deltas = delta::BlockDeltas::new();

          let coord = chunk.offset().as_dvec3();
          let biome = self.config.biome_noise.sample(self.noise, coord);
          let weird = self.config.weird_noise.sample(self.noise, coord);

          let biome = Biome::classify(biome, weird);
          biome.generate(chunk, &self.noise, &self.config, &mut outgoing_deltas);

          outgoing_deltas
     }
}
