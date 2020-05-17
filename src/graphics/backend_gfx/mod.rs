mod font;
mod format;
mod quad;
mod surface;
pub mod texture;
mod triangle;
mod types;

pub use font::Font;
pub use quad::Quad;
pub use surface::Surface;
pub use texture::Texture;
pub use triangle::Vertex;
pub use types::TargetView;

use gfx::{self, Device};
use gfx_device_gl as gl;

use crate::graphics::{Color, Transformation, WindowSettings};
use crate::Result;

/// A link between your game and a graphics processor.
///
/// It is necessary to perform any kind of graphical operation, like loading
/// resources and drawing.
///
/// A [`Gpu`] can be obtained from a [`Window`] or a [`Frame`].
///
/// [`Gpu`]: struct.Gpu.html
/// [`Window`]: struct.Window.html
/// [`Frame`]: struct.Frame.html
#[allow(missing_debug_implementations)]
pub struct Gpu {
    device: gl::Device,
    factory: gl::Factory,
    encoder: gfx::Encoder<gl::Resources, gl::CommandBuffer>,
    triangle_pipeline: triangle::Pipeline,
    quad_pipeline: quad::Pipeline,
}

impl Gpu {
    pub(super) fn for_window(
        settings: WindowSettings,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<(Gpu, Surface)> {
        let (surface, device, mut factory) =
            Surface::new(settings, event_loop)?;

        let mut encoder: gfx::Encoder<gl::Resources, gl::CommandBuffer> =
            factory.create_command_buffer().into();

        let triangle_pipeline = triangle::Pipeline::new(
            &mut factory,
            &mut encoder,
            surface.target(),
        );

        let quad_pipeline =
            quad::Pipeline::new(&mut factory, &mut encoder, surface.target());

        Ok((
            Gpu {
                device,
                factory,
                encoder,
                triangle_pipeline,
                quad_pipeline,
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

    pub(super) fn read_drawable_texture_pixels(
        &mut self,
        drawable: &texture::Drawable,
    ) -> image::DynamicImage {
        self.flush();

        drawable.read_pixels(&mut self.device, &mut self.factory)
    }

    pub(super) fn upload_font(&mut self, bytes: &'static [u8]) -> Font {
        Font::from_bytes(&mut self.factory, bytes)
    }

    pub(super) fn draw_triangles(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
        view: &TargetView,
        transformation: &Transformation,
    ) {
        self.triangle_pipeline.draw(
            &mut self.factory,
            &mut self.encoder,
            vertices,
            indices,
            transformation,
            view,
        );
    }

    pub(super) fn draw_texture_quads(
        &mut self,
        texture: &Texture,
        instances: &[Quad],
        view: &TargetView,
        transformation: &Transformation,
    ) {
        self.quad_pipeline.bind_texture(texture);

        self.quad_pipeline.draw_textured(
            &mut self.encoder,
            instances,
            transformation,
            view,
        );
    }

    pub(super) fn draw_font(
        &mut self,
        font: &mut Font,
        target: &TargetView,
        transformation: Transformation,
    ) {
        font.draw(&mut self.encoder, target, transformation);
    }
}
