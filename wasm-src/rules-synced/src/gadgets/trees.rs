use spring::commands;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::state::Global;

#[derive(Default)]
pub struct TreesGadget;

impl TreesGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for TreesGadget {
    fn name(&self) -> &'static str {
        "Trees"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        global
            .trees
            .access_mut()
            .update_wisps(frame, &mut global.mana.access_mut());
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        unit: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        let Some(level) = crate::definitions::defs().tree_level(def) else {
            return;
        };
        commands::run(move || {
            crate::with_global(|global| {
                let frame = spring::game::current_frame();
                global.trees.access_mut().add_tree(unit, level, frame);
            });
        });
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        if !crate::definitions::defs().is_tree(event.unit_def_id) {
            return;
        }
        ctx.global().trees.access_mut().remove_tree(event.unit_id);
    }
}
