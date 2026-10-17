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
        let spring = ctx.spring();
        let global = ctx.global();
        global
            .trees
            .access_mut()
            .update_wisps(spring, frame, &mut global.mana.access_mut());
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
        let Some(level) = crate::definitions::defs(spring).tree_level(def) else {
            return;
        };
        crate::with_addon_context(|ctx| {
            ctx.commands().run(move |spring| {
                crate::with_global(|global| {
                    let frame = spring.game().current_frame().unwrap_or(0);
                    global
                        .trees
                        .access_mut()
                        .add_tree(spring, unit, level, frame);
                });
            })
        });
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        let spring = ctx.spring();
        if !crate::definitions::defs(spring).is_tree(event.unit_def_id) {
            return;
        }
        ctx.global().trees.access_mut().remove_tree(event.unit_id);
    }
}
