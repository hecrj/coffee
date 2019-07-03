use std::mem;

use crate::graphics::Transformation;

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
                bindings: &[wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStageFlags::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer,
                }],
            });

        let matrix: [f32; 16] = Transformation::identity().into();

        let transform_buffer = device
            .create_buffer_mapped(
                16,
                wgpu::BufferUsageFlags::UNIFORM
                    | wgpu::BufferUsageFlags::TRANSFER_DST,
            )
            .fill_from_slice(&matrix[..]);

        let constant_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
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

        let vs_module = device
            .create_shader_module(include_bytes!("shader/triangle.vert.spv"));
        let fs_module = device
            .create_shader_module(include_bytes!("shader/triangle.frag.spv"));

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &layout,
                vertex_stage: wgpu::PipelineStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: wgpu::PipelineStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                },
                rasterization_state: wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::None,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                },
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    color: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWriteFlags::ALL,
                }],
                depth_stencil_state: None,
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: mem::size_of::<Vertex>() as u32,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttributeDescriptor {
                            attribute_index: 0,
                            format: wgpu::VertexFormat::Float2,
                            offset: 0,
                        },
                        wgpu::VertexAttributeDescriptor {
                            attribute_index: 1,
                            format: wgpu::VertexFormat::Float4,
                            offset: 4 * 2,
                        },
                    ],
                }],
                sample_count: 1,
            });

        let vertices = device.create_buffer(&wgpu::BufferDescriptor {
            size: mem::size_of::<Vertex>() as u32
                * Self::INITIAL_BUFFER_SIZE as u32,
            usage: wgpu::BufferUsageFlags::VERTEX,
        });

        let indices = device.create_buffer(&wgpu::BufferDescriptor {
            size: Self::INITIAL_BUFFER_SIZE * 2,
            usage: wgpu::BufferUsageFlags::INDEX,
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
        indices: &[u16],
        transformation: &Transformation,
        target: &wgpu::TextureView,
    ) {
        if vertices.is_empty() || indices.is_empty() {
            return;
        }

        let matrix: [f32; 16] = transformation.clone().into();

        let transform_buffer = device
            .create_buffer_mapped(16, wgpu::BufferUsageFlags::TRANSFER_SRC)
            .fill_from_slice(&matrix[..]);

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
                size: mem::size_of::<Vertex>() as u32 * new_size,
                usage: wgpu::BufferUsageFlags::VERTEX,
            });

            self.indices = device.create_buffer(&wgpu::BufferDescriptor {
                size: new_size * 2,
                usage: wgpu::BufferUsageFlags::INDEX,
            });

            self.buffer_size = new_size;
        }

        let vertex_buffer = device
            .create_buffer_mapped(
                vertices.len(),
                wgpu::BufferUsageFlags::TRANSFER_SRC,
            )
            .fill_from_slice(vertices);

        let index_buffer = device
            .create_buffer_mapped(
                indices.len(),
                wgpu::BufferUsageFlags::TRANSFER_SRC,
            )
            .fill_from_slice(indices);

        encoder.copy_buffer_to_buffer(
            &vertex_buffer,
            0,
            &self.vertices,
            0,
            (mem::size_of::<Vertex>() * vertices.len()) as u32,
        );

        encoder.copy_buffer_to_buffer(
            &index_buffer,
            0,
            &self.indices,
            0,
            (mem::size_of::<u16>() * indices.len()) as u32,
        );

        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: target,
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
            render_pass.set_bind_group(0, &self.constants);
            render_pass.set_index_buffer(&self.indices, 0);
            render_pass.set_vertex_buffers(&[(&self.vertices, 0)]);

            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
