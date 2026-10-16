use core::cell::Cell;

use spring_addons::ui::Widget;

use crate::camera;
use crate::state::Global;

#[derive(Default)]
pub struct GuiCameraWidget {
    round: Cell<Option<u32>>,
}

impl GuiCameraWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for GuiCameraWidget {
    fn name(&self) -> &'static str {
        "GuiCamera"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        let round = ctx.global().game.access().round;
        if self.round.replace(Some(round)) != Some(round) {
            camera::reset_camera();
        }
    }
}
