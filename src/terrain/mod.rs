use crate::world::chunk;
use crate::world::delta;

#[derive(bon::Builder, Debug)]
pub struct TerrainGenerator {}

impl TerrainGenerator
{
     pub fn form_chunk(&self, chunk: &mut chunk::Chunk) -> delta::BlockDeltas
     {
          delta::BlockDeltas::new()
     }
}
