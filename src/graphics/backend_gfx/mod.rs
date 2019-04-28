mod font;
mod format;
mod pipeline;
mod surface;
pub mod texture;
mod types;

pub use font::Font;
pub use pipeline::Instance;
pub use surface::{winit, Surface};
pub use texture::Texture;
pub use types::{DepthView, TargetView};

use gfx::{self, Device};
use gfx_device_gl as gl;

use crate::graphics::{Color, Transformation};
use crate::Result;
use pipeline::Pipeline;

/// A link between your game and a graphics processor.
///
/// It is necessary to perform any kind of graphical operation, like loading
/// resources and drawing.
///
/// [`Game`] provides a value of this type in the [`interact`] method. A [`Gpu`]
/// can also be obtained from a [`Window`].
///
/// [`Gpu`]: struct.Gpu.html
/// [`Game`]: ../trait.Game.html
/// [`interact`]: ../trait.Game.html#tymethod.interact
/// [`Window`]: struct.Window.html
#[allow(missing_debug_implementations)]
pub struct Gpu {
    device: gl::Device,
    factory: gl::Factory,
    encoder: gfx::Encoder<gl::Resources, gl::CommandBuffer>,
    pipeline: Pipeline,
}

impl Gpu {
    pub(super) fn for_window(
        builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
    ) -> Result<(Gpu, Surface)> {
        let (surface, device, mut factory) =
            Surface::new(builder, events_loop)?;

        let mut encoder: gfx::Encoder<gl::Resources, gl::CommandBuffer> =
            factory.create_command_buffer().into();

        let pipeline =
            Pipeline::new(&mut factory, &mut encoder, surface.target());

        Ok((
            Gpu {
                device,
                factory,
                encoder,
                pipeline,
            },
            surface,
        ))
    }

    pub(super) fn clear(&mut self, view: &TargetView, color: Color) {
        let typed_render_target: gfx::handle::RenderTargetView<
            gl::Resources,
            gfx::format::Srgba8,
        > = gfx::memory::Typed::new(view.clone());

        self.encoder
            .clear(&typed_render_target, color.into_linear())
    }

    fn flush(&mut self) {
        self.encoder.flush(&mut self.device);
    }

    fn cleanup(&mut self) {
        self.device.cleanup();
    }

    pub(super) fn upload_texture(
        &mut self,
        image: &image::DynamicImage,
    ) -> Texture {
        Texture::new(&mut self.factory, image)
    }

    pub(super) fn upload_texture_array(
        &mut self,
        layers: &[image::DynamicImage],
    ) -> Texture {
        Texture::new_array(&mut self.factory, layers)
    }

    pub(super) fn create_drawable_texture(
        &mut self,
        width: u16,
        height: u16,
    ) -> texture::Drawable {
        texture::Drawable::new(&mut self.factory, width, height)
    }

    pub(super) fn upload_font(&mut self, bytes: &'static [u8]) -> Font {
        Font::from_bytes(&mut self.factory, bytes)
    }

    pub(super) fn draw_texture_quads(
        &mut self,
        texture: &Texture,
        instances: &[Instance],
        view: &TargetView,
        transformation: &Transformation,
    ) {
        self.pipeline.bind_texture(texture);

        self.pipeline.draw_quads(
            &mut self.encoder,
            instances,
            &transformation,
            &view,
        );
    }

    pub(super) fn draw_font(
        &mut self,
        font: &mut Font,
        target: &TargetView,
        depth: &DepthView,
        _target_width: u32,
        _target_height: u32,
    ) {
        font.draw(&mut self.encoder, target, depth);
    }
}
