#![allow(unused)]

use crate::engine::neighbors;
use crate::terrain;
use crate::world;
use crate::world::block;
use crate::world::delta;

pub struct Maze;

impl terrain::BiomeTrait for Maze
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
}
