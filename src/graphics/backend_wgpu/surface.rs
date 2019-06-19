use std::rc::Rc;

use super::{Gpu, TargetView};
pub use wgpu::winit;

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
        let surface = instance.create_surface(&window);

        let size = window
            .get_inner_size()
            // TODO: Find out when and why the "inner size" might not be available
            // and do something smarter here.
            .unwrap_or(winit::dpi::LogicalSize {
                width: 1280.0,
                height: 1024.0,
            })
            .to_physical(window.get_hidpi_factor());

        let (swap_chain, extent, buffer, target) =
            new_swap_chain(device, &surface, size);

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

    pub fn resize(&mut self, gpu: &mut Gpu, size: winit::dpi::PhysicalSize) {
        let (swap_chain, extent, buffer, target) =
            new_swap_chain(&gpu.device, &self.surface, size);

        self.swap_chain = swap_chain;
        self.extent = extent;
        self.buffer = buffer;
        self.target = target;
    }

    pub fn swap_buffers(&mut self, gpu: &mut Gpu) {
        let output = self.swap_chain.get_next_texture();

        let new_encoder = gpu.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 },
        );

        // We swap the current decoder by a new one here, so we can finish the
        // current frame
        let mut encoder = std::mem::replace(&mut gpu.encoder, new_encoder);

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

fn new_swap_chain(
    device: &wgpu::Device,
    surface: &wgpu::Surface,
    size: winit::dpi::PhysicalSize,
) -> (wgpu::SwapChain, wgpu::Extent3d, wgpu::Texture, TargetView) {
    let swap_chain = device.create_swap_chain(
        surface,
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
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT
            | wgpu::TextureUsageFlags::TRANSFER_SRC,
    });

    let target = Rc::new(buffer.create_default_view());

    (swap_chain, extent, buffer, target)
}
