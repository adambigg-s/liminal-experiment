use std::f32;
use std::fs;

use kira::listener;
use kira::sound::static_sound;
use kira::track;
use rustc_hash as rh;

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
     pub zero_velocity: f32,
}

impl PlayerHeadBobber
{
     pub fn new() -> Self
     {
          Self {
               major_freq: f32::consts::PI * 3.0,
               minor_freq: f32::consts::TAU * 4.0,
               major_amp: 0.05,
               minor_amp: 0.0125,
               velocity_coeff: 0.25,
               zero_velocity: 0.05,
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
          let wobble_factor = self.velocity_coeff * velocity + self.zero_velocity;

          right * (cmaj * self.major_amp + cmin * self.minor_amp) * wobble_factor
               + up * (smaj * self.major_amp + smin * self.minor_amp) * wobble_factor
     }
}

#[derive(bon::Builder, Debug)]
pub struct PlayerSoundController
{
     pub named_sounds: rh::FxHashMap<String, static_sound::StaticSoundData>,
     pub walking_sound: Vec<static_sound::StaticSoundData>,
     pub ambience: Option<static_sound::StaticSoundData>,
     pub listener: Option<listener::ListenerHandle>,

     pub spatial_tracks: Vec<track::SpatialTrackHandle>,
     pub tracks: Vec<(static_sound::StaticSoundHandle, &'static str)>,

     last_sound: usize,
     last_sound_time: f32,
     sound_delay: f32,
     velocity_coeff: f32,
}

impl PlayerSoundController
{
     pub fn new(path: &'static str) -> anyhow::Result<Self>
     {
          let mut ambience = None;
          let mut walking_sound = Vec::new();
          let mut named_sounds = rh::FxHashMap::default();

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
                    named_sounds.insert(stem.to_string(), static_sound::StaticSoundData::from_file(path)?);
               }
          }

          let listener = None;
          let spatial_tracks = Vec::new();
          let tracks = Vec::new();

          Ok(Self {
               named_sounds,
               walking_sound,
               ambience,
               listener,
               spatial_tracks,
               tracks,

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
               return;
          }

          log::error!("Error playing ambience track");
     }

     pub fn named_sound(&mut self, audio: &mut kira::AudioManager, name: &'static str)
     {
          if let Some(sound) = self.named_sounds.get(name)
          {
               self.tracks.push((audio.play(sound.clone()).unwrap(), name));
               return;
          }

          log::error!("Error playing named audio");
     }

     pub fn named_sound_attenuated(&mut self, audio: &mut kira::AudioManager, name: &'static str, db: f32)
     {
          if let Some(sound) = self.named_sounds.get(name)
          {
               self.tracks.push((audio.play(sound.clone().volume(db)).unwrap(), name));
               return;
          }

          log::error!("Error playing named audio");
     }

     pub fn named_sound_stop(&mut self, name: &'static str)
     {
          self.tracks.iter_mut().for_each(|(handle, track_name)| {
               if *track_name == name
               {
                    handle.stop(kira::Tween::default());
               }
          });
     }

     pub fn named_sound_directional(
          &mut self,
          audio: &mut kira::AudioManager,
          name: &str,
          location: glam::Vec3,
     )
     {
          self.spatial_tracks.retain_mut(|track| track.state() != track::TrackPlaybackState::Paused);
          if let (Some(sound), Some(listener)) = (self.named_sounds.get(name), self.listener.as_ref())
          {
               let mut track = audio
                    .add_spatial_sub_track(listener, location, kira::track::SpatialTrackBuilder::default())
                    .unwrap();
               track.play(sound.clone()).unwrap();
               self.spatial_tracks.push(track);
               return;
          }

          log::error!("Error playing directional audio");
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
               let attenuation = (self.velocity_coeff * kinematics.velocity.length()).powf(2.0);
               audio.play(
                    self.walking_sound[self.last_sound % self.walking_sound.len()]
                         .clone()
                         .volume((rand::random_range(-15.0 .. -10.0) + attenuation).min(0.0)),
               )
               .unwrap();
               self.last_sound += 1;
               self.last_sound_time = time;
          }
     }
}
