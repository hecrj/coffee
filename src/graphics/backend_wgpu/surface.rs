use super::{Gpu, TargetView};

pub struct Surface {
    window: winit::window::Window,
    surface: wgpu::Surface,
    swap_chain: wgpu::SwapChain,
    extent: wgpu::Extent3d,
    output: Option<wgpu::SwapChainOutput>,
    vsync: bool,
}

impl Surface {
    pub fn new(
        window: winit::window::Window,
        device: &wgpu::Device,
        vsync: bool,
    ) -> Surface {
        let surface = wgpu::Surface::create(&window);
        let size = window.inner_size();

        let (swap_chain, extent) =
            new_swap_chain(device, &surface, size, vsync);

        Surface {
            window,
            surface,
            swap_chain,
            extent,
            output: None,
            vsync,
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn target(&mut self) -> &TargetView {
        if self.output.is_none() {
            let output = self
                .swap_chain
                .get_next_texture()
                .expect("Get next texture");

            self.output = Some(output);
        }

        &self.output.as_ref().unwrap().view
    }

    pub fn resize(
        &mut self,
        gpu: &mut Gpu,
        size: winit::dpi::PhysicalSize<u32>,
    ) {
        let (swap_chain, extent) =
            new_swap_chain(&gpu.device, &self.surface, size, self.vsync);

        self.swap_chain = swap_chain;
        self.extent = extent;
        self.output = None;
    }

    pub fn swap_buffers(&mut self, gpu: &mut Gpu) {
        let new_encoder = gpu.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("coffee::backend::surface blit"),
            },
        );

        // We swap the current decoder by a new one here, so we can finish the
        // current frame
        let encoder = std::mem::replace(&mut gpu.encoder, new_encoder);

        gpu.queue.submit(&[encoder.finish()]);

        self.output = None;
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }
}

fn new_swap_chain(
    device: &wgpu::Device,
    surface: &wgpu::Surface,
    size: winit::dpi::PhysicalSize<u32>,
    vsync: bool,
) -> (wgpu::SwapChain, wgpu::Extent3d) {
    let present_mode = if vsync {
        wgpu::PresentMode::Mailbox
    } else {
        wgpu::PresentMode::Fifo
    };

    let swap_chain = device.create_swap_chain(
        surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode,
        },
    );

    let extent = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth: 1,
    };

    (swap_chain, extent)
}
