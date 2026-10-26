use core::cell::Cell;

use spring_addons::rules::Gadget;

use crate::state::Global;

#[derive(Default)]
pub struct RoundGadget {
    started: Cell<bool>,
    decoration_round: Cell<u32>,
    decoration_at: Cell<Option<i32>>,
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

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let spring = ctx.spring();
        if !self.started.replace(true) {
            crate::round::populate(spring, ctx.global());
        }
        let round = ctx.global().round.get();
        if self.decoration_round.replace(round) != round {
            self.decoration_at.set(Some(frame + 3));
        }
        if self.decoration_at.get().is_some_and(|due| frame >= due) {
            self.decoration_at.set(None);
            crate::with_addon_context(|ctx| {
                ctx.commands()
                    .run(|spring| crate::spawn::populate_decorations(spring));
            });
        }
    }
}
