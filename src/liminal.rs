use std::mem;

use crate::application;
use crate::render;
use crate::render::resource;
use crate::render::util;
use crate::visual::pipelines;

pub struct Liminal {}

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
                              pos: [-0.5f32, -0.5, 0.0],
                              col: [1.0, 0.0, 0.0],
                         },
                         TriVertex {
                              pos: [0.5, -0.5, 0.0],
                              col: [0.0, 1.0, 0.0],
                         },
                         TriVertex {
                              pos: [0.0, 0.5, 0.0],
                              col: [0.0, 0.0, 1.0],
                         },
                    ],
                    &[0u32, 1, 2],
               ),
          );

          Ok(Self {})
     }

     fn physics_frame(
          &mut self,
          input: &mut application::input::Input,
          gfx_context: &crate::render::GfxContext,
          gfx_render: &crate::render::GfxRenderer,
     )
     {
          if input.get_key_pres("escape")
          {
               input.request_quit = !input.request_quit;
          }
     }

     fn gfx_frame(
          &mut self,
          input: &application::input::Input,
          gfx_context: &mut crate::render::GfxContext,
          gfx_render: &mut crate::render::GfxRenderer,
     )
     {
          gfx_render.queue(render::GfxDrawCall {
               mesh: "testing_mesh".into(),
               pipe: "terrain_pipe".into(),
               bind_groups: vec!["global_bg".into()],
          });
     }
}
