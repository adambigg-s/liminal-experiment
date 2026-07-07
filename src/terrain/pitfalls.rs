#![allow(unused)]

use crate::engine::neighbors;
use crate::terrain;
use crate::world;
use crate::world::block;
use crate::world::delta;

pub struct Pitfalls;

impl terrain::BiomeTrait for Pitfalls
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
                    let world_coord = chunk.world_position() + glam::ivec3(x, -1, z);
                    let chunk_world_coord = chunk.chunk_world_coords(world_coord);
                    let chunk_coord = chunk.to_chunk_coords(world_coord);
                    deltas.insert(
                         chunk_world_coord,
                         delta::ChunkDelta {
                              coord: chunk_coord,
                              delta: block::Block::Air,
                         },
                    );

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
}
