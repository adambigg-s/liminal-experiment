use crate::engine::neighbors;
use crate::world::block;
use crate::world::chunk;
use crate::world::delta;

#[derive(bon::Builder, Debug)]
pub struct TerrainGenerator {}

impl TerrainGenerator
{
     pub fn form_chunk(&self, chunk: &mut chunk::Chunk) -> delta::BlockDeltas
     {
          for i in 0 .. chunk.width() as i32
          {
               for j in 0 .. chunk.width() as i32
               {
                    for k in 0 .. chunk.width() as i32
                    {
                         let coord = glam::ivec3(i, j, k);
                         if neighbors::von_neumann3().iter().any(|&(dx, dy, dz)| {
                              let neighbor_coord = coord + glam::ivec3(dx, dy, dz);
                              !chunk.check_index(neighbor_coord)
                         }) && rand::random_bool(0.1)
                         {
                              *chunk.get_mut(coord) = if rand::random_bool(0.02)
                              {
                                   block::Block::Light
                              }
                              else
                              {
                                   block::Block::Plain
                              };
                         }
                    }
               }
          }

          delta::BlockDeltas::new()
     }
}
