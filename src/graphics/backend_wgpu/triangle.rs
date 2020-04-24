use std::mem;

use crate::graphics::Transformation;
use zerocopy::AsBytes;

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    transform: wgpu::Buffer,
    constants: wgpu::BindGroup,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    buffer_size: u32,
}

impl Pipeline {
    const INITIAL_BUFFER_SIZE: u32 = 100_000;

    pub fn new(device: &mut wgpu::Device) -> Pipeline {
        let transform_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("coffee::backend::triangle transform"),
                bindings: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }],
            });

        let matrix: [f32; 16] = Transformation::identity().into();

        let transform_buffer = device.create_buffer_with_data(
            matrix.as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let constant_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("coffee::backend::triangle constants"),
                layout: &transform_layout,
                bindings: &[wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &transform_buffer,
                        range: 0..64,
                    },
                }],
            });

        let layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&transform_layout],
            });

        let vs = include_bytes!("shader/triangle.vert.spv");
        let vs_module = device.create_shader_module(
            &wgpu::read_spirv(std::io::Cursor::new(&vs[..]))
                .expect("Read triangle vertex shader as SPIR-V"),
        );

        let fs = include_bytes!("shader/triangle.frag.spv");
        let fs_module = device.create_shader_module(
            &wgpu::read_spirv(std::io::Cursor::new(&fs[..]))
                .expect("Read triangle fragment shader as SPIR-V"),
        );

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::None,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                }),
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    color_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint32,
                    vertex_buffers: &[wgpu::VertexBufferDescriptor {
                        stride: mem::size_of::<Vertex>() as u64,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttributeDescriptor {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float2,
                                offset: 0,
                            },
                            wgpu::VertexAttributeDescriptor {
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float4,
                                offset: 4 * 2,
                            },
                        ],
                    }],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        let vertices = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("coffee::backend::triangle vertices"),
            size: mem::size_of::<Vertex>() as u64
                * Self::INITIAL_BUFFER_SIZE as u64,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });

        let indices = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("coffee::backend::triangle indices"),
            size: mem::size_of::<u32>() as u64
                * Self::INITIAL_BUFFER_SIZE as u64,
            usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST,
        });

        Pipeline {
            pipeline,
            transform: transform_buffer,
            constants: constant_bind_group,
            vertices,
            indices,
            buffer_size: Self::INITIAL_BUFFER_SIZE,
        }
    }

    pub fn draw(
        &mut self,
        device: &mut wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        vertices: &[Vertex],
        indices: &[u32],
        transformation: &Transformation,
        target: &wgpu::TextureView,
    ) {
        if vertices.is_empty() || indices.is_empty() {
            return;
        }

        let matrix: [f32; 16] = transformation.clone().into();

        let transform_buffer = device.create_buffer_with_data(
            matrix.as_bytes(),
            wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(
            &transform_buffer,
            0,
            &self.transform,
            0,
            16 * 4,
        );

        if self.buffer_size < vertices.len() as u32
            || self.buffer_size < indices.len() as u32
        {
            let new_size = vertices.len().max(indices.len()) as u32;

            self.vertices = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("coffee::backend::triangle vertices"),
                size: mem::size_of::<Vertex>() as u64 * new_size as u64,
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            });

            self.indices = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("coffee::backend::triangle indices"),
                size: mem::size_of::<u32>() as u64 * new_size as u64,
                usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST,
            });

            self.buffer_size = new_size;
        }

        let vertex_buffer = device.create_buffer_with_data(
            vertices.as_bytes(),
            wgpu::BufferUsage::COPY_SRC,
        );

        let index_buffer = device.create_buffer_with_data(
            indices.as_bytes(),
            wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(
            &vertex_buffer,
            0,
            &self.vertices,
            0,
            (mem::size_of::<Vertex>() * vertices.len()) as u64,
        );

        encoder.copy_buffer_to_buffer(
            &index_buffer,
            0,
            &self.indices,
            0,
            (mem::size_of::<u32>() * indices.len()) as u64,
        );

        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: target,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Load,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.0,
                            },
                        },
                    ],
                    depth_stencil_attachment: None,
                });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.constants, &[]);
            render_pass.set_index_buffer(&self.indices, 0, 0);
            render_pass.set_vertex_buffer(0, &self.vertices, 0, 0);

            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        }
    }
}

#[derive(Debug, Clone, Copy, AsBytes)]
#[repr(C)]
pub struct Vertex {
    _position: [f32; 2],
    _color: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 4]) -> Vertex {
        Vertex {
            _position: position,
            _color: color,
        }
    }
}
