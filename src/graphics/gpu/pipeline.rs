use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_device_gl as gl;
use image;

use crate::graphics::color::Color;
use crate::graphics::draw_parameters::DrawParameters;
use crate::graphics::gpu;
use crate::graphics::gpu::texture::Texture;
use crate::graphics::transformation::Transformation;

const MAX_POINTS: u32 = 50_000;
const QUAD_INDICES: [u8; 6] = [0, 1, 2, 0, 2, 3];

gfx_defines! {
    vertex Point {
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
        points: gfx::VertexBuffer<Point> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        globals: gfx::ConstantBuffer<Globals> = "Globals",
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
    slice: gfx::Slice<gl::Resources>,
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

        // Create point buffer
        let points = factory
            .create_buffer(
                MAX_POINTS as usize,
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                gfx::memory::Bind::SHADER_RESOURCE,
            )
            .unwrap();

        // Generate indices
        let mut indices: Vec<u32> = Vec::new();

        for i in 0..MAX_POINTS {
            for q in &QUAD_INDICES {
                indices.push(i * 4 + *q as u32);
            }
        }

        let index_buffer = factory.create_index_buffer(&indices[..]);

        let slice = gfx::Slice {
            start: 0,
            end: QUAD_INDICES.len() as u32,
            base_vertex: 0,
            instances: None,
            buffer: index_buffer,
        };

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
            texture: (texture.view().clone(), sampler),
            points,
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
            slice,
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
        point: Point,
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
            .update_buffer(&self.data.points, &[point, point, point, point], 0)
            .unwrap();

        self.slice.end = QUAD_INDICES.len() as u32;

        self.encoder
            .draw(&self.slice, &self.shader.state, &self.data)
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

impl Point {
    pub fn from_parameters(parameters: DrawParameters) -> Point {
        let scale = nalgebra::Vector2::new(
            parameters.scale.x * parameters.source.width,
            parameters.scale.y * parameters.source.height,
        );

        let source = parameters.source;
        let position = parameters.position;

        Point {
            src: [source.x, source.y, source.width, source.height],
            col1: [scale.x, 0.0, 0.0],
            col2: [0.0, scale.y, 0.0],
            col3: [position.x, position.y, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            layer: 0,
        }
    }
}
