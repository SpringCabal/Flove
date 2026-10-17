use alloc::format;
use spring_addons::AddonContext;

use super::world::living_trees;
use super::{Step, TestTracker, destroy, living, published, send, spawn};
use crate::definitions::defs;
use crate::mushrooms::{SHROOM_EVENT_KING_DEFEATED, SHROOM_EVENT_KING_SPAWNED};
use crate::state::Global;
use crate::story::{STAGE_ARMY_APPROACHES, STAGE_FREE_PLAY, STAGE_INTRO, STAGE_VICTORY};
use flove_shared::command;

const KING_SPOT: (f32, f32) = (6844.0, 7028.0);

const ROUND_SETUP_FRAMES: i32 = 2;

pub(super) fn skip_round(
    _t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    _frame: i32,
) -> Step {
    send(ctx, "restart|1");
    Step::Done
}

pub(super) fn skip(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    if t.elapsed(frame) < ROUND_SETUP_FRAMES {
        return Step::Wait;
    }
    let global = ctx.global();
    let trees_before = living_trees(spring).len();
    let mushrooms_before = living(spring, |def| defs(spring).is_mushroom(def));
    send(ctx, command::SKIP_TUTORIAL);
    {
        let story = global.story.access();
        t.check(
            spring,
            story.stage == STAGE_FREE_PLAY && story.tutorial_skipped,
            "Tutorial skip sets story stage to 0 and marks tutorial skipped",
        );
    }
    t.checks.after_commands(move |checks, ctx| {
let spring = ctx.spring();
        let global = ctx.global();
        let mana = *global.mana.access();
        let grown = living_trees(spring, ).len() - trees_before;
        checks.check(spring,
            mana == 30 && grown == 3,
            format!(
                "Tutorial skip grants 30 mana and grows 3 trees (mana: {mana}, trees: +{grown})"
            ),
        );
        let first_wave_spawned = global
            .mushrooms
            .access()
            .waves
            .first()
            .is_some_and(|wave| wave.spawned);
        let mushrooms = living(spring, |def| defs(spring).is_mushroom(def));
        checks.check(spring,
            first_wave_spawned && mushrooms == mushrooms_before,
            format!(
                "Tutorial skip counts the first wave as spawned without spawning it ({mushrooms_before} -> {mushrooms} mushrooms)"
            ),
        );
    });
    Step::Done
}

pub(super) fn king_arrives(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    frame: i32,
) -> Step {
    let spring = ctx.spring();
    if t.elapsed(frame) == 0 {
        t.king = spawn(spring, defs(spring).king_shroom, KING_SPOT);
    }
    if t.elapsed(frame) < 2 {
        return Step::Wait;
    }
    let global = ctx.global();
    let event = global.mushrooms.access().shroom_event;
    t.check(
        spring,
        event == SHROOM_EVENT_KING_SPAWNED
            && published(spring, global, "story") == STAGE_ARMY_APPROACHES,
        format!("A king's UnitCreated raises the king event and its story (event: {event})"),
    );
    send(ctx, command::STORY);
    let stage = global.story.access().stage;
    let waves_scheduled = global.mushrooms.access().start_spawn_frame.is_some();
    t.check(
        spring,
        stage == STAGE_FREE_PLAY && waves_scheduled,
        format!("Continue on the king's story keeps free play (stage: {stage})"),
    );
    destroy(spring, t.king);
    Step::Done
}

pub(super) fn king_defeated(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    frame: i32,
) -> Step {
    let spring = ctx.spring();
    if t.elapsed(frame) < 2 {
        return Step::Wait;
    }
    let global = ctx.global();
    let event = global.mushrooms.access().shroom_event;
    let game_over = global.victory.access().game_over;
    t.check(
        spring,
        event == SHROOM_EVENT_KING_DEFEATED
            && !game_over
            && published(spring, global, "story") == STAGE_VICTORY,
        format!("The king's death shows the victory story before the game ends (event: {event})"),
    );
    send(ctx, command::STORY);
    let victory = global.victory.access();
    t.check(
        spring,
        victory.game_over && victory.won,
        "Continue on the victory story wins the game",
    );
    Step::Done
}

pub(super) fn victory_published(
    t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    frame: i32,
) -> Step {
    let spring = ctx.spring();
    if t.elapsed(frame) < 2 {
        return Step::Wait;
    }
    let global = ctx.global();
    t.check(
        spring,
        published(spring, global, "game_over") == 1 && published(spring, global, "won") == 1,
        "The UI is told of the win",
    );
    send(ctx, "restart|2");
    Step::Done
}

pub(super) fn restart(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    if t.elapsed(frame) < ROUND_SETUP_FRAMES {
        return Step::Wait;
    }
    let global = ctx.global();
    {
        let victory = global.victory.access();
        t.check(
            spring,
            !victory.game_over && !victory.won,
            "In-game restart successfully clears game_over and won flags",
        );
    }
    t.check(
        spring,
        global.story.access().stage == STAGE_INTRO,
        "In-game restart resets story stage to 1",
    );
    let (event, difficulty) = {
        let mushrooms = global.mushrooms.access();
        (mushrooms.shroom_event, mushrooms.difficulty)
    };
    t.check(
        spring,
        event == 0,
        "In-game restart resets shroom_event to 0",
    );
    t.check(
        spring,
        difficulty == 2,
        "In-game restart applies chosen difficulty",
    );
    t.check(
        spring,
        global.spire.access().unit_id > 0,
        "In-game restart spawns a valid new spire",
    );
    let mushrooms_left = living(spring, |def| defs(spring).is_mushroom(def));
    t.check(
        spring,
        mushrooms_left == 0,
        format!("In-game restart leaves no mushrooms behind (found: {mushrooms_left})"),
    );
    Step::Done
}

pub(super) fn destroy_spire(
    _t: &mut TestTracker,
    ctx: &AddonContext<'_, Global>,
    _frame: i32,
) -> Step {
    let spring = ctx.spring();
    let spire = ctx.global().spire.access().unit_id;
    destroy(spring, spire);
    Step::Done
}

pub(super) fn defeat(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    if t.elapsed(frame) < 2 {
        return Step::Wait;
    }
    let global = ctx.global();
    {
        let victory = global.victory.access();
        t.check(
            spring,
            victory.game_over && !victory.won && global.spire.access().unit_id < 0,
            "Losing the spire loses the game",
        );
    }
    t.check(
        spring,
        published(spring, global, "game_over") == 1 && published(spring, global, "won") == 0,
        "The UI hears of the defeat from a surviving unit",
    );
    send(ctx, "restart|2");
    Step::Done
}

pub(super) fn fresh_round(t: &mut TestTracker, ctx: &AddonContext<'_, Global>, frame: i32) -> Step {
    let spring = ctx.spring();
    if t.elapsed(frame) < ROUND_SETUP_FRAMES {
        return Step::Wait;
    }
    let global = ctx.global();
    t.check(
        spring,
        !global.victory.access().game_over
            && global.spire.access().unit_id > 0
            && global.round.get() == 4,
        "A restart after a defeat starts a fresh round",
    );
    Step::Done
}
