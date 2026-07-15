#![allow(unused)]

use crate::engine::neighbors;
use crate::terrain;
use crate::world;
use crate::world::block;
use crate::world::delta;

#[derive(Debug)]
pub struct SuperLiminal;

impl terrain::BiomeGeneration for SuperLiminal
{
     fn generate(
          &self,
          chunk: &mut world::chunk::Chunk,
          noise: &noise::Perlin,
          config: &terrain::TerrainConfig,
          deltas: &mut delta::BlockDeltas,
     )
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
                              else if rand::random_bool(0.5)
                              {
                                   block::Block::wall_block(0.5)
                              }
                              else
                              {
                                   block::Block::corrupt_block(0.5)
                              }
                         }
                    }
               }
          }
     }

     fn as_any(&self) -> &dyn std::any::Any
     {
          self
     }
}
