use std::fmt::Display;
use std::fmt::{self};
use std::mem;

use crate::visual::light;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Visibility
{
     #[default]
     Invisible,
     Opaque,
     PartialOpaque,
     Transparent,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EmittedMesh
{
     #[default]
     RectilinearFull,
     RectilinearPartial,
     Decorator,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Block
{
     #[default]
     Air,
     Light,
     Plain,
     AlmondWater,
     Distressed1,
     Distressed2,
     Distressed3,
     Corrupt1,
     Corrupt2,
     Corrupt3,
     ExitSign,
     ExitDoor,
     BlockCounter,
}

impl Block
{
     const ALL: [Block; Block::BlockCounter as usize] = [
          Block::Air,
          Block::Light,
          Block::AlmondWater,
          Block::Plain,
          Block::Distressed1,
          Block::Distressed2,
          Block::Distressed3,
          Block::Corrupt1,
          Block::Corrupt2,
          Block::Corrupt3,
          Block::ExitSign,
          Block::ExitDoor,
     ];
     const SPECIAL: [Block; 3] = [Block::Distressed1, Block::Distressed2, Block::Distressed3];
     const CORRUPT: [Block; 3] = [Block::Corrupt1, Block::Corrupt2, Block::Corrupt3];
     const EMPTY: Block = Block::Air;

     pub fn empty() -> Self
     {
          Self::EMPTY
     }

     pub fn all() -> [Self; Self::BlockCounter as usize]
     {
          Self::ALL
     }

     pub fn name(&self) -> &'static str
     {
          match self
          {
               | Block::Air => "air",
               | Block::Plain => "plain",
               | Block::Light => "light",
               | Block::Distressed1 => "distressed1",
               | Block::Distressed2 => "distressed2",
               | Block::Distressed3 => "distressed3",
               | Block::AlmondWater => "almondwater",
               | Block::Corrupt1 => "corrupt1",
               | Block::Corrupt2 => "corrupt2",
               | Block::Corrupt3 => "corrupt3",
               | Block::ExitSign => "exitsign",
               | Block::ExitDoor => "door",
               | Block::BlockCounter => "",
          }
     }

     pub fn opacity(&self) -> light::Light
     {
          match self
          {
               | Block::Air => light::Light::new(0),
               | Block::Light => light::Light::new(0),
               | Block::AlmondWater => light::Light::new(0),
               | Block::ExitSign => light::Light::new(0),
               | Block::Corrupt1 => light::Light::new(3),
               | Block::Corrupt2 => light::Light::new(3),
               | Block::Corrupt3 => light::Light::new(3),
               | _ => light::Light::max_light(),
          }
     }

     pub fn visibility(&self) -> Visibility
     {
          match self
          {
               | Block::Air => Visibility::Invisible,
               | Block::AlmondWater => Visibility::PartialOpaque,
               | _ => Visibility::Opaque,
          }
     }

     pub fn emissivity(&self) -> Option<light::Light>
     {
          match self
          {
               | Block::Light => Some(light::Light::max_light()),
               | Block::Corrupt1 => Some(light::Light::new(2)),
               | Block::Corrupt2 => Some(light::Light::new(3)),
               | Block::Corrupt3 => Some(light::Light::new(4)),
               | Block::ExitSign => Some(light::Light::new(8)),
               | _ => None,
          }
     }

     pub fn mesh_style(&self) -> EmittedMesh
     {
          match self
          {
               | Block::AlmondWater => EmittedMesh::Decorator,
               | _ => EmittedMesh::RectilinearFull,
          }
     }

     pub fn random() -> Self
     {
          Self::ALL[rand::random_range(0 .. Self::BlockCounter as u8) as usize]
     }

     pub fn wall_block(special_chance: f64) -> Self
     {
          if rand::random_bool(special_chance)
          {
               let idx = rand::random_range(0 .. Self::SPECIAL.len());
               return Self::SPECIAL[idx];
          }

          Self::Plain
     }

     pub fn corrupt_block(special_chance: f64) -> Self
     {
          if rand::random_bool(special_chance)
          {
               return Self::Plain;
          }

          let idx = rand::random_range(0 .. Self::CORRUPT.len());
          Self::CORRUPT[idx]
     }
}

impl Display for Block
{
     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
     {
          write!(fmt, "{}", self.name())
     }
}

impl<T> From<T> for Block
where
     T: Into<u8>,
{
     fn from(value: T) -> Self
     {
          unsafe { mem::transmute(value.into()) }
     }
}
