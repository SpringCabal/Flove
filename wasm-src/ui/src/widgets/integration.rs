use core::cell::RefCell;

use spring_addons::ui::Widget;

use crate::integration::{self, UiTestTracker};
use crate::state::Global;

#[derive(Default)]
pub struct IntegrationWidget {
    tracker: RefCell<UiTestTracker>,
}

impl IntegrationWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for IntegrationWidget {
    fn name(&self) -> &'static str {
        "Integration"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        integration::update(ctx.global(), &mut self.tracker.borrow_mut());
    }
}
