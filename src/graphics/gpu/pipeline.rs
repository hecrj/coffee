use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_device_gl as gl;
use image;

use crate::graphics::color::Color;
use crate::graphics::draw_parameters::DrawParameters;
use crate::graphics::gpu;
use crate::graphics::gpu::texture::Texture;
use crate::graphics::transformation::Transformation;

gfx_defines! {
    vertex Vertex {
        position: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    vertex Instance {
        src: [f32; 4] = "a_Src",
        col1: [f32; 3] = "a_TCol1",
        col2: [f32; 3] = "a_TCol2",
        col3: [f32; 3] = "a_TCol3",
        color: [f32; 4] = "a_Color",
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
               gpu::COLOR_FORMAT, gfx::state::ColorMask::all(),
               Some(gfx::preset::blend::ALPHA)
          ),
    }
}

pub struct Pipeline {
    encoder: gfx::Encoder<gl::Resources, gl::CommandBuffer>,
    quads: gfx::handle::Buffer<gl::Resources, Vertex>,
    quad_slice: gfx::Slice<gl::Resources>,
    data: pipe::Data<gl::Resources>,
    shader: Shader,
    globals: Globals,
}

impl Pipeline {
    pub fn new(
        factory: &mut gl::Factory,
        target: &gfx::handle::RawRenderTargetView<gl::Resources>,
        color_format: gfx::format::Format,
    ) -> Pipeline {
        let mut encoder: gfx::Encoder<gl::Resources, gl::CommandBuffer> =
            factory.create_command_buffer().into();

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

        let init = pipe::Init {
            out: (
                "Target0",
                color_format,
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
            encoder,
            quads,
            quad_slice,
            data,
            shader,
            globals,
        }
    }

    pub fn clear(
        &mut self,
        view: &gfx::handle::RawRenderTargetView<gl::Resources>,
        color: Color,
    ) {
        let typed_render_target: gfx::handle::RenderTargetView<
            gl::Resources,
            gfx::format::Srgba8,
        > = gfx::memory::Typed::new(view.clone());

        self.encoder.clear(&typed_render_target, color.into())
    }

    pub fn flush(&mut self, device: &mut gl::Device) {
        self.encoder.flush(device);
    }

    pub fn bind_texture(&mut self, texture: &Texture) {
        self.data.texture.0 = texture.view().clone();
    }

    pub fn draw_quad(
        &mut self,
        instance: Instance,
        transformation: &Transformation,
        view: &gfx::handle::RawRenderTargetView<gl::Resources>,
    ) {
        let transformation_matrix: [[f32; 4]; 4] =
            transformation.clone().into();

        if self.globals.mvp != transformation_matrix {
            self.globals.mvp = transformation_matrix;

            self.encoder
                .update_buffer(&self.data.globals, &[self.globals], 0)
                .unwrap();
        }

        self.data.out = view.clone();

        self.encoder
            .update_buffer(&self.data.instances, &[instance], 0)
            .unwrap();

        self.encoder
            .draw(&self.quad_slice, &self.shader.state, &self.data)
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

impl Instance {
    pub fn from_parameters(parameters: DrawParameters) -> Instance {
        let scale = nalgebra::Vector2::new(
            parameters.scale.x * parameters.source.width,
            parameters.scale.y * parameters.source.height,
        );

        let source = parameters.source;
        let position = parameters.position;

        Instance {
            src: [source.x, source.y, source.width, source.height],
            col1: [scale.x, 0.0, 0.0],
            col2: [0.0, scale.y, 0.0],
            col3: [position.x, position.y, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            layer: 0,
        }
    }
}
