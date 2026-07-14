#![allow(unused)]

use crate::engine::neighbors;
use crate::terrain;
use crate::world;
use crate::world::block;
use crate::world::delta;

#[derive(Debug)]
pub struct Escape;

impl terrain::BiomeGeneration for Escape
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
               for x in 0 .. size.x
               {
                    let coord = glam::ivec3(x, size.y - 1, z);
                    *chunk.get_mut(coord) = block::Block::wall_block(0.5);
               }
          }

          if chunk.world_position().y > 32
          {
               *chunk.get_mut(glam::ivec3(size.x / 2, 0, size.z / 2)) = block::Block::ExitDoor;
               *chunk.get_mut(glam::ivec3(size.x / 2, 1, size.z / 2)) = block::Block::ExitDoor;
               *chunk.get_mut(glam::ivec3(size.x / 2, 2, size.z / 2)) = block::Block::ExitSign;
          }
     }
}
