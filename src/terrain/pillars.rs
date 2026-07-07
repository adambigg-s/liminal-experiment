#![allow(unused)]

use crate::engine::neighbors;
use crate::terrain;
use crate::world;
use crate::world::block;
use crate::world::delta;

pub struct Pillars;

impl terrain::BiomeTrait for Pillars
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
                    *chunk.get_mut(coord) = block::Block::wall_block(0.05);;
                    let coord = glam::ivec3(x, size.y - 2, z);
                    *chunk.get_mut(coord) = block::Block::wall_block(0.05);;

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
                         *chunk.get_mut(coord) = block::Block::wall_block(0.075);
                    }
               }
          }
     }
}
