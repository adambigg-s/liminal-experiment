use std::f32;
use std::fmt::Display;
use std::fmt::{self};
use std::mem;

use crate::engine::transform;
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum EmittedMesh
{
     #[default]
     RectilinearFull,
     Decorator,
     RectilinearPartial(transform::Transform),
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
     NotExit,
     Tape,
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
          Block::NotExit,
          Block::Tape,
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
               | Block::NotExit => "notexit",
               | Block::Tape => "tape",
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
               | Block::ExitDoor => light::Light::new(0),
               | Block::NotExit => light::Light::new(0),
               | Block::Corrupt1 => light::Light::new(3),
               | Block::Corrupt2 => light::Light::new(3),
               | Block::Corrupt3 => light::Light::new(3),
               | Block::Tape => light::Light::new(0),
               | _ => light::Light::max_light(),
          }
     }

     pub fn visibility(&self) -> Visibility
     {
          match self
          {
               | Block::Air => Visibility::Invisible,
               | Block::AlmondWater => Visibility::PartialOpaque,
               | Block::Tape => Visibility::PartialOpaque,
               | _ => Visibility::Opaque,
          }
     }

     pub fn emissivity(&self) -> Option<light::Light>
     {
          match self
          {
               | Block::Light => Some(light::Light::max_light()),
               | Block::Corrupt1 => Some(light::Light::new(5)),
               | Block::Corrupt2 => Some(light::Light::new(5)),
               | Block::Corrupt3 => Some(light::Light::new(5)),
               | Block::ExitSign => Some(light::Light::new(9)),
               | Block::ExitDoor => Some(light::Light::new(4)),
               | Block::NotExit => Some(light::Light::new(6)),
               | _ => None,
          }
     }

     pub fn mesh_style(&self) -> EmittedMesh
     {
          match self
          {
               | Block::AlmondWater =>
               {
                    EmittedMesh::RectilinearPartial(transform::Transform::new(
                         glam::vec3(0.0, -0.25, 0.0),
                         glam::Quat::from_mat3(&glam::Mat3::from_rotation_y(f32::consts::FRAC_2_SQRT_PI)),
                         glam::vec3(0.15, 0.5, 0.15),
                    ))
               }
               | Block::Tape =>
               {
                    EmittedMesh::RectilinearPartial(transform::Transform::new(
                         glam::vec3(0.0, -0.45, 0.0),
                         glam::Quat::from_mat3(&glam::Mat3::from_rotation_y(f32::consts::FRAC_PI_3)),
                         glam::vec3(0.7, 0.1, 0.3),
                    ))
               }
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
