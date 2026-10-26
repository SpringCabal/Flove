use alloc::vec::Vec;

use spring::Float3;

#[derive(Debug, Default)]
pub struct PoisonManager {
    pub poison_mushrooms: Vec<i32>,
}

impl PoisonManager {
    pub fn register(&mut self, unit_id: i32) {
        if !self.poison_mushrooms.contains(&unit_id) {
            self.poison_mushrooms.push(unit_id);
        }
    }

    pub fn unregister(&mut self, unit_id: i32) {
        if let Some(pos) = self.poison_mushrooms.iter().position(|&id| id == unit_id) {
            self.poison_mushrooms.swap_remove(pos);
        }
    }
}

pub fn spray(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    poison_mushrooms: &[i32],
    frame: i32,
) {
    let defs = crate::definitions::defs(spring);
    if frame % 30 != 0 || defs.poison_weapon <= 0 {
        return;
    }
    for &unit_id in poison_mushrooms {
        let Ok(pos) = spring.units_info().get_unit_position(
            unit_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        ) else {
            continue;
        };

        crate::with_addon_context(|ctx| {
            ctx.commands().run(move |spring| {
                crate::diagnostics::check(
                    spring,
                    spring.projectile_control().spawn_projectile(
                        defs.poison_weapon,
                        &spring::projectile_control::NativeProjectileParams {
                            pos,
                            end: pos,
                            speed: Float3::ZERO,
                            owner: unit_id,
                            team: -1,
                            weapon_num: 0,
                            ttl: 1.0,
                            max_range: 2.0,
                            up_time: -1.0,
                            end_alpha: 1.0,
                            ..Default::default()
                        },
                    ),
                    || alloc::format!("poison cloud of mushroom {unit_id}"),
                );
            })
        });
    }
}
