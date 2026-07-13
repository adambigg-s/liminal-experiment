use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::range;
use std::sync;

use crate::application;
use crate::application::input;
use crate::engine;
use crate::engine::camera;
use crate::engine::kinematics;
use crate::engine::kinematics::Collision;
use crate::engine::player;
use crate::engine::ray;
use crate::engine::ray::Cast;
use crate::engine::transform;
use crate::lifeforms::LifeForm;
use crate::lifeforms::smiler;
use crate::render;
use crate::render::GfxCamera;
use crate::render::resource;
use crate::render::util;
use crate::terrain;
use crate::visual::atlas;
use crate::visual::pipelines;
use crate::world::block;
use crate::world::manager;

#[derive(bon::Builder)]
pub struct Liminal
{
     pub camera: camera::Camera,
     pub player: player::PlayerController,
     pub sounds: player::PlayerSoundController,
     pub head_bobber: player::PlayerHeadBobber,
     pub flashlight: f32,
     pub almond_waters: i32,

     pub audio: kira::AudioManager,

     pub world: manager::ChunkManager,
     pub frame: engine::FrameData,

     pub smilers: smiler::FollowCubeManager,
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
               .view_distance(256)
               .chunk_height(8)
               .chunk_width(32)
               .view_coefficient(glam::usizevec3(1, 4, 1))
               .build();
          world.spawn_workers(2);

          let mut camera = camera::Camera::builder()
               .ar(context.config.width as f32 / context.config.height as f32)
               .fov(90f32)
               .znear(0.1)
               .zfear(500.0)
               .build();
          camera.inner.position += glam::vec3(0.0, 3.0, 0.0);
          camera.update_rotation(0.1, 0.1, 0.1);

          let player = player::PlayerController::builder()
               .lookspeed(0.00125)
               .movespeed(12.0 * 2f32.powf(3.0))
               .kinematics(kinematics::Kinematics::builder().up(glam::Vec3::Y).build())
               .collider(kinematics::BoxCollider::point_sides(
                    camera.inner.position.to_array(),
                    [0.45, 0.85, 0.45],
               ))
               .collisions(true)
               .build();
          let mut sounds = player::PlayerSoundController::new("./res/audio/")?;
          let head_bobber = player::PlayerHeadBobber::new();
          let mut audio = kira::AudioManager::new(kira::AudioManagerSettings::default())?;
          sounds.ambience(&mut audio);
          sounds.listener = Some(audio.add_listener(glam::Vec3::ZERO, glam::Quat::IDENTITY)?);

          let flashlight = 0.0;
          let frame = engine::FrameData::new();
          let almond_waters = 0;

          let smilers = smiler::FollowCubeManager::new(&atlas, context, render);

          render.register_bind_group_layout(
               context,
               "global_layout",
               &[
                    resource::GfxBindingLayout::Uniform,
                    resource::GfxBindingLayout::Uniform,
                    resource::GfxBindingLayout::Texture,
                    resource::GfxBindingLayout::Sampler,
                    resource::GfxBindingLayout::Uniform,
                    resource::GfxBindingLayout::Uniform,
                    resource::GfxBindingLayout::Uniform,
               ],
          )?;
          render.register_bind_group_layout(
               context,
               "dither_layout",
               &[
                    resource::GfxBindingLayout::Texture,
                    resource::GfxBindingLayout::Sampler,
               ],
          )?;
          render.register_bind_group_layout(
               context,
               "entity_layout",
               &[resource::GfxBindingLayout::Uniform],
          )?;

          render.register_pipeline::<pipelines::Opaque>(context, "terrain_pipe", &["global_layout"]);
          render.register_pipeline::<pipelines::Dither>(
               context,
               "dither_pipe",
               &["global_layout", "dither_layout"],
          );
          render.register_pipeline::<pipelines::Entity>(
               context,
               "entity_pipe",
               &["global_layout", "entity_layout"],
          );
          render.register_pipeline::<pipelines::Vignette>(
               context,
               "vignette_pipe",
               &["global_layout", "dither_layout"],
          );

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
               util::texture_image_mipmap(context, &atlas.atlas, "Texture atlas"),
          );
          render.register_resource("texture_sampler", util::sampler_mipmap(context, "Texture atlas sampler"));
          render.register_resource(
               "screen_ar_uni",
               util::uniform::<f32>(context, "Screen aspect ratio uniform"),
          );
          render.register_resource("dither_sampler", util::sampler_mipmap(context, "Dither sampler"));
          render.register_resource("time_uni", util::uniform::<f32>(context, "Global time uniform"));
          render.register_resource(
               "flashlight_uni",
               util::uniform::<f32>(context, "Flashlight toggle uniform"),
          );

          render.register_bind_group(
               context,
               "global_bg",
               "global_layout",
               &[
                    "camera_view_proj_uni",
                    "camera_view_uni",
                    "texture_atlas",
                    "texture_sampler",
                    "screen_ar_uni",
                    "flashlight_uni",
                    "time_uni",
               ],
          )?;

          let readme = fs::File::open("./res/README")?;
          let reader = io::BufReader::new(readme);
          for line in reader.lines()
          {
               log::warn!("{}", line?);
          }

          Ok(Self {
               camera,
               player,
               sounds,
               head_bobber,
               flashlight,
               almond_waters,

               audio,

               world,
               frame,

               smilers,
          })
     }

     fn physics_frame(&mut self, input: &mut input::Input, _: &render::GfxContext, _: &render::GfxRenderer)
     {
          self.frame.update();
          self.world.update_chunks(self.camera.inner.position, self.frame.dt);
          self.smilers.update(&self.player, self.frame.dt);

          if self.almond_waters > 100 || self.player.collider.center().y > 100.0
          {
               log::error!("Nice job! You escaped by finding the 100 almond waters or climbing 100 meters");
               input.request_quit = !input.request_quit;
          }

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
          if input.consume_key_press("bracketleft")
          {
               self.player.movespeed *= 0.5;
          }
          if input.consume_key_press("bracketright")
          {
               self.player.movespeed *= 2.0;
          }
          if input.consume_key_press("minus")
          {
               self.player.lookspeed /= 1.1;
          }
          if input.consume_key_press("equal")
          {
               self.player.lookspeed *= 1.1;
          }
          if input.consume_key_press("keyt")
          {
               if self.flashlight == 0.0
               {
                    self.flashlight = 1.0;
               }
               else
               {
                    self.flashlight = 0.0;
               }
               self.sounds.named_sound(&mut self.audio, "flashlight");
          }
          if input.consume_mouse_left_press()
          {
               let ray = ray::Ray {
                    origin: self.camera.inner.position,
                    direction: self.camera.inner.forward(),
                    tspan: range::Range {
                         start: 0.0,
                         end: 10.0,
                    },
               };
               if let Some(hit) = self.world.cast(ray)
                    && hit.block == block::Block::AlmondWater
               {
                    self.sounds.named_sound(&mut self.audio, "beep");
                    self.world.modify(hit.position, block::Block::empty());
                    self.almond_waters += 1;

                    if rand::random_bool(0.025)
                    {
                         self.sounds.named_sound_directional(
                              &mut self.audio,
                              "follow",
                              self.camera.inner.position - self.camera.inner.forward(),
                         );

                         self.smilers.add_smiler(transform::Transform::from_position(
                              self.camera.inner.position
                                   + glam::vec3(
                                        rand::random_range(-256.0 .. 256.0),
                                        rand::random_range(-8.0 .. 8.0),
                                        rand::random_range(-256.0 .. 256.0),
                                   ),
                         ));
                    }
                    if rand::random_bool(0.005)
                    {
                         self.smilers.add_smiler(transform::Transform::from_position(
                              self.camera.inner.position
                                   + glam::vec3(
                                        rand::random_range(-256.0 .. 256.0),
                                        rand::random_range(-8.0 .. 8.0),
                                        rand::random_range(-256.0 .. 256.0),
                                   ),
                         ));
                    }
               }
          }
          // if input.consume_mouse_right_press()
          // {
          //      self.smilers.add_smiler(transform::Transform::from_position(
          //           self.camera.inner.position
          //                + glam::vec3(
          //                     rand::random_range(-64.0 .. 64.0),
          //                     rand::random_range(-8.0 .. 8.0),
          //                     rand::random_range(-64.0 .. 64.0),
          //                ),
          //      ));
          // }

          let mut unadd = Vec::new();
          for (index, smiler) in self.smilers.cubes.iter().enumerate()
          {
               if (smiler.transform.position - self.camera.inner.position).length() < 2.5
                    && (smiler.transform.position - self.camera.inner.position)
                         .dot(self.camera.inner.forward())
                         > 0.75
               {
                    self.sounds.named_sound_attenuated(&mut self.audio, "puff", 32.0);
                    unadd.push(index);
               }
          }
          for index in unadd
          {
               self.smilers.unadd_smiler(index);
          }

          if let Some(listener) = &mut self.sounds.listener
          {
               listener.set_position(self.camera.inner.position, kira::Tween::default());
               listener.set_orientation(self.camera.inner.rotation, kira::Tween::default());
          }
          else
          {
               log::error!("No listener configured");
          }

          if self.world.collides(self.player.collider)
          {
               self.player.jiggle_free(&self.world);
          }
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
                         frame_movement_speed *= 1.75;
                    }
                    if input.get_key_pres("controlleft")
                    {
                         camera_offset /= 2.0;
                         frame_movement_speed /= 2.0;
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
                    self.camera.inner.position = self.player.collider.center()
                         + glam::vec3(0.0, camera_offset, 0.0)
                         + self.head_bobber.head_bob(&self.camera, &self.player.kinematics, self.frame.time);

                    self.sounds.movement(&mut self.audio, &self.player.kinematics, self.frame.time);
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
          self.smilers.gfx_sync(context, render);

          if let Some(resource::GfxResource::Uniform(camera_uni)) =
               render.resources.get("camera_view_proj_uni")
          {
               camera_uni.write(context, &self.camera.view_proj());
          }
          if let Some(resource::GfxResource::Uniform(camera_view_uni)) =
               render.resources.get("camera_view_uni")
          {
               camera_view_uni.write(context, &self.camera.view());
          }
          if let Some(resource::GfxResource::Uniform(screen_ar)) = render.resources.get("screen_ar_uni")
          {
               screen_ar.write(context, &self.camera.ar);
          }
          if let Some(resource::GfxResource::Uniform(flashlight)) = render.resources.get("flashlight_uni")
          {
               flashlight.write(context, &self.flashlight);
          }
          if let Some(resource::GfxResource::Uniform(time)) = render.resources.get("time_uni")
          {
               time.write(context, &self.frame.time);
          }

          self.world.render_chunks.iter().for_each(|&chunk_coord| {
               render.queue(render::GfxDrawCall {
                    mesh: manager::ChunkManager::chunk_key(chunk_coord),
                    pipe: "terrain_pipe".into(),
                    bind_groups: vec!["global_bg".into()],
               });
          });

          for index in 0 .. self.smilers.cubes.len()
          {
               render.queue(render::GfxDrawCall {
                    mesh: self.smilers.mesh_key().into(),
                    pipe: "entity_pipe".into(),
                    bind_groups: vec!["global_bg".into(), self.smilers.transform_key(index)],
               });
          }
     }

     fn gfx_postpass(
          &mut self,
          _: &input::Input,
          gfx_context: &mut render::GfxContext,
          gfx_render: &mut render::GfxRenderer,
          gfx_encoder: &mut wgpu::CommandEncoder,
          surface_view: &wgpu::TextureView,
     )
     {
          let Some(postpass_a) = &gfx_render.offscreen_texture_a
          else
          {
               log::error!("Attempt to complete graphics postpass without a configured postpass texture");
               return;
          };
          let Some(postpass_b) = &gfx_render.offscreen_texture_b
          else
          {
               log::error!("Attempt to complete graphics postpass without a configured postpass texture");
               return;
          };
          let Some(global_bg) = gfx_render.bind_groups.get("global_bg")
          else
          {
               log::error!("Attempt to grab nonexistant global bindgroup in postpass");
               return;
          };
          let Some(layout) = gfx_render.bind_group_layouts.get("dither_layout")
          else
          {
               log::error!("Attempt to grab nonexistant layout in postpass");
               return;
          };
          let Some(resource::GfxResource::Sampler(sampler)) = gfx_render.resources.get("dither_sampler")
          else
          {
               log::error!("Attempt to grab nonexistant sampler in postpass");
               return;
          };
          let Some(dither_pipe) = gfx_render.pipelines.get("dither_pipe")
          else
          {
               log::error!("Attempt to grab nonexistant pipeline in postpass");
               return;
          };
          let Some(vignette_pipe) = gfx_render.pipelines.get("vignette_pipe")
          else
          {
               log::error!("Attempt to grab nonexistant pipeline in postpass");
               return;
          };

          // Dither pass
          {
               let bind_group = gfx_context.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Dither postpass bind group"),
                    layout,
                    entries: &[
                         wgpu::BindGroupEntry {
                              binding: 0,
                              resource: wgpu::BindingResource::TextureView(&postpass_a.view),
                         },
                         wgpu::BindGroupEntry {
                              binding: 1,
                              resource: wgpu::BindingResource::Sampler(sampler),
                         },
                    ],
               });
               let mut render_pass = gfx_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Postpass render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                         view: &postpass_b.view,
                         depth_slice: None,
                         resolve_target: None,
                         ops: wgpu::Operations {
                              load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                              store: wgpu::StoreOp::Store,
                         },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
               });

               render_pass.set_pipeline(dither_pipe);
               render_pass.set_bind_group(0, global_bg, &[]);
               render_pass.set_bind_group(1, &bind_group, &[]);
               render_pass.draw(0 .. 3, 0 .. 1);
          }

          // Write pass
          {
               let bind_group = gfx_context.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Vignette postpass bind group"),
                    layout,
                    entries: &[
                         wgpu::BindGroupEntry {
                              binding: 0,
                              resource: wgpu::BindingResource::TextureView(&postpass_b.view),
                         },
                         wgpu::BindGroupEntry {
                              binding: 1,
                              resource: wgpu::BindingResource::Sampler(sampler),
                         },
                    ],
               });
               let mut render_pass = gfx_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Postpass render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                         view: surface_view,
                         depth_slice: None,
                         resolve_target: None,
                         ops: wgpu::Operations {
                              load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                              store: wgpu::StoreOp::Store,
                         },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
               });

               render_pass.set_pipeline(vignette_pipe);
               render_pass.set_bind_group(0, global_bg, &[]);
               render_pass.set_bind_group(1, &bind_group, &[]);
               render_pass.draw(0 .. 3, 0 .. 1);
          }
     }
}
