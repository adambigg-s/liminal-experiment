#![allow(unused)]

use crate::engine::neighbors;
use crate::terrain;
use crate::world;
use crate::world::block;
use crate::world::delta;

pub struct Empty;

impl terrain::BiomeTrait for Empty
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
                         *chunk.get_mut(coord) = block::Block::Plain;
                    }
               }
          }
     }
}
