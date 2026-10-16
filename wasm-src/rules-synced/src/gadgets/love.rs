pub mod behavior;
use spring_addons::rules::{Gadget, UnitPreDamagedEvent};

use crate::state::Global;

#[derive(Default)]
pub struct LoveGadget;

impl LoveGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for LoveGadget {
    fn name(&self) -> &'static str {
        "Love"
    }

    fn projectile_destroyed(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        projectile_id: i32,
        _owner_id: i32,
        weapon_def_id: i32,
    ) {
        if weapon_def_id == crate::definitions::defs().zap_weapon {
            behavior::zap_landed(projectile_id);
        }
    }

    fn unit_pre_damaged(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        let global = ctx.global();
        let trees_take_no_damage = global.story.access().trees_take_no_damage;
        Some(behavior::handle_pre_damage(
            event.unit_id,
            event.unit_def_id,
            event.damage,
            event.weapon_def_id,
            trees_take_no_damage,
        ))
    }
}
