use std::mem;

use crate::application;
use crate::engine;
use crate::engine::camera;
use crate::engine::kinematics;
use crate::engine::player;
use crate::render;
use crate::render::GfxCamera;
use crate::render::resource;
use crate::render::util;
use crate::visual::pipelines;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, bon::Builder, Debug, Default, Clone, Copy)]
pub struct TriVertex
{
     pub pos: [f32; 3],
     pub col: [f32; 3],
}

impl render::GfxVertex for TriVertex
{
     fn descriptor() -> wgpu::VertexBufferLayout<'static>
     {
          const ATTRIBS: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
               0 => Float32x3,
               1 => Float32x3,
          ];

          wgpu::VertexBufferLayout {
               array_stride: mem::size_of::<Self>() as u64,
               step_mode: wgpu::VertexStepMode::Vertex,
               attributes: ATTRIBS,
          }
     }
}

#[derive(bon::Builder, Debug)]
pub struct Liminal
{
     pub camera: camera::Camera,
     pub player: player::PlayerController,
     pub frame: engine::FrameData,
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

     fn setup(
          gfx_context: &mut crate::render::GfxContext,
          gfx_render: &mut crate::render::GfxRenderer,
     ) -> anyhow::Result<Self>
     {
          gfx_render
               .register_resource("camera_uni", util::uniform::<glam::Mat4>(gfx_context, "Camera uniform"));
          gfx_render.register_bind_group_layout(
               gfx_context,
               "global_layout",
               &[resource::GfxBindingLayout::Uniform],
          )?;
          gfx_render.register_bind_group(gfx_context, "global_bg", "global_layout", &["camera_uni"])?;
          gfx_render.register_pipeline::<pipelines::Opaque>(gfx_context, "terrain_pipe", &["global_layout"]);
          gfx_render.register_mesh(
               "testing_mesh",
               util::mesh(
                    gfx_context,
                    &[
                         TriVertex {
                              pos: [-0.5f32, -0.5, 1.0],
                              col: [1.0, 0.0, 0.0],
                         },
                         TriVertex {
                              pos: [0.5, -0.5, 1.0],
                              col: [0.0, 1.0, 0.0],
                         },
                         TriVertex {
                              pos: [0.0, 0.5, 1.0],
                              col: [0.0, 0.0, 1.0],
                         },
                    ],
                    &[0u32, 1, 2],
               ),
          );

          let camera = camera::Camera::builder()
               .ar(gfx_context.config.width as f32 / gfx_context.config.height as f32)
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

          Ok(Self {
               camera,
               player,
               frame,
          })
     }

     fn physics_frame(
          &mut self,
          input: &mut application::input::Input,
          gfx_context: &crate::render::GfxContext,
          gfx_render: &crate::render::GfxRenderer,
     )
     {
          self.frame.update();

          if input.get_key_pres("escape")
          {
               input.request_quit = !input.request_quit;
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
     }

     fn gfx_frame(
          &mut self,
          input: &application::input::Input,
          gfx_context: &mut crate::render::GfxContext,
          gfx_render: &mut crate::render::GfxRenderer,
     )
     {
          if let Some(resource::GfxResource::Uniform(camera_uni)) = gfx_render.resources.get("camera_uni")
          {
               camera_uni.write(gfx_context, &self.camera.view_proj());
          }

          gfx_render.queue(render::GfxDrawCall {
               mesh: "testing_mesh".into(),
               pipe: "terrain_pipe".into(),
               bind_groups: vec!["global_bg".into()],
          });
     }
}
