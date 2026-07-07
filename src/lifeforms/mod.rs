pub mod smiler;

use crate::engine::player;
use crate::render;

pub trait LifeForm
{
     fn new(context: &mut render::GfxContext, render: &mut render::GfxRenderer) -> Self;

     fn update(&self, player_info: &player::PlayerController, dt: f32);

     fn gfx_sync(&self, context: &mut render::GfxContext, render: &mut render::GfxRenderer);

     fn special_event(&self, player_info: &player::PlayerController);

     fn cleanup(&self);
}
