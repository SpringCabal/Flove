use flove_shared::command;
use spring::Float3;
use spring_addons::AddonContext;
use springcabal::message::{self, Fields};

use crate::state::Global;
use crate::story;

const UPGRADE_PICK_RADIUS: f32 = 90.0;

pub fn handle_command(ctx: &AddonContext<'_, Global>, msg: &str) {
    let spring = ctx.spring();
    let global = ctx.global();
    let frame = spring.game().current_frame().unwrap_or(0);
    let Some((name, mut fields)) = message::command(msg) else {
        return;
    };

    match name {
        command::RESTART => {
            crate::round::restart(spring, fields.i32());
        }
        #[cfg(feature = "e2e")]
        "add_mana" => {
            if let Some(amount) = fields.i32() {
                *global.mana.access_mut() += amount;
            }
        }
        command::ZAP => {
            if let Some(target) = position(&mut fields) {
                let total_power = global.trees.access().total_power();
                let _ = global
                    .spire
                    .access_mut()
                    .fire_zap(spring, target, frame, total_power);
            }
        }
        command::RMB => {
            let field_count = fields.clone().take(4).count();
            let (tree, target) = spire_target(spring, global, false, field_count, fields);
            spire_right_click(spring, global, frame, tree, target);
        }
        #[cfg(feature = "e2e")]
        "upgrade" | "field_of_flowers" | "flower" => {
            let field_count = fields.clone().take(4).count();
            let (tree, target) =
                spire_target(spring, global, name == "upgrade", field_count, fields);
            spire_right_click(spring, global, frame, tree, target);
        }
        command::STORY => story::next(spring, global, frame),
        command::SKIP_TUTORIAL => {
            story::with_world(global, frame, |story, world| {
                story.skip_tutorial(spring, world)
            });
        }
        command::DIFFICULTY => {
            if let Some(difficulty) = fields.i32() {
                global.mushrooms.access_mut().set_difficulty(difficulty);
            }
        }
        _ => {}
    }
}

pub fn is_spire_command(msg: &str) -> bool {
    message::command(msg).is_some_and(|(name, _)| matches!(name, command::ZAP | command::RMB))
}

fn position(fields: &mut Fields<'_>) -> Option<Float3> {
    let (x, y, z) = fields.xyz()?;
    Some(Float3::new(x, y, z))
}

fn spire_target(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &Global,
    upgrade_near_ground: bool,
    field_count: usize,
    mut fields: Fields<'_>,
) -> (Option<i32>, Option<Float3>) {
    let trees = global.trees.access();
    let upgradable = |unit: i32| trees.is_upgradable(unit).then_some(unit);

    match field_count {
        4 => {
            let (Some(unit), Some(pos)) = (fields.i32(), position(&mut fields)) else {
                return (None, None);
            };
            (upgradable(unit), Some(pos))
        }
        3 => {
            let Some(pos) = position(&mut fields) else {
                return (None, None);
            };
            let tree = if upgrade_near_ground {
                trees
                    .find_tree_near(spring, pos.x, pos.z, UPGRADE_PICK_RADIUS)
                    .and_then(upgradable)
            } else {
                None
            };
            (tree, Some(pos))
        }
        1 | 2 => {
            let Some(unit) = fields.i32() else {
                return (None, None);
            };
            let tree = upgradable(unit).or_else(|| {
                let pos = spring
                    .units_info()
                    .get_unit_position(
                        unit,
                        spring::units_info::GetUnitPositionOptions {
                            mid_pos: false,
                            aim_pos: false,
                        },
                    )
                    .ok()?;
                trees
                    .find_tree_near(spring, pos.x, pos.z, UPGRADE_PICK_RADIUS)
                    .and_then(upgradable)
            });
            (tree, None)
        }
        _ => (None, None),
    }
}

fn spire_right_click(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &Global,
    frame: i32,
    tree: Option<i32>,
    target: Option<Float3>,
) {
    if let Some(tree_id) = tree {
        let _ = global.spire.access_mut().fire_upgrade_shot(
            spring,
            tree_id,
            frame,
            &mut global.mana.access_mut(),
        );
    } else if let Some(target) = target {
        global.spire.access_mut().fire_flower_shot(
            spring,
            target,
            frame,
            &mut global.mana.access_mut(),
        );
    }
}
