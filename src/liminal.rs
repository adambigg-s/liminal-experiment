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
               .title("Superulta Liminal Game")
               .build()
     }

     fn setup(context: &mut render::GfxContext, render: &mut render::GfxRenderer) -> anyhow::Result<Self>
     {
          let camera = camera::Camera::builder()
               .ar(context.config.width as f32 / context.config.height as f32)
               .fov(90f32)
               .znear(0.1)
               .zfear(500.0)
               .build();
          let player = player::PlayerController::builder()
               .lookspeed(0.0025)
               .movespeed(2.0)
               .kinematics(kinematics::Kinematics::builder().up(glam::Vec3::Y).build())
               .collider(kinematics::BoxCollider::new([0.0; 3], [0.0; 3]))
               .build();
          let frame = engine::FrameData::new();

          let atlas = sync::Arc::new(atlas::TextureAtlas::new("./res/liminal/", 128)?);
          atlas.save("./res/liminal_atlas.png")?;
          let terrain = terrain::TerrainGenerator::builder().build();
          let mut world = manager::ChunkManager::builder()
               .atlas(sync::Arc::clone(&atlas))
               .terrain(sync::Arc::new(terrain))
               .view_distance(4)
               .chunk_height(32)
               .chunk_width(32)
               .build();
          world.spawn_workers(1);

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

          let mut movement = glam::IVec3::ZERO;
          if input.get_key_pres("keyw")
          {
               movement.z += 1;
          }
          if input.get_key_pres("keys")
          {
               movement.z -= 1;
          }
          if input.get_key_pres("keyd")
          {
               movement.x += 1;
          }
          if input.get_key_pres("keya")
          {
               movement.x -= 1;
          }
          let movement = movement.as_vec3() * self.player.movespeed * self.frame.dt;
          self.camera.update_position(movement.x, movement.y, movement.z);

          let [mut dy, mut dx] = input.consume_mouse_delta().into();
          [dy, dx] = (glam::vec2(dy, dx) * self.player.lookspeed).to_array();
          self.camera.yaw -= dy;
          self.camera.pitch -= dx;
          self.camera.confine_euler();
          self.camera.inner.rotation = glam::Quat::from_rotation_z(0.0)
               * glam::Quat::from_rotation_y(self.camera.yaw)
               * glam::Quat::from_rotation_x(self.camera.pitch);

          log::info!("{}", self.camera); }

     fn gfx_frame(
          &mut self,
          _: &input::Input,
          gfx_context: &mut render::GfxContext,
          gfx_render: &mut render::GfxRenderer,
     )
     {
          if let Some(resource::GfxResource::Uniform(camera_uni)) =
               gfx_render.resources.get("camera_view_proj_uni")
          {
               camera_uni.write(gfx_context, &self.camera.view_proj());
          }
          else
          {
               panic!();
          };
          if let Some(resource::GfxResource::Uniform(camera_view_uni)) =
               gfx_render.resources.get("camera_view_uni")
          {
               camera_view_uni.write(gfx_context, &self.camera.view());
          }
          else
          {
               panic!();
          }

          self.world.sync_gfx_chunks(gfx_context, gfx_render);

          self.world.render_chunks.iter().for_each(|&chunk_coord| {
               gfx_render.queue(render::GfxDrawCall {
                    mesh: manager::ChunkManager::chunk_key(chunk_coord),
                    pipe: "terrain_pipe".into(),
                    bind_groups: vec!["global_bg".into()],
               });
          });

          log::info!("number of draw calls: {}", self.world.render_chunks.len());
     }
}
