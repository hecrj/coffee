use std::fmt;
use std::rc::Rc;

use super::types::TargetView;
use crate::graphics::gpu::quad::{self, Pipeline};
use crate::graphics::Transformation;

#[derive(Clone)]
pub struct Texture {
    texture: Rc<wgpu::Texture>,
    view: TargetView,
    binding: Rc<quad::TextureBinding>,
    width: u16,
    height: u16,
    layers: u16,
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Texture {{ width: {}, height: {}, layers: {} }}",
            self.width, self.height, self.layers
        )
    }
}

impl Texture {
    pub(super) fn new(
        device: &mut wgpu::Device,
        pipeline: &Pipeline,
        image: &image::DynamicImage,
    ) -> Texture {
        let bgra = image.to_bgra();
        let width = bgra.width() as u16;
        let height = bgra.height() as u16;

        let (texture, view, binding) = create_texture_array(
            device,
            pipeline,
            width,
            height,
            Some(&[&bgra.into_raw()[..]]),
            wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        );

        Texture {
            texture: Rc::new(texture),
            view: Rc::new(view),
            binding: Rc::new(binding),
            width,
            height,
            layers: 1,
        }
    }

    pub(super) fn new_array(
        device: &mut wgpu::Device,
        pipeline: &Pipeline,
        layers: &[image::DynamicImage],
    ) -> Texture {
        let first_layer = &layers[0].to_bgra();
        let width = first_layer.width() as u16;
        let height = first_layer.height() as u16;

        let bgra: Vec<Vec<u8>> =
            layers.iter().map(|i| i.to_bgra().into_raw()).collect();

        let raw_layers: Vec<&[u8]> = bgra.iter().map(|i| &i[..]).collect();

        let (texture, view, binding) = create_texture_array(
            device,
            pipeline,
            width,
            height,
            Some(&raw_layers[..]),
            wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        );

        Texture {
            texture: Rc::new(texture),
            view: Rc::new(view),
            binding: Rc::new(binding),
            width,
            height,
            layers: layers.len() as u16,
        }
    }

    pub(super) fn view(&self) -> &TargetView {
        &self.view
    }

    pub(super) fn binding(&self) -> &quad::TextureBinding {
        &self.binding
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Clone)]
pub struct Drawable {
    texture: Texture,
}

impl Drawable {
    pub fn new(
        device: &mut wgpu::Device,
        pipeline: &Pipeline,
        width: u16,
        height: u16,
    ) -> Drawable {
        let (texture, view, binding) = create_texture_array(
            device,
            pipeline,
            width,
            height,
            None,
            wgpu::TextureUsage::OUTPUT_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        );

        let texture = Texture {
            texture: Rc::new(texture),
            view: Rc::new(view),
            binding: Rc::new(binding),
            width,
            height,
            layers: 1,
        };

        Drawable { texture }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn target(&self) -> &TargetView {
        self.texture().view()
    }

    pub fn render_transformation() -> Transformation {
        Transformation::identity()
    }
}

// Helpers
fn create_texture_array(
    device: &mut wgpu::Device,
    pipeline: &Pipeline,
    width: u16,
    height: u16,
    layers: Option<&[&[u8]]>,
    usage: wgpu::TextureUsage,
) -> (wgpu::Texture, wgpu::TextureView, quad::TextureBinding) {
    let extent = wgpu::Extent3d {
        width: width as u32,
        height: height as u32,
        depth: 1,
    };

    let layer_count = layers.map(|l| l.len()).unwrap_or(1) as u32;

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: extent,
        array_layer_count: layer_count,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        usage,
    });

    if let Some(layers) = layers {
        // TODO: Learn more about iterators and find a way to improve this.
        // Are these many copies needed?
        let slice: Vec<u8> =
            layers.iter().cloned().flatten().cloned().collect();

        let temp_buf = device
            .create_buffer_mapped(slice.len(), wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&slice[..]);

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                todo: 0,
            });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &temp_buf,
                offset: 0,
                row_pitch: 4 * width as u32,
                image_height: height as u32,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                array_layer: 0,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            extent,
        );

        device.get_queue().submit(&[encoder.finish()]);
    }

    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        dimension: wgpu::TextureViewDimension::D2Array,
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        array_layer_count: layer_count,
    });

    let binding = pipeline.create_texture_binding(device, &view);

    (texture, view, binding)
}
