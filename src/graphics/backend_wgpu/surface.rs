use std::rc::Rc;

use super::{DepthView, Gpu, TargetView};
pub use wgpu_winit as winit;

pub struct Surface {
    window: winit::Window,
    surface: wgpu::Surface,
    swap_chain: wgpu::SwapChain,
    extent: wgpu::Extent3d,
    buffer: wgpu::Texture,
    target: TargetView,
}

impl Surface {
    pub fn new(
        window: winit::Window,
        instance: &wgpu::Instance,
        device: &wgpu::Device,
    ) -> Surface {
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

        Surface {
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

    pub fn update_viewport(&mut self) {}

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
