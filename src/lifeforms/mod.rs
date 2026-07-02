use crate::engine::player;

pub trait LifeForm
{
     fn update(&self, player_info: &player::PlayerController, dt: f32);

     fn special_event(&self, player_info: &player::PlayerController);
}

#[derive(bon::Builder, Debug)]
pub struct FollowCube
{
     pub health: u32,
}
