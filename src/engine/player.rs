use std::f32;
use std::fs;

use kira::sound::static_sound;

use crate::engine::aabb;
use crate::engine::camera;
use crate::engine::kinematics;

#[derive(bon::Builder, Debug)]
pub struct PlayerController
{
     pub movespeed: f32,
     pub lookspeed: f32,
     pub collider: aabb::AaBb<f32, 3>,
     pub kinematics: kinematics::Kinematics,
     #[builder(default)]
     pub collisions: bool,
}

impl PlayerController
{
     pub fn jiggle_free<Collider>(&mut self, world: &Collider)
     where
          Collider: kinematics::Collision<Collider = kinematics::BoxCollider>,
     {
          while world.collides(self.collider)
          {
               self.collider = self.collider
                    + glam::vec3(
                         rand::random_range(-1.0 ..= 1.0),
                         rand::random_range(-1.0 ..= 1.0),
                         rand::random_range(-1.0 ..= 1.0),
                    );
          }
     }
}

#[derive(bon::Builder, Debug, Default)]
pub struct PlayerHeadBobber
{
     pub major_freq: f32,
     pub minor_freq: f32,
     pub major_amp: f32,
     pub minor_amp: f32,

     velocity_coeff: f32,
}

impl PlayerHeadBobber
{
     pub fn new() -> Self
     {
          Self {
               major_freq: f32::consts::PI * 2.2,
               minor_freq: f32::consts::TAU * 3.3,
               major_amp: 0.075,
               minor_amp: 0.0125,
               velocity_coeff: 1.0 / 3.0,
          }
     }

     pub fn head_bob(
          &self,
          camera: &camera::Camera,
          kinematics: &kinematics::Kinematics,
          time: f32,
     ) -> glam::Vec3
     {
          let (smaj, cmaj) = (time * self.major_freq).sin_cos();
          let (smin, cmin) = (time * self.minor_freq).sin_cos();
          let up = camera.inner.up();
          let right = camera.inner.right();
          let velocity = kinematics.velocity.with_y(0.0).length();

          right * (cmaj * self.major_amp + cmin * self.minor_amp) * self.velocity_coeff * velocity
               + up * (smaj * self.major_amp + smin * self.minor_amp) * self.velocity_coeff * velocity
     }
}

#[derive(bon::Builder, Debug)]
pub struct PlayerSoundController
{
     pub walking_sound: Vec<static_sound::StaticSoundData>,
     pub interaction_sound: Option<static_sound::StaticSoundData>,
     pub ambience: Option<static_sound::StaticSoundData>,

     last_sound: usize,
     last_sound_time: f32,
     sound_delay: f32,
     velocity_coeff: f32,
}

impl PlayerSoundController
{
     pub fn new(path: &'static str) -> anyhow::Result<Self>
     {
          let mut interaction_sound = None;
          let mut ambience = None;
          let mut walking_sound = Vec::new();

          let entries = fs::read_dir(path)?;
          for entry in entries
          {
               let entry = entry?;
               let path = entry.path();

               if !path.is_file()
               {
                    continue;
               }

               let ext = path.extension().and_then(|extension| extension.to_str());
               if ext != Some("wav") && ext != Some("mp3")
               {
                    log::error!("Attempted read on invalid file: {:?}", path);
                    continue;
               }

               let stem = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or_else(|| anyhow::anyhow!("Invalid file name: {:?}", path))?;

               if stem.contains("step")
               {
                    walking_sound.push(static_sound::StaticSoundData::from_file(path)?);
               }
               else if stem.contains("amb")
               {
                    ambience =
                         Some(static_sound::StaticSoundData::from_file(path)?.loop_region(..).volume(0.5))
               }
               else
               {
                    interaction_sound = Some(static_sound::StaticSoundData::from_file(path)?);
               }
          }

          Ok(Self {
               walking_sound,
               interaction_sound,
               ambience,

               last_sound: 0,
               last_sound_time: 0.0,
               sound_delay: 1.2,
               velocity_coeff: 0.5,
          })
     }

     pub fn ambience(&self, audio: &mut kira::AudioManager)
     {
          if let Some(sound) = &self.ambience
          {
               audio.play(sound.clone()).unwrap();
          }
     }

     pub fn interaction(&self, audio: &mut kira::AudioManager)
     {
          if let Some(sound) = &self.interaction_sound
          {
               audio.play(sound.clone()).unwrap();
          }
     }

     pub fn movement(
          &mut self,
          audio: &mut kira::AudioManager,
          kinematics: &kinematics::Kinematics,
          time: f32,
     )
     {
          let diff = time - self.last_sound_time;
          if diff > self.sound_delay / (self.velocity_coeff * kinematics.velocity.length())
               && !kinematics.flying
          {
               audio.play(self.walking_sound[self.last_sound % self.walking_sound.len()].clone().volume(
                    rand::random_range(-20.0 .. -15.0)
                         + (self.velocity_coeff * kinematics.velocity.length()).powf(2.0),
               ))
               .unwrap();
               self.last_sound += 1;
               self.last_sound_time = time;
          }
     }
}
