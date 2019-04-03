use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_device_gl as gl;
use image;

use crate::graphics::gpu;
use crate::graphics::gpu::texture::Texture;

pub struct Pipeline {
    quads: gfx::handle::Buffer<gl::Resources, Vertex>,
    quad_slice: gfx::Slice<gl::Resources>,
    data: pipe::Data<gl::Resources>,
}

impl Pipeline {
    pub fn new(
        factory: &mut gl::Factory,
        target: &gfx::handle::RawRenderTargetView<gl::Resources>,
    ) -> Pipeline {
        let instances = factory
            .create_buffer(
                1,
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                gfx::memory::Bind::SHADER_RESOURCE,
            )
            .unwrap();

        let (quads, mut quad_slice) = factory
            .create_vertex_buffer_with_slice(&QUAD_VERTS, &QUAD_INDICES[..]);

        quad_slice.instances = Some((1, 0));

        let sampler = factory.create_sampler(gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Scale,
            gfx::texture::WrapMode::Clamp,
        ));

        let texture = Texture::new(
            factory,
            &image::DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(
                1,
                1,
                image::Rgba([255, 255, 255, 255]),
            )),
        );

        let data = pipe::Data {
            vertices: quads.clone(),
            texture: (texture.view().clone(), sampler),
            instances,
            globals: factory.create_constant_buffer(1),
            out: target.clone(),
        };

        Pipeline {
            quads,
            quad_slice,
            data,
        }
    }
}

gfx_defines! {
    vertex Vertex {
        position: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    vertex Instance {
        src: [f32; 4] = "a_Src",
        col1: [f32; 4] = "a_TCol1",
        col2: [f32; 4] = "a_TCol2",
        col3: [f32; 4] = "a_TCol3",
        col4: [f32; 4] = "a_TCol4",
        color: [f32; 4] = "a_Color",
        layer: u32 = "t_Layer",
    }

    constant Globals {
        mvp_matrix: [[f32; 4]; 4] = "u_MVP",
    }

    pipeline pipe {
        vertices: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        globals: gfx::ConstantBuffer<Globals> = "Globals",
        instances: gfx::InstanceBuffer<Instance> = (),
        out: gfx::RawRenderTarget =
          (
              "Target0",
               gpu::COLOR_FORMAT, gfx::state::ColorMask::all(),
               Some(gfx::preset::blend::ALPHA)
          ),
    }
}

const QUAD_VERTS: [Vertex; 4] = [
    Vertex {
        position: [0.0, 0.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0],
        uv: [0.0, 1.0],
    },
];

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];
