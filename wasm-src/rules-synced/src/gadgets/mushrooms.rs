use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::mushrooms::{SHROOM_EVENT_KING_DEFEATED, SHROOM_EVENT_KING_SPAWNED};
use crate::state::Global;
use crate::story::STAGE_FREE_PLAY;

#[derive(Default)]
pub struct MushroomsGadget;

impl MushroomsGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for MushroomsGadget {
    fn name(&self) -> &'static str {
        "Mushrooms"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let spring = ctx.spring();
        let global = ctx.global();
        if frame == 4
            && spring
                .game()
                .get_mod_option("skip_tutorial")
                .is_ok_and(|v| v.value == "1")
        {
            crate::story::with_world(global, frame, |story, world| {
                story.skip_tutorial(spring, world)
            });
            spring::log_notice!(spring, "FloveStartup", "tutorial skipped frame=4");
        }
        let spire_id = global.spire.access().unit_id;
        let free_play = global.story.access().stage == STAGE_FREE_PLAY;
        global.mushrooms.access_mut().update(
            spring,
            frame,
            spire_id,
            &global.trees.access(),
            free_play,
        );
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
        let defs = crate::definitions::defs(spring);
        if defs.is_mushroom(def) {
            crate::attributes::apply_base_movement(spring, unit);
            let mut mushrooms = ctx.global().mushrooms.access_mut();
            mushrooms.register_mushroom(unit, def, spring.game().current_frame().unwrap_or(0));
            if def == defs.king_shroom {
                mushrooms.shroom_event = SHROOM_EVENT_KING_SPAWNED;
            }
        }
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        let spring = ctx.spring();
        let defs = crate::definitions::defs(spring);
        if !defs.is_mushroom(event.unit_def_id) {
            return;
        }
        let mut mushrooms = ctx.global().mushrooms.access_mut();
        if event.unit_def_id == defs.king_shroom {
            mushrooms.shroom_event = SHROOM_EVENT_KING_DEFEATED;
        }
        mushrooms.unregister_mushroom(event.unit_id);
    }
}
