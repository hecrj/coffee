use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_device_gl as gl;

use super::format;
use super::texture::Texture;
use crate::graphics::{self, Transformation};

const MAX_INSTANCES: u32 = 100_000;
const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const QUAD_VERTS: [Vertex; 4] = [
    Vertex {
        position: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0],
    },
];

gfx_defines! {
    vertex Vertex {
        position: [f32; 2] = "a_Pos",
    }

    vertex Quad {
        src: [f32; 4] = "a_Src",
        translation: [f32; 2] = "a_Translation",
        scale: [f32; 2] = "a_Scale",
        layer: u32 = "t_Layer",
    }

    constant Globals {
        mvp: [[f32; 4]; 4] = "u_MVP",
    }

    pipeline pipe {
        vertices: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        globals: gfx::ConstantBuffer<Globals> = "Globals",
        instances: gfx::InstanceBuffer<Quad> = (),
        out: gfx::RawRenderTarget =
          (
              "Target0",
               format::COLOR,
               gfx::state::ColorMask::all(),
               Some(gfx::preset::blend::ALPHA)
          ),
    }
}

pub struct Pipeline {
    slice: gfx::Slice<gl::Resources>,
    data: pipe::Data<gl::Resources>,
    shader: Shader,
    globals: Globals,
}

impl Pipeline {
    pub fn new(
        factory: &mut gl::Factory,
        encoder: &mut gfx::Encoder<gl::Resources, gl::CommandBuffer>,
        target: &gfx::handle::RawRenderTargetView<gl::Resources>,
    ) -> Pipeline {
        // Create point buffer
        let instances = factory
            .create_buffer(
                MAX_INSTANCES as usize,
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                gfx::memory::Bind::SHADER_RESOURCE,
            )
            .expect("Instance buffer creation");

        let (quads, slice) = factory
            .create_vertex_buffer_with_slice(&QUAD_VERTS, &QUAD_INDICES[..]);

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
            globals: factory.create_constant_buffer(1),
            instances,
            out: target.clone(),
        };

        let init = pipe::Init {
            out: (
                "Target0",
                format::COLOR,
                gfx::state::ColorMask::all(),
                Some(gfx::preset::blend::ALPHA),
            ),
            ..pipe::new()
        };

        let shader = Shader::new(factory, init);

        let globals = Globals {
            mvp: Transformation::identity().into(),
        };

        encoder
            .update_buffer(&data.globals, &[globals], 0)
            .expect("Globals initialization");

        Pipeline {
            slice,
            data,
            shader,
            globals,
        }
    }

    pub fn bind_texture(&mut self, texture: &Texture) {
        self.data.texture.0 = texture.view().clone();
    }

    pub fn draw_textured(
        &mut self,
        encoder: &mut gfx::Encoder<gl::Resources, gl::CommandBuffer>,
        instances: &[Quad],
        transformation: &Transformation,
        view: &gfx::handle::RawRenderTargetView<gl::Resources>,
    ) {
        let transformation_matrix: [[f32; 4]; 4] =
            transformation.clone().into();

        if self.globals.mvp != transformation_matrix {
            self.globals.mvp = transformation_matrix;

            encoder
                .update_buffer(&self.data.globals, &[self.globals], 0)
                .expect("Globals upload");
        }

        self.data.out = view.clone();

        let mut i = 0;
        let total = instances.len();

        while i < total {
            let end = (i + MAX_INSTANCES as usize).min(total);

            encoder
                .update_buffer(&self.data.instances, &instances[i..end], 0)
                .expect("Instance upload");

            self.slice.instances = Some((end as u32 - i as u32, 0));

            encoder.draw(&self.slice, &self.shader.state, &self.data);

            i += MAX_INSTANCES as usize;
        }
    }
}

pub struct Shader {
    state: gfx::pso::PipelineState<gl::Resources, pipe::Meta>,
}

impl Shader {
    pub fn new(factory: &mut gl::Factory, init: pipe::Init<'_>) -> Shader {
        let set = factory
            .create_shader_set(
                include_bytes!("shader/quad.vert"),
                include_bytes!("shader/quad.frag"),
            )
            .expect("Shader set creation");

        let rasterizer = gfx::state::Rasterizer {
            front_face: gfx::state::FrontFace::CounterClockwise,
            cull_face: gfx::state::CullFace::Nothing,
            method: gfx::state::RasterMethod::Fill,
            offset: None,
            samples: None,
        };

        let state = factory
            .create_pipeline_state(
                &set,
                Primitive::TriangleList,
                rasterizer,
                init,
            )
            .expect("Pipeline state creation");

        Shader { state }
    }
}

impl From<graphics::Quad> for Quad {
    fn from(quad: graphics::Quad) -> Quad {
        let source = quad.source;
        let position = quad.position;
        let (width, height) = quad.size;

        Quad {
            src: [source.x, source.y, source.width, source.height],
            translation: [position.x, position.y],
            scale: [width, height],
            layer: 0,
        }
    }
}
