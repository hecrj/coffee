use std::mem;

use wgpu;

use crate::graphics::{DrawParameters, Transformation};

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    transform: wgpu::Buffer,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    instances: wgpu::Buffer,
    constants: wgpu::BindGroup,
    texture_layout: wgpu::BindGroupLayout,
}

impl Pipeline {
    pub fn new(device: &mut wgpu::Device) -> Pipeline {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            r_address_mode: wgpu::AddressMode::ClampToEdge,
            s_address_mode: wgpu::AddressMode::ClampToEdge,
            t_address_mode: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            max_anisotropy: 0,
            compare_function: wgpu::CompareFunction::Always,
            border_color: wgpu::BorderColor::TransparentBlack,
        });

        let constant_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutBinding {
                        binding: 0,
                        visibility: wgpu::ShaderStageFlags::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 1,
                        visibility: wgpu::ShaderStageFlags::FRAGMENT,
                        ty: wgpu::BindingType::Sampler,
                    },
                ],
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
                layout: &constant_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &transform_buffer,
                            range: 0..64,
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

        let texture_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStageFlags::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture,
                }],
            });

        let layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&constant_layout, &texture_layout],
            });

        let vs_module = device
            .create_shader_module(include_bytes!("shader/basic.vert.spv"));
        let fs_module = device
            .create_shader_module(include_bytes!("shader/basic.frag.spv"));

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
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                },
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: wgpu::TextureFormat::Bgra8Unorm,
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
                vertex_buffers: &[
                    wgpu::VertexBufferDescriptor {
                        stride: mem::size_of::<Vertex>() as u32,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &[wgpu::VertexAttributeDescriptor {
                            attribute_index: 0,
                            format: wgpu::VertexFormat::Float2,
                            offset: 0,
                        }],
                    },
                    wgpu::VertexBufferDescriptor {
                        stride: mem::size_of::<Instance>() as u32,
                        step_mode: wgpu::InputStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttributeDescriptor {
                                attribute_index: 1,
                                format: wgpu::VertexFormat::Float4,
                                offset: 0,
                            },
                            wgpu::VertexAttributeDescriptor {
                                attribute_index: 2,
                                format: wgpu::VertexFormat::Float2,
                                offset: 4 * 4,
                            },
                            wgpu::VertexAttributeDescriptor {
                                attribute_index: 3,
                                format: wgpu::VertexFormat::Float2,
                                offset: 4 * (4 + 2),
                            },
                            wgpu::VertexAttributeDescriptor {
                                attribute_index: 4,
                                format: wgpu::VertexFormat::Uint,
                                offset: 4 * (4 + 2 + 2),
                            },
                        ],
                    },
                ],
                sample_count: 1,
            });

        let vertices = device
            .create_buffer_mapped(
                QUAD_VERTS.len(),
                wgpu::BufferUsageFlags::VERTEX,
            )
            .fill_from_slice(&QUAD_VERTS);

        let indices = device
            .create_buffer_mapped(
                QUAD_INDICES.len(),
                wgpu::BufferUsageFlags::INDEX,
            )
            .fill_from_slice(&QUAD_INDICES);

        let instances = device.create_buffer(&wgpu::BufferDescriptor {
            size: mem::size_of::<Instance>() as u32 * Instance::MAX,
            usage: wgpu::BufferUsageFlags::VERTEX
                | wgpu::BufferUsageFlags::TRANSFER_DST,
        });

        Pipeline {
            pipeline,
            transform: transform_buffer,
            vertices,
            indices,
            instances,
            constants: constant_bind_group,
            texture_layout,
        }
    }

    pub fn create_texture_binding(
        &self,
        device: &mut wgpu::Device,
        view: &wgpu::TextureView,
    ) -> TextureBinding {
        let binding = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(view),
            }],
        });

        TextureBinding(binding)
    }

    pub fn draw_texture_quads(
        &mut self,
        device: &mut wgpu::Device,
        texture: &TextureBinding,
        instances: &[Instance],
        transformation: &Transformation,
        target: &wgpu::TextureView,
    ) {
        let matrix: [f32; 16] = transformation.clone().into();

        let transform_buffer = device
            .create_buffer_mapped(16, wgpu::BufferUsageFlags::TRANSFER_SRC)
            .fill_from_slice(&matrix[..]);

        let instance_buffer = device
            .create_buffer_mapped(
                instances.len(),
                wgpu::BufferUsageFlags::TRANSFER_SRC,
            )
            .fill_from_slice(instances);

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                todo: 0,
            });

        encoder.copy_buffer_to_buffer(
            &transform_buffer,
            0,
            &self.transform,
            0,
            16 * 4,
        );

        encoder.copy_buffer_to_buffer(
            &instance_buffer,
            0,
            &self.instances,
            0,
            (mem::size_of::<Instance>() * instances.len()) as u32,
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
            render_pass.set_bind_group(1, &texture.0);
            render_pass.set_index_buffer(&self.indices, 0);
            render_pass.set_vertex_buffers(&[
                (&self.vertices, 0),
                (&self.instances, 0),
            ]);

            render_pass.draw_indexed(
                0..QUAD_INDICES.len() as u32,
                0,
                0..instances.len() as u32,
            );
        }

        device.get_queue().submit(&[encoder.finish()]);
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
}

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

#[derive(Debug, Clone, Copy)]
pub struct Instance {
    pub source: [f32; 4],
    pub scale: [f32; 2],
    pub translation: [f32; 2],
    pub layer: u32,
}

impl Instance {
    const MAX: u32 = 100_000;

    pub fn from_parameters(parameters: DrawParameters) -> Instance {
        let source = parameters.source;
        let position = parameters.position;
        let scale = parameters.scale;

        Instance {
            source: [source.x, source.y, source.width, source.height],
            translation: [position.x, position.y],
            scale: [scale.x, scale.y],
            layer: 0,
        }
    }
}

pub struct TextureBinding(wgpu::BindGroup);
