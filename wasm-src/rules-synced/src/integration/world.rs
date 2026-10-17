use alloc::format;
use alloc::vec::Vec;
use spring::Float3;
use spring_addons::AddonContext;

use super::{Step, TestTracker, distance_2d, ground_position, living_units, send, spawn, xyz};
use crate::definitions::defs;
use crate::state::Global;
use crate::trees::tree_config;

const LEVEL1_TREE_POWER: i32 = 3;

const OPEN_GROUND: (f32, f32) = (4300.0, 6000.0);

const FLOWER_SHOT_RELOAD_FRAMES: i32 = 11;

pub(super) fn world(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    if t.elapsed(frame) < 5 {
        return Step::Wait;
    }
    let global = ctx.global();
    let spire_id = global.spire.access().unit_id;
    let trees = global.trees.access();
    let defs = defs(spring);
    t.check(
        spring,
        global.round.get() == 1,
        "World successfully initialized",
    );
    t.check(spring, spire_id > 0, "Spire unit ID is valid");
    let flare = crate::spire::get_spire_flare_position(spring, spire_id);
    let world_space = match (flare, ground_position(spring, spire_id)) {
        (Some(flare), Some(base)) => {
            (flare.x - base.x).abs() < 100.0 && (flare.z - base.z).abs() < 100.0 && flare.y > base.y
        }
        _ => false,
    };
    t.check(
        spring,
        world_space,
        "Spire flare is above the world-space spire, not at model-local coordinates",
    );
    let expected_trees = crate::spawn_data::INITIAL_UNITS
        .iter()
        .filter(|point| point.marker == crate::spawn_data::Marker::Base)
        .count();
    let tree_count = trees.trees.len();
    t.check(spring,
        tree_count == expected_trees,
        format!(
            "Tree manager holds one tree per base spawn point (count: {tree_count}, expected: {expected_trees})"
        ),
    );
    let power = trees.total_power();
    let expected_power = (tree_count as i32) * LEVEL1_TREE_POWER;
    t.check(
        spring,
        power == expected_power,
        format!(
            "Total tree power matches level-1 trees (power: {power}, expected: {expected_power})"
        ),
    );
    t.check(
        spring,
        defs.tree_level1 > 0 && defs.tree_level2 > 0 && defs.tree_level3 > 0,
        "Tree unit defs (L1, L2, L3) are valid",
    );
    t.check(
        spring,
        defs.normal_mushroom > 0 && defs.king_shroom > 0 && defs.big_mushroom > 0,
        "Mushroom unit defs (Normal, King, Big) are valid",
    );
    t.check(
        spring,
        defs.zap_weapon >= 0 && defs.flower_shot_weapon >= 0,
        "Weapon definitions (Zap, FlowerShot) are registered",
    );
    Step::Done
}

pub(super) fn zap(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    let Some(tree) = global.trees.access().trees.first().map(|tree| tree.unit_id) else {
        t.check(
            spring,
            false,
            "Spire fired a zap through the zap command (no tree)",
        );
        return Step::Done;
    };
    set_health_fraction(spring, tree, 0.5);
    t.tree = tree;
    let target = spring
        .units_info()
        .get_unit_position(
            tree,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: true,
                aim_pos: false,
            },
        )
        .unwrap_or_default();
    send(ctx, &format!("zap|{}", xyz(target)));
    t.shot_frame = frame;
    t.check(
        spring,
        global.spire.access().last_zap_frame == frame,
        "Spire fired a zap through the zap command",
    );
    Step::Done
}

pub(super) fn zap_lands(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    if t.elapsed(frame) == 0 {
        send(ctx, "zap|2500|0|2500");
        let last = global.spire.access().last_zap_frame;
        t.check(
            spring,
            last == t.shot_frame,
            format!("Spire cooldown refuses a second zap on the next frame (last: {last})"),
        );
    }
    let Some(fraction) = health_fraction(spring, t.tree) else {
        return Step::Wait;
    };
    if fraction <= 0.5 {
        return Step::Wait;
    }
    t.check(
        spring,
        true,
        format!(
            "The zap heals the tree it hits (health {:.0}%)",
            fraction * 100.0
        ),
    );
    Step::Done
}

pub(super) fn growth(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    let delays: Vec<i32> = (0..200)
        .map(|_| {
            crate::gadgets::growth::behavior::GrowthManager::starting_at(spring, frame).due_frame()
                - frame
        })
        .collect();
    let (min, max) = (delays.iter().min(), delays.iter().max());
    t.check(
        spring,
        min.is_some_and(|&min| (120..135).contains(&min))
            && max.is_some_and(|&max| (165..=180).contains(&max)),
        format!("Trees grow every 120 to 180 frames (sampled {min:?} to {max:?})"),
    );
    t.trees_before = ctx.global().trees.access().get_tree_ids();
    let spire = ctx.global().spire.access().unit_id;
    crate::gadgets::growth::behavior::spawn_tree(spring, spire);
    Step::Done
}

pub(super) fn grown_tree(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, _frame: i32) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    let spire_id = global.spire.access().unit_id;
    let ids = global.trees.access().get_tree_ids();
    let new_trees: Vec<i32> = ids
        .iter()
        .copied()
        .filter(|id| !t.trees_before.contains(id))
        .collect();
    if new_trees.is_empty() {
        return Step::Wait;
    }
    let mut unique = ids.clone();
    unique.sort_unstable();
    unique.dedup();
    t.check(
        spring,
        new_trees.len() == 1 && unique.len() == ids.len(),
        format!(
            "Growth adds exactly one tree, registered once (added: {})",
            new_trees.len()
        ),
    );
    let min_distance = crate::gadgets::growth::behavior::MIN_TREE_DISTANCE - 12.0;
    let nearest = t
        .trees_before
        .iter()
        .copied()
        .chain([spire_id])
        .filter_map(|unit| distance_2d(spring, unit, new_trees[0]))
        .fold(f32::MAX, f32::min);
    t.check(
        spring,
        nearest >= min_distance,
        format!("The grown tree keeps its distance to trees and the spire (nearest: {nearest:.0})"),
    );
    Step::Done
}

pub(super) fn wisps(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    let (l1, l2, l3) = (tree_config(1), tree_config(2), tree_config(3));
    t.check(
        spring,
        l1.max_wisps == 3 && l2.max_wisps == 7 && l3.max_wisps == 12,
        "Tree max wisp capacities correct across levels (L1:3, L2:7, L3:12)",
    );
    let mut trees = global.trees.access_mut();
    let mut mana = global.mana.access_mut();
    let mana_before = *mana;
    let wisps_before: usize = trees.trees.iter().map(|tree| tree.wisps.len()).sum();
    let first = frame + 1000;
    trees.update_wisps(spring, first, &mut mana);
    let wisps_after: usize = trees.trees.iter().map(|tree| tree.wisps.len()).sum();
    let tree_count = trees.trees.len();
    let published = trees.trees.iter().all(|tree| {
        spring
            .rules_params()
            .get_unit_rules_param_f32(tree.unit_id, "wispCount")
            .is_ok_and(|count| count as usize == tree.wisps.len())
    });
    t.check(spring,
        wisps_before == 0 && wisps_after == tree_count && published,
        format!("Every tree grows a wisp and publishes its count ({wisps_after} wisps, {tree_count} trees)"),
    );
    for stage in 1..=3 {
        trees.update_wisps(
            spring,
            first + stage * crate::trees::WISP_STAGE_DURATION,
            &mut mana,
        );
    }
    t.check(
        spring,
        *mana == mana_before + tree_count as i32,
        format!(
            "Each first wisp turned into one mana after three stages (mana {mana_before} -> {})",
            *mana
        ),
    );
    Step::Done
}

pub(super) fn mushroom_ai(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    _frame: i32,
) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    if t.mushroom < 0 {
        let spire = ground_position(spring, global.spire.access().unit_id).unwrap_or_default();
        t.mushroom = spawn(
            spring,
            defs(spring).normal_mushroom,
            (spire.x + 600.0, spire.z + 600.0),
        );
        return Step::Wait;
    }
    let mushroom = t.mushroom;
    let orders = spring
        .units_commands()
        .get_unit_commands(mushroom, 2)
        .map(|commands| commands.len())
        .unwrap_or(0);
    if orders == 0 {
        return Step::Wait;
    }
    let mushrooms = global.mushrooms.access();
    let wave_count = mushrooms.waves.len();
    t.check(
        spring,
        wave_count == 15,
        format!("Mushroom manager has all 15 waves configured (found: {wave_count})"),
    );
    t.check(
        spring,
        mushrooms.waves[13].spec.spawn_king && mushrooms.waves[14].spec.repeat_seconds.is_some(),
        "Wave schedule contains King Mushroom boss wave and endgame repeating wave",
    );
    t.check(
        spring,
        mushrooms.ai_units.iter().any(|ai| ai.unit_id == mushroom),
        "A new mushroom joins the AI through UnitCreated",
    );
    t.check(
        spring,
        orders > 0,
        format!("The mushroom AI gives an idle mushroom an order ({orders} queued)"),
    );
    Step::Done
}

pub(super) fn flower_shots(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    frame: i32,
) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    *global.mana.access_mut() = 20;
    send(ctx, &format!("rmb|{}", xyz(open_ground(spring,))));
    t.shot_frame = frame;
    t.check(
        spring,
        global.spire.access().last_flower_shot_frame == frame,
        "Spire fired a flower shot through the rmb command",
    );
    let mana = *global.mana.access();
    t.check(
        spring,
        mana == 10,
        format!("Flower shot correctly deducted 10 mana (remaining: {mana})"),
    );
    Step::Done
}

pub(super) fn flower_shot_reload(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    frame: i32,
) -> Step {
    let spring = ctx.spring();
    if frame - t.shot_frame < FLOWER_SHOT_RELOAD_FRAMES {
        return Step::Wait;
    }
    let global = ctx.global();
    *global.mana.access_mut() = 0;
    send(ctx, &format!("rmb|{}", xyz(open_ground(spring,))));
    t.check(
        spring,
        global.spire.access().last_flower_shot_frame == t.shot_frame,
        "Flower shot correctly rejected when mana is insufficient",
    );
    *global.mana.access_mut() = 10;
    let at_mushroom = ground_position(spring, t.mushroom).unwrap_or_default();
    send(ctx, &format!("rmb|{}", xyz(at_mushroom)));
    t.check(
        spring,
        global.spire.access().last_flower_shot_frame == frame,
        "A second flower shot fires at the test mushroom",
    );
    Step::Done
}

pub(super) fn flower_field(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    _frame: i32,
) -> Step {
    let spring = ctx.spring();
    if ctx.global().flowers.access().centers.len() < 2 {
        return Step::Wait;
    }
    let (x, z) = OPEN_GROUND;
    let bloomed = spring
        .units_query()
        .get_units_in_cylinder(
            x,
            z,
            crate::flowers::FLOWER_FIELD_RADIUS + 50.0,
            crate::definitions::ALL_UNITS,
        )
        .unwrap_or_default()
        .into_iter()
        .filter(|&unit| {
            spring
                .units_info()
                .get_unit_def_id(unit)
                .is_ok_and(|def| defs(spring).is_flower(def))
        })
        .count();
    t.check(
        spring,
        (35..=60).contains(&bloomed),
        format!("A flower field blooms a grid of flowers where its shot lands ({bloomed})"),
    );
    Step::Done
}

pub(super) fn flower_slow(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    _frame: i32,
) -> Step {
    let spring = ctx.spring();
    if !ctx
        .global()
        .flowers
        .access()
        .slowed_units
        .contains(&t.mushroom)
    {
        return Step::Wait;
    }
    t.check(
        spring,
        true,
        "A flower field slows the mushroom standing in it",
    );
    let def = defs(spring).normal_mushroom;
    let speed = spring.unit_defs().get_unit_def_speed(def).unwrap_or(0.0) * 0.05;
    let turn_rate = spring
        .unit_defs()
        .get_unit_def_param_float(def, "turnRate")
        .unwrap_or(0.0);
    let slowed = spring
        .move_ctrl()
        .get_unit_move_type_data(t.mushroom)
        .is_ok_and(|data| {
            (data.max_speed - speed * 0.25).abs() < 0.01
                && (data.turn_rate - turn_rate * 0.25).abs() < 0.5
                && data.max_reverse_speed == 0.0
        });
    t.check(
        spring,
        slowed,
        "The slowed mushroom moves and turns at a quarter rate and can't reverse",
    );
    Step::Done
}

pub(super) fn poison(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, _frame: i32) -> Step {
    let spring = ctx.spring();
    let defs = defs(spring);
    let trees_protected = ctx.global().story.access().trees_take_no_damage;
    let damage_to = |unit: i32| {
        let def = spring.units_info().get_unit_def_id(unit).unwrap_or(-1);
        crate::gadgets::love::behavior::handle_pre_damage(
            spring,
            unit,
            def,
            100.0,
            defs.poison_weapon,
            trees_protected,
        )
        .new_damage
    };
    let flower = living_units(spring, |def| defs.is_flower(def))
        .first()
        .copied();
    let (tree, mushroom) = (t.tree, t.mushroom);
    let (to_flower, to_tree, to_mushroom) =
        (flower.map(damage_to), damage_to(tree), damage_to(mushroom));
    t.check(spring,
        to_flower.is_some_and(|damage| damage >= 1000.0) && to_tree == 0.0 && to_mushroom == 0.0,
        format!(
            "Poison kills flowers and harms nothing else (flower {to_flower:?}, tree {to_tree}, mushroom {to_mushroom})"
        ),
    );
    Step::Done
}

fn open_ground(spring: &spring::Spring<'_, spring::RulesSynced>) -> Float3 {
    let (x, z) = OPEN_GROUND;
    Float3::new(
        x,
        spring.terrain().get_ground_height(x, z).unwrap_or(0.0),
        z,
    )
}

fn health_fraction(spring: &spring::Spring<'_, spring::RulesSynced>, unit: i32) -> Option<f32> {
    let health = spring.units_info().get_unit_health(unit).ok()?;
    (health.max_health > 0.0).then(|| health.health / health.max_health)
}

pub(super) fn set_health_fraction(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit: i32,
    fraction: f32,
) {
    let Ok(health) = spring.units_info().get_unit_health(unit) else {
        return;
    };
    let _ = spring.unit_control().set_unit_health(
        unit,
        spring::unit_control::UnitHealthValue {
            health: health.max_health * fraction,
            capture: 0.0,
            paralyze: 0.0,
            build: 1.0,
            use_amounts: false,
        },
    );
}

pub(super) fn living_trees(spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<i32> {
    living_units(spring, |def| defs(spring).is_tree(def))
}
