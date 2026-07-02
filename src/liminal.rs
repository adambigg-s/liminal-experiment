use std::env;
use std::sync;

use crate::application;
use crate::application::input;
use crate::engine;
use crate::engine::camera;
use crate::engine::kinematics;
use crate::engine::player;
use crate::render;
use crate::render::GfxCamera;
use crate::render::resource;
use crate::render::util;
use crate::terrain;
use crate::visual::atlas;
use crate::visual::pipelines;
use crate::world::manager;

#[derive(bon::Builder, Debug)]
pub struct Liminal
{
     pub camera: camera::Camera,
     pub player: player::PlayerController,
     pub frame: engine::FrameData,

     pub world: manager::ChunkManager,
}

impl application::Application for Liminal
{
     fn config() -> application::Config
     {
          application::Config::builder()
               .width(1920)
               .height(1080)
               .title("super liminal game experiment")
               .build()
     }

     fn setup(context: &mut render::GfxContext, render: &mut render::GfxRenderer) -> anyhow::Result<Self>
     {
          let atlas = sync::Arc::new(atlas::TextureAtlas::new("./res/liminal/", 128)?);
          atlas.save("./res/liminal_atlas.png")?;
          let seed = env::args()
               .collect::<Vec<String>>()
               .get(1)
               .map(|val| val.parse::<u32>().unwrap_or(0))
               .unwrap_or(0);
          let terrain = terrain::TerrainGenerator::new(seed);
          let mut world = manager::ChunkManager::builder()
               .atlas(sync::Arc::clone(&atlas))
               .terrain(sync::Arc::new(terrain))
               .view_distance(8)
               .chunk_height(32)
               .chunk_width(32)
               .build();
          world.spawn_workers(3);

          let camera = camera::Camera::builder()
               .ar(context.config.width as f32 / context.config.height as f32)
               .fov(90f32)
               .znear(0.1)
               .zfear(100.0)
               .build();
          let player = player::PlayerController::builder()
               .lookspeed(0.0025)
               .movespeed(20.0)
               .kinematics(kinematics::Kinematics::builder().up(glam::Vec3::Y).build())
               .collider(kinematics::BoxCollider::point_sides(
                    camera.inner.position.to_array(),
                    [0.45, 0.85, 0.45],
               ))
               .build();
          let frame = engine::FrameData::new();

          render.register_bind_group_layout(
               context,
               "global_layout",
               &[
                    resource::GfxBindingLayout::Uniform,
                    resource::GfxBindingLayout::Uniform,
                    resource::GfxBindingLayout::Texture,
                    resource::GfxBindingLayout::Sampler,
               ],
          )?;
          render.register_pipeline::<pipelines::Opaque>(context, "terrain_pipe", &["global_layout"]);

          render.register_resource(
               "camera_view_proj_uni",
               util::uniform::<glam::Mat4>(context, "Camera view-projection unioform"),
          );
          render.register_resource(
               "camera_view_uni",
               util::uniform::<glam::Mat4>(context, "Camera view uniform"),
          );
          render.register_resource(
               "texture_atlas",
               util::texture_image(context, &atlas.atlas, "Texture atlas"),
          );
          render.register_resource("texture_sampler", util::sampler(context, "Texture atlas sampler"));

          render.register_bind_group(
               context,
               "global_bg",
               "global_layout",
               &[
                    "camera_view_proj_uni",
                    "camera_view_uni",
                    "texture_atlas",
                    "texture_sampler",
               ],
          )?;

          Ok(Self {
               camera,
               player,
               frame,

               world,
          })
     }

     fn physics_frame(&mut self, input: &mut input::Input, _: &render::GfxContext, _: &render::GfxRenderer)
     {
          self.frame.update();

          self.world.update_chunks(self.camera.inner.position, self.frame.dt);

          if input.consume_key_press("escape")
          {
               input.request_quit = !input.request_quit;
          }
          if input.consume_key_press("keyq")
          {
               input.request_grab = !input.request_grab;
          }
          if input.consume_key_press("keyy")
          {
               self.player.collisions = !self.player.collisions;
          }
          if input.consume_key_press("digit1")
          {
               self.player.movespeed *= 0.5;
          }
          if input.consume_key_press("digit2")
          {
               self.player.movespeed *= 2.0;
          }

          // let mut movement = glam::IVec3::ZERO;
          // if input.get_key_pres("keyw")
          // {
          //      movement.z += 1;
          // }
          // if input.get_key_pres("keys")
          // {
          //      movement.z -= 1;
          // }
          // if input.get_key_pres("keyd")
          // {
          //      movement.x += 1;
          // }
          // if input.get_key_pres("keya")
          // {
          //      movement.x -= 1;
          // }
          // if input.get_key_pres("space")
          // {
          //      movement.y += 1;
          // }
          // if input.get_key_pres("shiftleft")
          // {
          //      movement.y -= 1;
          // }
          // let movement = movement.as_vec3() * self.player.movespeed * self.frame.dt;
          // self.camera.update_position(movement.x, movement.y, movement.z);

          match self.player.collisions
          {
               | true =>
               {
                    let mut frame_movement_speed = self.player.movespeed;
                    let mut camera_offset = 0.65;
                    let [mut dx, _, mut dz] = [0.0; 3];
                    if input.get_key_pres("keyw")
                    {
                         dz += 1.0;
                    }
                    if input.get_key_pres("keys")
                    {
                         dz -= 1.0;
                    }
                    if input.get_key_pres("keyd")
                    {
                         dx += 1.0;
                    }
                    if input.get_key_pres("keya")
                    {
                         dx -= 1.0;
                    }
                    if input.get_key_pres("space")
                    {
                         self.player.kinematics.jump(9.5);
                    }
                    if input.get_key_pres("shiftleft")
                    {
                         frame_movement_speed *= 1.5;
                    }
                    if input.get_key_pres("controlleft")
                    {
                         camera_offset /= 1.5;
                         frame_movement_speed /= 1.5;
                    }
                    let forward = self.camera.inner.forward().with_y(0.0).normalize_or_zero();
                    let right = self.camera.inner.right().with_y(0.0).normalize_or_zero();
                    let movement = (right * dx + forward * dz).normalize_or_zero();
                    self.player.kinematics.velocity.x += movement.x * frame_movement_speed * self.frame.dt;
                    self.player.kinematics.velocity.z += movement.z * frame_movement_speed * self.frame.dt;
                    self.player.kinematics.apply_gravity(32.0, self.frame.dt);
                    self.player.kinematics.apply_drag(24.0, self.frame.dt);
                    self.player.collider =
                         self.player.kinematics.translate(self.player.collider, &self.world, self.frame.dt);
                    self.camera.inner.position =
                         self.player.collider.center() + glam::vec3(0.0, camera_offset, 0.0);
               }
               | false =>
               {
                    let [mut dx, mut dy, mut dz] = [0.0; 3];
                    if input.get_key_pres("keyw")
                    {
                         dz += 1.0;
                    }
                    if input.get_key_pres("keys")
                    {
                         dz -= 1.0;
                    }
                    if input.get_key_pres("keyd")
                    {
                         dx += 1.0;
                    }
                    if input.get_key_pres("keya")
                    {
                         dx -= 1.0;
                    }
                    if input.get_key_pres("space")
                    {
                         dy += 1.0;
                    }
                    if input.get_key_pres("shiftleft")
                    {
                         dy -= 1.0;
                    }
                    [dx, dy, dz] =
                         (glam::vec3(dx, dy, dz).normalize_or_zero() * self.player.movespeed * self.frame.dt)
                              .to_array();
                    self.camera.update_position(dx, dy, dz);
                    self.player.collider =
                         self.player.collider + (self.camera.inner.position - self.player.collider.center());
               }
          }

          let [mut dy, mut dx] = input.consume_mouse_delta().into();
          [dy, dx] = (glam::vec2(dy, dx) * self.player.lookspeed).to_array();
          self.camera.yaw -= dy;
          self.camera.pitch -= dx;
          self.camera.confine_euler();
          self.camera.inner.rotation = glam::Quat::from_rotation_z(0.0)
               * glam::Quat::from_rotation_y(self.camera.yaw)
               * glam::Quat::from_rotation_x(self.camera.pitch);
     }

     fn gfx_frame(
          &mut self,
          _: &input::Input,
          context: &mut render::GfxContext,
          render: &mut render::GfxRenderer,
     )
     {
          self.camera.ar = context.config.width as f32 / context.config.height as f32;
          self.world.sync_gfx_chunks(context, render);
          if let Some(resource::GfxResource::Uniform(camera_uni)) =
               render.resources.get("camera_view_proj_uni")
          {
               camera_uni.write(context, &self.camera.view_proj());
          }
          else
          {
               panic!();
          };
          if let Some(resource::GfxResource::Uniform(camera_view_uni)) =
               render.resources.get("camera_view_uni")
          {
               camera_view_uni.write(context, &self.camera.view());
          }
          else
          {
               panic!();
          }

          self.world.render_chunks.iter().for_each(|&chunk_coord| {
               render.queue(render::GfxDrawCall {
                    mesh: manager::ChunkManager::chunk_key(chunk_coord),
                    pipe: "terrain_pipe".into(),
                    bind_groups: vec!["global_bg".into()],
               });
          });
     }
}
