use crate::graphics;

// Null debug implementation
#[cfg(not(debug_assertions))]
pub struct Debug {}

#[cfg(not(debug_assertions))]
impl Debug {
    pub fn new(_gpu: &mut graphics::Gpu, _draw_rate: u16) -> Self {
        Self {}
    }

    pub(crate) fn loading_started(&mut self) {}

    pub(crate) fn loading_finished(&mut self) {}

    pub(crate) fn frame_started(&mut self) {}
    pub(crate) fn frame_finished(&mut self) {}

    pub(crate) fn event_loop_started(&mut self) {}

    pub(crate) fn event_loop_finished(&mut self) {}

    pub(crate) fn update_started(&mut self) {}

    pub(crate) fn update_finished(&mut self) {}

    pub(crate) fn draw_started(&mut self) {}

    pub(crate) fn draw_finished(&mut self) {}

    pub(crate) fn toggle(&mut self) {}

    pub(crate) fn is_enabled(&self) -> bool {
        false
    }

    pub fn draw(
        &mut self,
        _frame: &mut graphics::Frame,
    ) -> graphics::Result<()> {
        Ok(())
    }
}
