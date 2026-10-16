use core::cell::RefCell;

use spring_addons::rules::Gadget;

use crate::integration::{self, TestTracker};
use crate::state::Global;

#[derive(Default)]
pub struct IntegrationGadget {
    tracker: RefCell<TestTracker>,
}

impl IntegrationGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for IntegrationGadget {
    fn name(&self) -> &'static str {
        "Integration"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        integration::announce();
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        integration::update(ctx, &mut self.tracker.borrow_mut(), frame);
    }

    fn handle_lua_msg(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        data: &[u8],
    ) {
        if data == flove_shared::suites::UI_DONE.as_bytes() {
            integration::ui_suite_done(&mut self.tracker.borrow_mut());
        }
    }
}
