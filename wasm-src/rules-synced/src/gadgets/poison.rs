pub mod behavior;
use core::cell::RefCell;

use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use self::behavior::PoisonManager;
use crate::state::Global;

#[derive(Default)]
pub struct PoisonGadget {
    poison: RefCell<PoisonManager>,
}

impl PoisonGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for PoisonGadget {
    fn name(&self) -> &'static str {
        "Poison"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let spring = _ctx.spring();
        crate::gadgets::poison::behavior::spray(
            spring,
            &self.poison.borrow().poison_mushrooms,
            frame,
        );
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        unit: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        let spring = _ctx.spring();
        if def == crate::definitions::defs(spring).poison_mushroom {
            self.poison.borrow_mut().register(unit);
        }
    }

    fn unit_destroyed(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        self.poison.borrow_mut().unregister(event.unit_id);
    }
}
