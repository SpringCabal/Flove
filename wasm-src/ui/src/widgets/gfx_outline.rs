pub mod outline;
use core::cell::RefCell;

use spring_addons::event::ViewGeometry;
use spring_addons::rules::UnitDestroyedEvent;
use spring_addons::ui::Widget;

use self::outline::{CreatedFrames, Outline};
use crate::state::Global;

#[derive(Default)]
pub struct GfxOutlineWidget {
    outline: RefCell<Outline>,
    created_frames: RefCell<CreatedFrames>,
}

impl GfxOutlineWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for GfxOutlineWidget {
    fn name(&self) -> &'static str {
        "Outline"
    }

    fn draw_world_pre_unit(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        outline::draw_world_pre_unit(&self.outline, &self.created_frames);
    }

    fn unit_destroyed(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        self.created_frames.borrow_mut().forget(event.unit_id);
    }

    fn view_resize(&self, _ctx: &spring_addons::AddonContext<'_, Global>, geometry: &ViewGeometry) {
        self.outline
            .borrow_mut()
            .on_view_resize(geometry.view_size.0, geometry.view_size.1);
    }
}
