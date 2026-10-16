use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::projectiles::RoundProjectiles;
use crate::state::Global;

#[derive(Default)]
pub struct FlowersGadget {
    shots: RoundProjectiles,
}

impl FlowersGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for FlowersGadget {
    fn name(&self) -> &'static str {
        "Flowers"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        ctx.global().flowers.access_mut().update(frame);
    }

    fn projectile_created(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        projectile_id: i32,
        _owner_id: i32,
        weapon_def_id: i32,
    ) {
        if weapon_def_id == crate::definitions::defs().flower_shot_weapon {
            self.shots.created(ctx.global().round.get(), projectile_id);
        }
    }

    fn projectile_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        projectile_id: i32,
        _owner_id: i32,
        weapon_def_id: i32,
    ) {
        if weapon_def_id != crate::definitions::defs().flower_shot_weapon
            || !self
                .shots
                .destroyed(ctx.global().round.get(), projectile_id)
        {
            return;
        }
        let Ok(pos) = spring::projectiles::get_projectile_position(projectile_id) else {
            return;
        };
        let frame = spring::game::current_frame();
        ctx.global().flowers.access_mut().shot_landed(pos, frame);
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        ctx.global().flowers.access_mut().remove_unit(event.unit_id);
    }
}
