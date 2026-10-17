use core::cell::Cell;

use spring_addons::rules::Gadget;

use crate::state::Global;

#[derive(Default)]
pub struct RoundGadget {
    started: Cell<bool>,
}

impl RoundGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for RoundGadget {
    fn name(&self) -> &'static str {
        "Round"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let spring = ctx.spring();
        if !self.started.replace(true) {
            crate::round::populate(spring, ctx.global());
        }
    }
}
