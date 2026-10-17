use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::state::Global;

#[derive(Default)]
pub struct SpireGadget;

impl SpireGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for SpireGadget {
    fn name(&self) -> &'static str {
        "Spire"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let spring = ctx.spring();
        let total_power = ctx.global().trees.access().total_power();
        ctx.global()
            .spire
            .access_mut()
            .update(spring, frame, total_power);
    }

    fn unit_created(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        unit: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        let spring = ctx.spring();
        if crate::definitions::defs(spring).is_spire(def) {
            ctx.global().spire.access_mut().unit_id = unit;
        }
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        let spring = ctx.spring();
        if !crate::definitions::defs(spring).is_spire(event.unit_def_id) {
            return;
        }
        let global = ctx.global();
        global.spire.access_mut().unit_id = -1;
        global.victory.access_mut().notify_spire_destroyed();
    }
}
