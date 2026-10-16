use spring::types::Float3;
use spring_addons::rules::Gadget;

use crate::projectiles::RoundProjectiles;
use crate::state::Global;

const UPGRADE_IMPACT_RADIUS: f32 = 100.0;

#[derive(Default)]
pub struct UpgradeShotsGadget {
    shots: RoundProjectiles,
}

impl UpgradeShotsGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for UpgradeShotsGadget {
    fn name(&self) -> &'static str {
        "UpgradeShots"
    }

    fn projectile_created(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        projectile_id: i32,
        _owner_id: i32,
        weapon_def_id: i32,
    ) {
        if weapon_def_id == crate::definitions::defs().upgrade_shot_weapon {
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
        let global = ctx.global();
        if weapon_def_id != crate::definitions::defs().upgrade_shot_weapon
            || !self.shots.destroyed(global.round.get(), projectile_id)
        {
            return;
        }
        let Ok(position) = spring::projectiles::get_projectile_position(projectile_id) else {
            return;
        };
        let frame = spring::game::current_frame();
        let candidates = global
            .trees
            .access()
            .trees_near_sorted(position, UPGRADE_IMPACT_RADIUS);
        crate::diagnostics::trace!(
            "[upgrade] shot landed  f={frame} shot={projectile_id} at=({:.0},{:.0},{:.0}) trees_in_range={candidates:?}",
            position.x,
            position.y,
            position.z,
        );
        for tree_id in candidates {
            if global
                .trees
                .access_mut()
                .add_upgrade_progress(tree_id, frame, projectile_id)
            {
                let _ = spring::sound::play_sound_file(
                    "sounds/wind+leaves.wav",
                    10.0,
                    position,
                    Float3::ZERO,
                    0,
                );
                return;
            }
        }
        crate::diagnostics::trace!("[upgrade] NO-TREE f={frame} pro={projectile_id}");
    }
}
