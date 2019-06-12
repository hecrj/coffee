use crate::graphics::{gpu, Color, Shape, Target};

use lyon_tessellation as lyon;

#[derive(Debug)]
pub struct Mesh {
    buffers: lyon::VertexBuffers<gpu::Vertex, u16>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            buffers: lyon::VertexBuffers::new(),
        }
    }

    #[inline]
    pub fn fill(&mut self, shape: Shape, color: Color) {
        let mut builder = lyon::BuffersBuilder::new(
            &mut self.buffers,
            WithColor(color.into_linear()),
        );

        match shape {
            Shape::Circle { center, radius } => {
                let _ = lyon::basic_shapes::fill_circle(
                    lyon::math::point(center.x, center.y),
                    radius,
                    &Self::fill_options(),
                    &mut builder,
                )
                .expect("Circle mesh");
            }
        }
    }

    #[inline]
    pub fn stroke(&mut self, shape: Shape, color: Color, width: u16) {}

    pub fn draw(&self, target: &mut Target<'_>) {
        target.draw_triangles(&self.buffers.vertices, &self.buffers.indices);
    }

    fn fill_options() -> lyon::FillOptions {
        lyon::FillOptions::DEFAULT.with_normals(false)
    }
}

struct WithColor([f32; 4]);

impl lyon::VertexConstructor<lyon::FillVertex, gpu::Vertex> for WithColor {
    fn new_vertex(&mut self, vertex: lyon::FillVertex) -> gpu::Vertex {
        gpu::Vertex::new([vertex.position.x, vertex.position.y], self.0)
    }
}
