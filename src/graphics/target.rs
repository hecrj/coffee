use crate::graphics::gpu::{self, Font, Gpu, TargetView, Texture, Vertex};
use crate::graphics::{Color, Transformation};

/// A rendering target.
///
/// In Coffee, all the draw operations need an explicit [`Target`]. You can
/// obtain one from a [`Frame`] or a [`Canvas`].
///
/// The default coordinate system of a [`Target`] has the origin `(0, 0)` at
/// its top-left corner and `(Target::width, Target::height)` at its bottom-right
/// corner.
///
/// [`Target`]: struct.Target.html
/// [`Frame`]: struct.Frame.html
/// [`Canvas`]: struct.Canvas.html
pub struct Target<'a> {
    gpu: &'a mut Gpu,
    view: &'a TargetView,
    transformation: Transformation,
}

impl<'a> Target<'a> {
    pub(super) fn new(
        gpu: &'a mut Gpu,
        view: &'a TargetView,
        width: f32,
        height: f32,
    ) -> Self {
        Target {
            gpu,
            view,
            transformation: Transformation::orthographic(width, height),
        }
    }

    pub(super) fn with_transformation(
        gpu: &'a mut Gpu,
        view: &'a TargetView,
        width: f32,
        height: f32,
        transformation: Transformation,
    ) -> Self {
        let mut target = Self::new(gpu, view, width, height);
        target.transformation = transformation * target.transformation;
        target
    }

    /// Creates a new [`Target`] applying the given transformation.
    ///
    /// This is equivalent to multiplying the current [`Target`] transform by
    /// the provided transform.
    ///
    /// You can use blocks to emulate a transformation stack! Imagine we want to
    /// apply a camera translation with some zoom, but only use it to draw a
    /// particular scene. We can simply do:
    ///
    /// ```
    /// use coffee::graphics::{Frame, Transformation, Vector};
    ///
    /// fn draw_something(frame: &mut Frame) {
    ///     let mut target = frame.as_target();
    ///
    ///     // We can draw stuff on `target` here
    ///     // ...
    ///
    ///     {
    ///         let transformation = Transformation::scale(2.0)
    ///             * Transformation::translate(Vector::new(10.0, 10.0));
    ///
    ///         let mut camera = target.transform(transformation);
    ///
    ///         // Use `camera` to draw the particular scene here
    ///         // ...
    ///     }
    ///
    ///     // We can keep using `target` as if no transformation happened
    ///     // ...
    /// }
    /// ```
    ///
    /// [`Target`]: struct.Target.html
    pub fn transform(&mut self, transformation: Transformation) -> Target<'_> {
        Target {
            gpu: self.gpu,
            view: self.view,
            transformation: self.transformation * transformation,
        }
    }

    /// Clears the [`Target`] with the given [`Color`].
    ///
    /// [`Target`]: struct.Target.html
    /// [`Color`]: struct.Color.html
    pub fn clear(&mut self, color: Color) {
        self.gpu.clear(&self.view, color);
    }

    pub(super) fn draw_triangles(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
    ) {
        self.gpu.draw_triangles(
            vertices,
            indices,
            &self.view,
            &self.transformation,
        );
    }

    pub(super) fn draw_texture_quads(
        &mut self,
        texture: &Texture,
        instances: &[gpu::Quad],
    ) {
        self.gpu.draw_texture_quads(
            texture,
            instances,
            &self.view,
            &self.transformation,
        );
    }

    pub(in crate::graphics) fn draw_font(&mut self, font: &mut Font) {
        self.gpu.draw_font(font, &self.view, self.transformation);
    }
}

impl<'a> std::fmt::Debug for Target<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Target {{ transformation: {:?} }}", self.transformation)
    }
}
