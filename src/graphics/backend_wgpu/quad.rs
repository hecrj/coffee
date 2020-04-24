use std::mem;

use crate::graphics::{self, Transformation};
use zerocopy::AsBytes;

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
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        let constant_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("coffee::backend::quad constants"),
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false },
                    },
                ],
            });

        let matrix: [f32; 16] = Transformation::identity().into();

        let transform_buffer = device.create_buffer_with_data(
            matrix.as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let constant_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("coffee::backend::quad constants"),
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
                label: Some("coffee::backend::quad texture"),
                bindings: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2Array,
                        component_type: wgpu::TextureComponentType::Float,
                    },
                }],
            });

        let layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&constant_layout, &texture_layout],
            });

        let vs = include_bytes!("shader/quad.vert.spv");
        let vs_module = device.create_shader_module(
            &wgpu::read_spirv(std::io::Cursor::new(&vs[..]))
                .expect("Read quad vertex shader as SPIR-V"),
        );

        let fs = include_bytes!("shader/quad.frag.spv");
        let fs_module = device.create_shader_module(
            &wgpu::read_spirv(std::io::Cursor::new(&fs[..]))
                .expect("Read quad fragment shader as SPIR-V"),
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
                    front_face: wgpu::FrontFace::Cw,
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
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[
                        wgpu::VertexBufferDescriptor {
                            stride: mem::size_of::<Vertex>() as u64,
                            step_mode: wgpu::InputStepMode::Vertex,
                            attributes: &[wgpu::VertexAttributeDescriptor {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float2,
                                offset: 0,
                            }],
                        },
                        wgpu::VertexBufferDescriptor {
                            stride: mem::size_of::<Quad>() as u64,
                            step_mode: wgpu::InputStepMode::Instance,
                            attributes: &[
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float4,
                                    offset: 0,
                                },
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 2,
                                    format: wgpu::VertexFormat::Float2,
                                    offset: 4 * 4,
                                },
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 3,
                                    format: wgpu::VertexFormat::Float2,
                                    offset: 4 * (4 + 2),
                                },
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 4,
                                    format: wgpu::VertexFormat::Uint,
                                    offset: 4 * (4 + 2 + 2),
                                },
                            ],
                        },
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        let vertices = device.create_buffer_with_data(
            QUAD_VERTS.as_bytes(),
            wgpu::BufferUsage::VERTEX,
        );

        let indices = device.create_buffer_with_data(
            QUAD_INDICES.as_bytes(),
            wgpu::BufferUsage::INDEX,
        );

        let instances = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("coffee::backend::quad instances"),
            size: mem::size_of::<Quad>() as u64 * Quad::MAX as u64,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
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
            label: Some("coffee::backend::quad texture"),
            layout: &self.texture_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(view),
            }],
        });

        TextureBinding(binding)
    }

    pub fn draw_textured(
        &mut self,
        device: &mut wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        texture: &TextureBinding,
        instances: &[Quad],
        transformation: &Transformation,
        target: &wgpu::TextureView,
    ) {
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

        let mut i = 0;
        let total = instances.len();

        while i < total {
            let end = (i + Quad::MAX).min(total);
            let amount = end - i;

            let instance_buffer = device.create_buffer_with_data(
                instances[i..end].as_bytes(),
                wgpu::BufferUsage::COPY_SRC,
            );

            encoder.copy_buffer_to_buffer(
                &instance_buffer,
                0,
                &self.instances,
                0,
                (mem::size_of::<Quad>() * amount) as u64,
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
                render_pass.set_bind_group(1, &texture.0, &[]);
                render_pass.set_index_buffer(&self.indices, 0, 0);
                render_pass.set_vertex_buffer(0, &self.vertices, 0, 0);
                render_pass.set_vertex_buffer(1, &self.instances, 0, 0);

                render_pass.draw_indexed(
                    0..QUAD_INDICES.len() as u32,
                    0,
                    0..amount as u32,
                );
            }

            i += Quad::MAX;
        }
    }
}

#[derive(Clone, Copy, AsBytes)]
#[repr(C)]
pub struct Vertex {
    _position: [f32; 2],
}

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const QUAD_VERTS: [Vertex; 4] = [
    Vertex {
        _position: [0.0, 0.0],
    },
    Vertex {
        _position: [1.0, 0.0],
    },
    Vertex {
        _position: [1.0, 1.0],
    },
    Vertex {
        _position: [0.0, 1.0],
    },
];

#[derive(Debug, Clone, Copy, AsBytes)]
#[repr(C)]
pub struct Quad {
    source: [f32; 4],
    scale: [f32; 2],
    translation: [f32; 2],
    pub layer: u32,
}

impl Quad {
    const MAX: usize = 100_000;
}

impl From<graphics::Quad> for Quad {
    fn from(quad: graphics::Quad) -> Quad {
        let source = quad.source;
        let position = quad.position;
        let (width, height) = quad.size;

        Quad {
            source: [source.x, source.y, source.width, source.height],
            translation: [position.x, position.y],
            scale: [width, height],
            layer: 0,
        }
    }
}

pub struct TextureBinding(wgpu::BindGroup);
