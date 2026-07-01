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
                         let block = block::Block::random();
                         if rand::random_bool(0.1)
                         {
                              *chunk.get_mut(glam::ivec3(i, j, k)) = block;
                         }
                    }
               }
          }

          delta::BlockDeltas::new()
     }
}
