#![allow(unused)]

use crate::engine::neighbors;
use crate::terrain;
use crate::world;
use crate::world::block;
use crate::world::delta;

#[derive(Debug)]
pub struct Maze;

impl terrain::BiomeGeneration for Maze
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
                    *chunk.get_mut(coord) = block::Block::wall_block(0.05);

                    let coord = glam::ivec3(x, size.y - 2, z);
                    *chunk.get_mut(coord) = block::Block::wall_block(0.05);

                    if x % 5 == 0 && z % 5 == 0 && config.feature_noise.sample(noise, coord.as_dvec3()) > 0.45
                    {
                         *chunk.get_mut(coord) = block::Block::Light
                    }

                    let coord = glam::ivec3(x, 0, z);
                    let world_coord = chunk.world_position() + coord;
                    if config.weird_noise.sample(noise, world_coord.as_dvec3()) > 0.9
                    {
                         *chunk.get_mut(coord) = block::Block::AlmondWater;
                    }

                    // if config.feature_noise.sample(noise, world_coord.as_dvec3()) > 0.8
                    // {
                    //      let bias = if rand::random_bool(0.5)
                    //      {
                    //           if rand::random_bool(0.5)
                    //           {
                    //                glam::ivec3(1, 0, 0)
                    //           }
                    //           else
                    //           {
                    //                glam::ivec3(-1, 0, 0)
                    //           }
                    //      }
                    //      else
                    //      {
                    //           if rand::random_bool(0.5)
                    //           {
                    //                glam::ivec3(0, 0, 1)
                    //           }
                    //           else
                    //           {
                    //                glam::ivec3(0, 0, -1)
                    //           }
                    //      };

                    //      let mut coord = glam::ivec3(x, 0, z);
                    //      for _ in 0 .. 10
                    //      {
                    //           let offset =
                    //                glam::ivec3(rand::random_range(-1 ..= 1), 0, rand::random_range(-1 ..= 1));
                    //           coord += offset + bias;
                    //           if !chunk.check_index(coord)
                    //           {
                    //                break;
                    //           }

                    //           for y in 0 .. size.y - 2
                    //           {
                    //                let coord = coord.with_y(y);
                    //                *chunk.get_mut(coord) = block::Block::wall_block(0.01);
                    //           }
                    //      }
                    // }
               }
          }

          for x in 0 .. size.x
          {
               for z in 0 .. size.z
               {
                    if rand::random_bool(0.00025)
                    {
                         for i in 0 .. 2
                         {
                              for j in 0 .. 2
                              {
                                   let coord = glam::ivec3(x + i, 0, z + j);
                                   if !chunk.check_index(coord)
                                   {
                                        continue;
                                   }

                                   let coord = coord.with_y(size.y - 1);
                                   *chunk.get_mut(coord) = block::Block::Air;

                                   let coord = coord.with_y(size.y - 2);
                                   *chunk.get_mut(coord) = block::Block::Air;
                              }
                         }
                    }
               }
          }
     }
}
