use crate::engine::neighbors;
use crate::world::block;
use crate::world::chunk;
use crate::world::delta;

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

#[derive(bon::Builder, Debug)]
pub struct TerrainGenerator
{
     pub noise: noise::Perlin,
     pub seed: u32,
}

impl TerrainGenerator
{
     pub fn new(seed: u32) -> Self
     {
          let noise = noise::Perlin::new(seed);

          Self {
               noise,
               seed,
          }
     }

     pub fn form_chunk(&self, chunk: &mut chunk::Chunk) -> delta::BlockDeltas
     {
          for z in 0 .. chunk.width() as i32
          {
               for y in 0 .. chunk.height() as i32
               {
                    for x in 0 .. chunk.width() as i32
                    {
                         let coord = glam::ivec3(x, y, z);
                         if neighbors::von_neumann3().iter().any(|&(dx, dy, dz)| {
                              let neighbor_coord = coord + glam::ivec3(dx, dy, dz);
                              !chunk.check_index(neighbor_coord)
                         }) && rand::random_bool(0.1)
                         {
                              *chunk.get_mut(coord) = if rand::random_bool(0.02)
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
                              else
                              {
                                   block::Block::Distressed1
                              }
                         }
                    }
               }
          }

          delta::BlockDeltas::new()
     }
}
