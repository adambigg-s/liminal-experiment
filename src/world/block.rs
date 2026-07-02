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
     Plain,
     Light,
     Distressed1,
     AlmondWater,
     BlockCounter,
}

impl Block
{
     const ALL: [Block; Block::BlockCounter as usize] = [
          Block::Air,
          Block::Plain,
          Block::Light,
          Block::Distressed1,
          Block::AlmondWater,
     ];
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
               | Block::AlmondWater => "almondwater",
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
