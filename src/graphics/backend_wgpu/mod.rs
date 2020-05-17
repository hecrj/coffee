mod font;
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

use crate::graphics::{Color, Transformation, WindowSettings};
use crate::{Error, Result};

#[allow(missing_debug_implementations)]
#[allow(missing_docs)]
pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    quad_pipeline: quad::Pipeline,
    triangle_pipeline: triangle::Pipeline,
    encoder: wgpu::CommandEncoder,
}

impl Gpu {
    pub(super) fn for_window(
        settings: WindowSettings,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<(Gpu, Surface)> {
        let vsync = settings.vsync;
        let builder = settings.into_builder(event_loop);
        let window = builder
            .build(event_loop)
            .map_err(|error| Error::WindowCreation(error.to_string()))?;

        let (mut device, queue) = futures::executor::block_on(async {
            let adapter = wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: None,
                },
                wgpu::BackendBit::all(),
            )
            .await
            .expect("Request adapter");

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits::default(),
                })
                .await;

            (device, queue)
        });

        let surface = Surface::new(window, &device, vsync);

        let quad_pipeline = quad::Pipeline::new(&mut device);
        let triangle_pipeline = triangle::Pipeline::new(&mut device);

        let encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("coffee::backend encoder"),
            });

        Ok((
            Gpu {
                device,
                queue,
                quad_pipeline,
                triangle_pipeline,
                encoder,
            },
            surface,
        ))
    }

    pub(super) fn clear(&mut self, view: &TargetView, color: Color) {
        let [r, g, b, a] = color.into_linear();

        let _ = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: r as f64,
                    g: g as f64,
                    b: b as f64,
                    a: a as f64,
                },
            }],
            depth_stencil_attachment: None,
        });
    }

    pub(super) fn upload_texture(
        &mut self,
        image: &image::DynamicImage,
    ) -> Texture {
        Texture::new(&mut self.device, &self.queue, &self.quad_pipeline, image)
    }

    pub(super) fn upload_texture_array(
        &mut self,
        layers: &[image::DynamicImage],
    ) -> Texture {
        Texture::new_array(
            &mut self.device,
            &self.queue,
            &self.quad_pipeline,
            layers,
        )
    }

    pub(super) fn create_drawable_texture(
        &mut self,
        width: u16,
        height: u16,
    ) -> texture::Drawable {
        texture::Drawable::new(
            &mut self.device,
            &self.queue,
            &self.quad_pipeline,
            width,
            height,
        )
    }

    pub(super) fn read_drawable_texture_pixels(
        &mut self,
        drawable: &texture::Drawable,
    ) -> image::DynamicImage {
        let new_encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("coffee::backend encoder"),
            },
        );

        let encoder = std::mem::replace(&mut self.encoder, new_encoder);

        drawable.read_pixels(&mut self.device, &self.queue, encoder)
    }

    pub(super) fn upload_font(&mut self, bytes: &'static [u8]) -> Font {
        Font::from_bytes(&mut self.device, bytes)
    }

    pub(super) fn draw_triangles(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
        view: &TargetView,
        transformation: &Transformation,
    ) {
        self.triangle_pipeline.draw(
            &mut self.device,
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
        self.quad_pipeline.draw_textured(
            &mut self.device,
            &mut self.encoder,
            texture.binding(),
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
        font.draw(&mut self.device, &mut self.encoder, target, transformation);
    }
}
