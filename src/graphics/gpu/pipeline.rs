use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_device_gl as gl;

use super::format;
use super::texture::Texture;
use crate::graphics::draw_parameters::DrawParameters;
use crate::graphics::transformation::Transformation;

const MAX_POINTS: u32 = 100_000;
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

    vertex Instance {
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
        instances: gfx::InstanceBuffer<Instance> = (),
        out: gfx::RawRenderTarget =
          (
              "Target0",
               format::COLOR, gfx::state::ColorMask::all(),
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
                MAX_POINTS as usize,
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                gfx::memory::Bind::SHADER_RESOURCE,
            )
            .unwrap();

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

        encoder.update_buffer(&data.globals, &[globals], 0).unwrap();

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

    pub fn draw_quads(
        &mut self,
        encoder: &mut gfx::Encoder<gl::Resources, gl::CommandBuffer>,
        instances: &[Instance],
        transformation: &Transformation,
        view: &gfx::handle::RawRenderTargetView<gl::Resources>,
    ) {
        let transformation_matrix: [[f32; 4]; 4] =
            transformation.clone().into();

        if self.globals.mvp != transformation_matrix {
            self.globals.mvp = transformation_matrix;

            encoder
                .update_buffer(&self.data.globals, &[self.globals], 0)
                .unwrap();
        }

        self.data.out = view.clone();

        encoder
            .update_buffer(&self.data.instances, instances, 0)
            .unwrap();

        self.slice.instances = Some((instances.len() as u32, 0));

        encoder.draw(&self.slice, &self.shader.state, &self.data)
    }
}

pub struct Shader {
    state: gfx::pso::PipelineState<gl::Resources, pipe::Meta>,
}

impl Shader {
    pub fn new(factory: &mut gl::Factory, init: pipe::Init) -> Shader {
        let set = factory
            .create_shader_set(
                include_bytes!("shader/basic.vert"),
                include_bytes!("shader/basic.frag"),
            )
            .unwrap();

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
            .unwrap();

        Shader { state }
    }
}

impl Instance {
    pub fn from_parameters(parameters: DrawParameters) -> Instance {
        let source = parameters.source;
        let position = parameters.position;
        let scale = parameters.scale;

        Instance {
            src: [source.x, source.y, source.width, source.height],
            translation: [position.x, position.y],
            scale: [scale.x, scale.y],
            layer: 0,
        }
    }
}
