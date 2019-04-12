mod font;
mod pipeline;
pub mod texture;
mod types;

pub use font::Font;
pub use pipeline::Instance;
pub use texture::Texture;
pub use types::{DepthView, TargetView};
pub use wgpu_winit as winit;

use std::rc::Rc;

use wgpu;

use crate::graphics::{Color, Transformation};
use pipeline::Pipeline;

pub struct Gpu {
    device: wgpu::Device,
    pipeline: Pipeline,
}

impl Gpu {
    pub(super) fn new(mut device: wgpu::Device) -> Gpu {
        let pipeline = Pipeline::new(&mut device);

        Gpu { device, pipeline }
    }

    pub(super) fn window(
        builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
    ) -> (Gpu, WindowedContext) {
        let window = builder.build(events_loop).unwrap();

        let instance = wgpu::Instance::new();
        let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
            power_preference: wgpu::PowerPreference::HighPerformance,
        });
        let device = adapter.create_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
        });

        let context = WindowedContext::new(window, &instance, &device);

        (Gpu::new(device), context)
    }

    pub(super) fn clear(&mut self, view: &TargetView, color: Color) {
        let [r, g, b, a]: [f32; 4] = color.into();

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 },
        );

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &view,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color { r, g, b, a },
            }],
            depth_stencil_attachment: None,
        });

        self.device.get_queue().submit(&[encoder.finish()]);
    }

    pub(super) fn upload_texture(
        &mut self,
        image: &image::DynamicImage,
    ) -> Texture {
        Texture::new(&mut self.device, &self.pipeline, image)
    }

    pub(super) fn upload_texture_array(
        &mut self,
        layers: &[image::DynamicImage],
    ) -> Texture {
        Texture::new_array(&mut self.device, &self.pipeline, layers)
    }

    pub(super) fn create_drawable_texture(
        &mut self,
        width: u16,
        height: u16,
    ) -> texture::Drawable {
        texture::Drawable::new(&mut self.device, &self.pipeline, width, height)
    }

    pub(super) fn upload_font(&mut self, bytes: &'static [u8]) -> Font {
        Font::from_bytes(bytes)
    }

    pub(super) fn draw_texture_quads(
        &mut self,
        texture: &Texture,
        instances: &[Instance],
        view: &TargetView,
        transformation: &Transformation,
    ) {
        self.pipeline.draw_texture_quads(
            &mut self.device,
            texture.binding(),
            instances,
            &transformation,
            &view,
        );
    }

    pub(super) fn draw_font(
        &mut self,
        _font: &mut Font,
        _target: &TargetView,
        _depth: &DepthView,
    ) {
    }
}

pub struct WindowedContext {
    window: winit::Window,
    surface: wgpu::Surface,
    swap_chain: wgpu::SwapChain,
    extent: wgpu::Extent3d,
    buffer: wgpu::Texture,
    target: TargetView,
}

impl WindowedContext {
    pub fn new(
        window: winit::Window,
        instance: &wgpu::Instance,
        device: &wgpu::Device,
    ) -> WindowedContext {
        let size = window
            .get_inner_size()
            .unwrap()
            .to_physical(window.get_hidpi_factor());

        let surface = instance.create_surface(&window);
        let swap_chain = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT
                    | wgpu::TextureUsageFlags::TRANSFER_DST,
                format: wgpu::TextureFormat::Bgra8Unorm,
                width: size.width.round() as u32,
                height: size.height.round() as u32,
            },
        );

        let extent = wgpu::Extent3d {
            width: size.width.round() as u32,
            height: size.height.round() as u32,
            depth: 1,
        };

        let buffer = device.create_texture(&wgpu::TextureDescriptor {
            size: extent,
            dimension: wgpu::TextureDimension::D2,
            array_size: 1,
            format: wgpu::TextureFormat::Bgra8Unorm,
            usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT
                | wgpu::TextureUsageFlags::TRANSFER_SRC,
        });

        let target = Rc::new(buffer.create_default_view());

        WindowedContext {
            window,
            surface,
            swap_chain,
            extent,
            buffer,
            target,
        }
    }

    pub fn window(&self) -> &winit::Window {
        &self.window
    }

    pub fn target(&self) -> &TargetView {
        &self.target
    }

    pub fn depth(&self) -> &DepthView {
        &()
    }

    pub(super) fn update_viewport(&mut self) {}

    pub fn swap_buffers(&mut self, gpu: &mut Gpu) {
        let output = self.swap_chain.get_next_texture();
        let mut encoder = gpu.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 },
        );

        encoder.copy_texture_to_texture(
            wgpu::TextureCopyView {
                texture: &self.buffer,
                level: 0,
                slice: 0,
                origin: wgpu::Origin3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            wgpu::TextureCopyView {
                texture: &output.texture,
                level: 0,
                slice: 0,
                origin: wgpu::Origin3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            self.extent,
        );

        gpu.device.get_queue().submit(&[encoder.finish()]);
    }
}
