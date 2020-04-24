use crate::graphics;

// Null debug implementation
#[allow(missing_debug_implementations)]
#[allow(missing_docs)]
pub struct Debug {}

impl Debug {
    pub(crate) fn new(_gpu: &mut graphics::Gpu) -> Self {
        Self {}
    }

    pub(crate) fn loading_started(&mut self) {}
    pub(crate) fn loading_finished(&mut self) {}
    pub(crate) fn frame_started(&mut self) {}
    pub(crate) fn frame_finished(&mut self) {}
    pub(crate) fn interact_started(&mut self) {}
    pub(crate) fn interact_finished(&mut self) {}
    pub(crate) fn update_started(&mut self) {}
    pub(crate) fn update_finished(&mut self) {}
    pub(crate) fn draw_started(&mut self) {}
    pub(crate) fn draw_finished(&mut self) {}
    pub(crate) fn ui_started(&mut self) {}
    pub(crate) fn ui_finished(&mut self) {}
    pub(crate) fn debug_started(&mut self) {}
    pub(crate) fn debug_finished(&mut self) {}

    #[allow(dead_code)]
    pub(crate) fn toggle(&mut self) {}

    pub(crate) fn is_enabled(&self) -> bool {
        false
    }

    #[allow(missing_docs)]
    pub fn draw(&mut self, _frame: &mut graphics::Frame<'_>) {}
}
