pub mod behavior;
use core::cell::RefCell;

use spring_addons::rules::Gadget;

use self::behavior::GrowthManager;
use crate::state::Global;
use crate::story::STAGE_FREE_PLAY;

#[derive(Default)]
pub struct GrowthGadget {
    growth: RefCell<Option<(u32, GrowthManager)>>,
}

impl GrowthGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for GrowthGadget {
    fn name(&self) -> &'static str {
        "Growth"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        let mut growth = self.growth.borrow_mut();
        let round = global.round.get();
        let growth = match &mut *growth {
            Some((scheduled, growth)) if *scheduled == round => growth,
            slot => &mut slot.insert((round, GrowthManager::starting_at(frame))).1,
        };
        if global.story.access().stage != STAGE_FREE_PLAY {
            return;
        }
        let spire_id = global.spire.access().unit_id;
        growth.update(frame, spire_id);
    }
}
