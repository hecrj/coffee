use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_device_gl as gl;

use super::format;
use crate::graphics::Transformation;

gfx_defines! {
    vertex Vertex {
        position: [f32; 2] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    constant Globals {
        mvp: [[f32; 4]; 4] = "u_MVP",
    }

    pipeline pipe {
        vertices: gfx::VertexBuffer<Vertex> = (),
        globals: gfx::ConstantBuffer<Globals> = "Globals",
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
    data: pipe::Data<gl::Resources>,
    indices: gfx::handle::Buffer<gl::Resources, u32>,
    shader: Shader,
    globals: Globals,
}

impl Pipeline {
    const INITIAL_BUFFER_SIZE: usize = 100_000;

    pub fn new(
        factory: &mut gl::Factory,
        encoder: &mut gfx::Encoder<gl::Resources, gl::CommandBuffer>,
        target: &gfx::handle::RawRenderTargetView<gl::Resources>,
    ) -> Pipeline {
        let vertices = factory
            .create_buffer(
                Self::INITIAL_BUFFER_SIZE,
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                gfx::memory::Bind::SHADER_RESOURCE,
            )
            .expect("Vertex buffer creation");

        let indices = factory
            .create_buffer(
                Self::INITIAL_BUFFER_SIZE,
                gfx::buffer::Role::Index,
                gfx::memory::Usage::Dynamic,
                gfx::memory::Bind::empty(),
            )
            .expect("Index buffer creation");

        let data = pipe::Data {
            vertices,
            globals: factory.create_constant_buffer(1),
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
            data,
            indices,
            shader,
            globals,
        }
    }

    pub fn draw(
        &mut self,
        factory: &mut gl::Factory,
        encoder: &mut gfx::Encoder<gl::Resources, gl::CommandBuffer>,
        vertices: &[Vertex],
        indices: &[u32],
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

        if self.data.vertices.len() < vertices.len()
            || self.indices.len() < indices.len()
        {
            let vertices = factory
                .create_buffer(
                    self.data.vertices.len(),
                    gfx::buffer::Role::Vertex,
                    gfx::memory::Usage::Dynamic,
                    gfx::memory::Bind::SHADER_RESOURCE,
                )
                .expect("Vertex buffer creation");

            let indices = factory
                .create_buffer(
                    indices.len(),
                    gfx::buffer::Role::Index,
                    gfx::memory::Usage::Dynamic,
                    gfx::memory::Bind::empty(),
                )
                .expect("Index buffer creation");

            self.data.vertices = vertices;
            self.indices = indices;
        }

        encoder
            .update_buffer(&self.data.vertices, &vertices, 0)
            .expect("Vertex upload");

        encoder
            .update_buffer(&self.indices, &indices, 0)
            .expect("Index upload");

        let slice = gfx::Slice {
            start: 0,
            end: indices.len() as u32,
            base_vertex: 0,
            instances: None,
            buffer: gfx::IndexBuffer::Index32(self.indices.clone()),
        };

        encoder.draw(&slice, &self.shader.state, &self.data);
    }
}

pub struct Shader {
    state: gfx::pso::PipelineState<gl::Resources, pipe::Meta>,
}

impl Shader {
    pub fn new(factory: &mut gl::Factory, init: pipe::Init<'_>) -> Shader {
        let set = factory
            .create_shader_set(
                include_bytes!("shader/triangle.vert"),
                include_bytes!("shader/triangle.frag"),
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

impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 4]) -> Vertex {
        Vertex { position, color }
    }
}
