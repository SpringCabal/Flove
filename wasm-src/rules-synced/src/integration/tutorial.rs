use alloc::format;
use spring_addons::AddonContext;

use super::world::{living_trees, set_health_fraction};
use super::{Step, TestTracker, check_skip_refused, destroy, ground_position, living_units, send};
use crate::definitions::defs;
use crate::state::Global;
use crate::story::{
    STAGE_FIRST_WAVE, STAGE_FLOWER_FIELDS, STAGE_FREE_PLAY, STAGE_HEAL, STAGE_REPEL, STAGE_UPGRADE,
};
use flove_shared::command;

const UPGRADE_SHOT_RELOAD_FRAMES: i32 = 15;

fn stage(global: &Global) -> i32 {
    global.story.access().stage
}

pub(super) fn intro(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, _frame: i32) -> Step {
    let spring = ctx.spring();
    destroy(spring, t.mushroom);
    send(ctx, command::STORY);
    t.checks.after_commands(|checks, ctx| {
let spring = ctx.spring();
        let global = ctx.global();
        let (stage, protected) = {
            let story = global.story.access();
            (story.stage, story.trees_take_no_damage)
        };
        let story_mushrooms = global.mushrooms.access().story_mushrooms.len();
        checks.check(spring,
            stage == STAGE_REPEL && story_mushrooms == 5 && protected,
            format!("The intro's Next starts Repel with 5 mushrooms (stage {stage}, {story_mushrooms} mushrooms)"),
        );
    });
    check_skip_refused(spring, t, "on Repel");
    Step::Done
}

pub(super) fn repel(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    if t.elapsed(frame) == 0 {
        let story_mushrooms = global.mushrooms.access().story_mushrooms.clone();
        for mushroom in story_mushrooms {
            destroy(spring, mushroom);
        }
    }
    if stage(global) != STAGE_HEAL {
        return Step::Wait;
    }
    t.check(
        spring,
        true,
        "Repelling the story mushrooms moves on to Heal",
    );
    let wounded = living_trees(spring).into_iter().any(|tree| {
        spring
            .units_info()
            .get_unit_health(tree)
            .is_ok_and(|health| health.health < 0.96 * health.max_health)
    });
    t.check(
        spring,
        wounded && !global.story.access().trees_take_no_damage,
        "Heal wounds the trees and lets them take damage again",
    );
    check_skip_refused(spring, t, "on Heal");
    Step::Done
}

pub(super) fn heal(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    if t.elapsed(frame) == 0 {
        for tree in living_trees(spring) {
            set_health_fraction(spring, tree, 1.0);
        }
    }
    if stage(global) != STAGE_FIRST_WAVE {
        return Step::Wait;
    }
    t.check(spring, true, "Healing the trees moves on to the first wave");
    t.checks.after_commands(|checks, ctx| {
        let spring = ctx.spring();
        let spawned = ctx
            .global()
            .mushrooms
            .access()
            .waves
            .first()
            .is_some_and(|wave| wave.spawned);
        let mushrooms = living_units(spring, |def| defs(spring).is_mushroom(def)).len();
        checks.check(
            spring,
            spawned && mushrooms > 0,
            format!("The first wave spawns ({mushrooms} mushrooms)"),
        );
    });
    check_skip_refused(spring, t, "during the first wave");
    Step::Done
}

pub(super) fn first_wave(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    if t.elapsed(frame) == 0 {
        t.count = living_trees(spring).len();
        for mushroom in living_units(spring, |def| defs(spring).is_mushroom(def)) {
            destroy(spring, mushroom);
        }
    }
    if stage(global) != STAGE_UPGRADE {
        return Step::Wait;
    }
    let mana = *global.mana.access();
    t.check(
        spring,
        mana == 100,
        format!("Beating the first wave moves on to Upgrade with 100 mana (mana {mana})"),
    );
    let trees_before = t.count;
    t.checks.after_commands(move |checks, ctx| {
        let spring = ctx.spring();
        let grown = living_trees(spring).len() - trees_before;
        checks.check(
            spring,
            grown == 3,
            format!("Upgrade grows 3 trees (grown: {grown})"),
        );
    });
    check_skip_refused(spring, t, "on Upgrade");
    Step::Done
}

pub(super) fn upgrade_stage(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    _frame: i32,
) -> Step {
    let spring = ctx.spring();
    let trees = ctx.global().trees.access();
    if trees.trees.len() != living_trees(spring).len() {
        return Step::Wait;
    }
    t.tree = trees
        .trees
        .iter()
        .find(|tree| tree.level == 1)
        .map_or(-1, |tree| tree.unit_id);
    t.power = trees.total_power();
    t.shots = 0;
    t.count = 0;
    Step::Done
}

pub(super) fn upgrade_shots(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    frame: i32,
) -> Step {
    let spring = ctx.spring();
    let global = ctx.global();
    let promoted = global
        .trees
        .access()
        .trees
        .iter()
        .any(|tree| tree.level == 2);
    if promoted {
        t.check(
            spring,
            t.shots == 3,
            format!(
                "Three upgrade shots grow a tree to level 2 ({} shots)",
                t.shots
            ),
        );
        let power = global.trees.access().total_power();
        t.check(
            spring,
            power == t.power + 12,
            format!(
                "Total power increased from {} to {power} (L1:3 -> L2:15)",
                t.power
            ),
        );
        return Step::Done;
    }
    let charge = spring
        .rules_params()
        .get_unit_rules_param_f32(t.tree, "upgradeProgress")
        .map_or(0, |charge| charge as i32);
    if charge as usize > t.count {
        t.count = charge as usize;
        t.check(
            spring,
            charge == t.shots,
            format!("Upgrade shot {} charges the tree to {charge}/3", t.shots),
        );
    }
    let landed = charge == t.shots;
    if t.shots < 3 && landed && frame - t.shot_frame >= UPGRADE_SHOT_RELOAD_FRAMES {
        let Some(pos) = ground_position(spring, t.tree) else {
            return Step::Wait;
        };
        let mana_before = *global.mana.access();
        send(ctx, &format!("rmb|{}|{}", t.tree, super::xyz(pos)));
        let mana = *global.mana.access();
        if t.shots == 0 {
            t.check(
                spring,
                mana == mana_before - 10,
                format!("An upgrade shot costs 10 mana ({mana_before} -> {mana})"),
            );
        }
        t.shots += 1;
        t.shot_frame = frame;
    }
    Step::Wait
}

pub(super) fn flower_fields(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    _frame: i32,
) -> Step {
    let spring = ctx.spring();
    if stage(ctx.global()) != STAGE_FLOWER_FIELDS {
        return Step::Wait;
    }
    t.check(spring, true, "Upgrading a tree moves on to Flower Fields");
    check_skip_refused(spring, t, "on Flower Fields");
    Step::Done
}

pub(super) fn free_play(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, _frame: i32) -> Step {
    let spring = ctx.spring();
    send(ctx, command::STORY);
    let global = ctx.global();
    let (stage, skipped) = {
        let story = global.story.access();
        (story.stage, story.tutorial_skipped)
    };
    let waves_scheduled = global.mushrooms.access().start_spawn_frame.is_some();
    t.check(
        spring,
        stage == STAGE_FREE_PLAY && waves_scheduled && !skipped,
        format!(
            "Continue on Flower Fields ends the tutorial and schedules the waves (stage {stage})"
        ),
    );
    check_skip_refused(spring, t, "in free play");
    Step::Done
}
