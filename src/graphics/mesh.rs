use crate::graphics::{gpu, Color, Rectangle, Shape, Target};

use lyon_tessellation as lyon;

/// A set of shapes that can be drawn.
///
/// # Tolerance
/// When shapes contain curves or arcs, they will be approximated using line
/// segments. The `tolerance` parameter controls this approximation by
/// establishing the maximum distance between a curve and its line segments.
///
/// The lower the tolerance provided, the better a [`Mesh`] will approximate
/// a [`Shape`]. However, a lower tolerance can have a noticeable performance
/// impact. Use it wisely!
///
/// [`Mesh`]: struct.Mesh.html
/// [`Shape`]: enum.Shape.html
#[derive(Debug, Clone)]
pub struct Mesh {
    tolerance: f32,
    buffers: lyon::VertexBuffers<gpu::Vertex, u32>,
}

impl Mesh {
    /// Creates a new empty [`Mesh`] with a default tolerance of `0.1`.
    ///
    /// [`Mesh`]: struct.Mesh.html
    pub fn new() -> Mesh {
        Mesh {
            tolerance: 0.1,
            buffers: lyon::VertexBuffers::new(),
        }
    }

    /// Creates a new empty [`Mesh`] with the given tolerance.
    ///
    /// Providing a lower tolerance here can allow you to zoom in your [`Mesh`]
    /// using a [`Transformation`] and still observe smooth curves. See
    /// [Tolerance](#tolerance).
    ///
    /// [`Mesh`]: struct.Mesh.html
    /// [`Transformation`]: struct.Transformation.html
    pub fn new_with_tolerance(tolerance: f32) -> Mesh {
        Mesh {
            tolerance,
            buffers: lyon::VertexBuffers::new(),
        }
    }

    /// Returns true if the [`Mesh`] is empty.
    ///
    /// [`Mesh`]: struct.Mesh.html
    pub fn is_empty(&self) -> bool {
        self.buffers.vertices.is_empty()
    }

    /// Adds a filled [`Shape`] to the [`Mesh`].
    ///
    /// [`Shape`]: enum.Shape.html
    /// [`Mesh`]: struct.Mesh.html
    #[inline]
    pub fn fill(&mut self, shape: Shape, color: Color) {
        let mut builder = lyon::BuffersBuilder::new(
            &mut self.buffers,
            WithColor(color.into_linear()),
        );

        match shape {
            Shape::Rectangle(Rectangle {
                x,
                y,
                width,
                height,
            }) => {
                let _ = lyon::basic_shapes::fill_rectangle(
                    &lyon::math::rect(x, y, width, height),
                    &Self::fill_options(self.tolerance),
                    &mut builder,
                )
                .expect("Fill rectangle");
            }
            Shape::Circle { center, radius } => {
                let _ = lyon::basic_shapes::fill_circle(
                    lyon::math::point(center.x, center.y),
                    radius,
                    &Self::fill_options(self.tolerance),
                    &mut builder,
                )
                .expect("Fill circle");
            }
            Shape::Ellipse {
                center,
                horizontal_radius,
                vertical_radius,
                rotation,
            } => {
                let _ = lyon::basic_shapes::fill_ellipse(
                    lyon::math::point(center.x, center.y),
                    lyon::math::vector(horizontal_radius, vertical_radius),
                    lyon::math::Angle::radians(rotation),
                    &Self::fill_options(self.tolerance),
                    &mut builder,
                )
                .expect("Fill ellipse");
            }
            Shape::Polyline { points } => {
                let _ = lyon::basic_shapes::fill_polyline(
                    points
                        .iter()
                        .map(|point| lyon::math::point(point.x, point.y)),
                    &mut lyon::FillTessellator::new(),
                    &Self::fill_options(self.tolerance),
                    &mut builder,
                )
                .expect("Fill polyline");
            }
        }
    }

    /// Adds the stroke of a [`Shape`] to the [`Mesh`].
    ///
    /// [`Shape`]: enum.Shape.html
    /// [`Mesh`]: struct.Mesh.html
    #[inline]
    pub fn stroke(&mut self, shape: Shape, color: Color, width: f32) {
        let mut builder = lyon::BuffersBuilder::new(
            &mut self.buffers,
            WithColor(color.into_linear()),
        );

        match shape {
            Shape::Rectangle(Rectangle {
                x,
                y,
                width: rect_width,
                height,
            }) => {
                let _ = lyon::basic_shapes::stroke_rectangle(
                    &lyon::math::rect(x, y, rect_width, height),
                    &Self::stroke_options(self.tolerance, width),
                    &mut builder,
                )
                .expect("Stroke rectangle");
            }
            Shape::Circle { center, radius } => {
                let _ = lyon::basic_shapes::stroke_circle(
                    lyon::math::point(center.x, center.y),
                    radius,
                    &Self::stroke_options(self.tolerance, width),
                    &mut builder,
                )
                .expect("Stroke circle");
            }
            Shape::Ellipse {
                center,
                horizontal_radius,
                vertical_radius,
                rotation,
            } => {
                let _ = lyon::basic_shapes::stroke_ellipse(
                    lyon::math::point(center.x, center.y),
                    lyon::math::vector(horizontal_radius, vertical_radius),
                    lyon::math::Angle::radians(rotation),
                    &Self::stroke_options(self.tolerance, width),
                    &mut builder,
                )
                .expect("Stroke ellipse");
            }
            Shape::Polyline { points } => {
                let _ = lyon::basic_shapes::stroke_polyline(
                    points
                        .iter()
                        .map(|point| lyon::math::point(point.x, point.y)),
                    false,
                    &Self::stroke_options(self.tolerance, width),
                    &mut builder,
                )
                .expect("Stroke polyline");
            }
        }
    }

    /// Draws the [`Mesh`] on the given [`Target`].
    ///
    /// [`Mesh`]: struct.Mesh.html
    /// [`Target`]: struct.Target.html
    pub fn draw(&self, target: &mut Target<'_>) {
        target.draw_triangles(&self.buffers.vertices, &self.buffers.indices);
    }

    fn fill_options(tolerance: f32) -> lyon::FillOptions {
        lyon::FillOptions::DEFAULT
            .with_tolerance(tolerance)
            .with_normals(false)
    }

    fn stroke_options(tolerance: f32, width: f32) -> lyon::StrokeOptions {
        lyon::StrokeOptions::DEFAULT
            .with_tolerance(tolerance)
            .with_line_width(width)
    }
}

struct WithColor([f32; 4]);

impl lyon::VertexConstructor<lyon::FillVertex, gpu::Vertex> for WithColor {
    fn new_vertex(&mut self, vertex: lyon::FillVertex) -> gpu::Vertex {
        gpu::Vertex::new([vertex.position.x, vertex.position.y], self.0)
    }
}

impl lyon::VertexConstructor<lyon::StrokeVertex, gpu::Vertex> for WithColor {
    fn new_vertex(&mut self, vertex: lyon::StrokeVertex) -> gpu::Vertex {
        gpu::Vertex::new([vertex.position.x, vertex.position.y], self.0)
    }
}
