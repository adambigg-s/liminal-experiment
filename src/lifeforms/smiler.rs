#![allow(unused)]

use crate::engine::player;
use crate::engine::transform;
use crate::lifeforms;
use crate::render;

#[derive(bon::Builder, Debug)]
pub struct FollowCube
{
     pub transform: transform::Transform,
     pub health: u32,
}

impl lifeforms::LifeForm for FollowCube
{
     fn new(context: &mut render::GfxContext, render: &mut render::GfxRenderer) -> Self
     {
          todo!()
     }

     fn update(&self, player_info: &player::PlayerController, dt: f32)
     {
          todo!()
     }

     fn gfx_sync(&self, context: &mut render::GfxContext, render: &mut render::GfxRenderer)
     {
          todo!()
     }

     fn special_event(&self, player_info: &player::PlayerController)
     {
          todo!()
     }

     fn cleanup(&self)
     {
          todo!()
     }
}
