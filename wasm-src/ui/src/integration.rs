use alloc::format;
use spring_addons::event::KeyEvent;
use springcabal::check::Suite;

use crate::input;
use crate::state::Global;
use crate::story;
use flove_shared::{STAGE_INTRO, STAGE_REPEL};

const SECTION: &str = "flove-ui-test";

const PLAYER_TEAM_ID: i32 = 1;

const RESUME_INTERVAL: u32 = 15;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    WaitForTurn,
    Start,
    RevealIntro,
    SendNext,
    Repel,
    RepelPaused,
    CloseRepel,
    Resumed,
    Restarted,
    DocumentRecovered,
    Finished,
}

pub struct UiTestTracker {
    phase: Phase,
    updates: u32,
    since_resume: u32,
    document_loads: u32,
    suite: Suite,
}

impl Default for UiTestTracker {
    fn default() -> Self {
        Self {
            phase: Phase::WaitForTurn,
            updates: 0,
            since_resume: RESUME_INTERVAL,
            document_loads: 0,
            suite: Suite::new("[Flove-UI-Test]", "FLOVE UI INPUT TESTS", 10),
        }
    }
}

impl UiTestTracker {
    fn next(&mut self, phase: Phase) {
        self.phase = phase;
        self.updates = 0;
    }

    fn waited_too_long(
        &mut self,
        spring: &spring::Spring<'_, spring::Ui>,
        limit: u32,
        what: &str,
    ) -> bool {
        if self.updates < limit {
            return false;
        }
        self.suite
            .check(spring, false, format!("{what} within {limit} updates"));
        self.finish(spring);
        true
    }

    fn finish(&mut self, spring: &spring::Spring<'_, spring::Ui>) {
        self.suite.finish(spring);
        self.phase = Phase::Finished;
        springcabal::message::send_to_rules(spring, flove_shared::suites::UI_DONE);
    }
}

pub fn update(spring: &spring::Spring<'_, spring::Ui>, global: &Global, t: &mut UiTestTracker) {
    t.updates += 1;
    match t.phase {
        Phase::WaitForTurn => {
            keep_game_running(spring, t);
            if ui_turn(spring) {
                spring::log_info!(
                    spring,
                    SECTION,
                    "Starting Flove UI Input Simulation Tests..."
                );
                t.next(Phase::Start);
            }
        }
        Phase::Start if t.updates >= 5 => {
            input::action_start_difficulty(spring, global, 1);
            let (started, stage, skip) = {
                let game = global.game.access();
                (game.game_started, game.story_stage, game.skip_offered)
            };
            t.suite.check(
                spring,
                started
                    && story::modal_visible(global)
                    && stage == STAGE_INTRO
                    && skip
                    && !paused(spring),
                "Starting a game shows the intro, offering Skip, without pausing",
            );
            t.next(Phase::RevealIntro);
        }
        Phase::RevealIntro if t.updates >= 20 => {
            let sent = input::action_story_next(spring, global);
            let stage = global.game.access().story_stage;
            let (chars, total_chars) = story::typing_progress(global);
            t.suite.check(
                spring,
                !sent
                    && stage == STAGE_INTRO
                    && story::modal_visible(global)
                    && chars == total_chars,
                "first action_story_next() reveals the complete story text",
            );
            t.next(Phase::SendNext);
        }
        Phase::SendNext if t.updates >= 5 => {
            let sent = input::action_story_next(spring, global);
            let stage = global.game.access().story_stage;
            t.suite.check(
                spring,
                sent && stage == STAGE_INTRO && story::modal_visible(global),
                "second action_story_next() waits for the synced story update",
            );
            let handled = input::key_press(spring, global, &key_s());
            t.suite.check(
                spring,
                !handled && !global.game.access().skip_offered,
                "S does nothing once the intro's Next is pressed",
            );
            t.next(Phase::Repel);
        }
        Phase::Repel => {
            let stage = global.game.access().story_stage;
            if stage == STAGE_REPEL && story::modal_visible(global) {
                t.suite.check(
                    spring,
                    true,
                    "the rules' next stage reaches the story modal (Repel)",
                );
                t.next(Phase::RepelPaused);
            } else {
                t.waited_too_long(spring, 300, "Repel's story opening");
            }
        }
        Phase::RepelPaused => {
            if paused(spring) {
                t.suite.check(spring, true, "Repel's story pauses the game");
                let handled = input::key_press(spring, global, &key_s());
                t.suite.check(
                    spring,
                    !handled && global.game.access().story_stage == STAGE_REPEL,
                    "S does nothing on Repel's story",
                );
                t.next(Phase::CloseRepel);
            } else {
                t.waited_too_long(spring, 300, "the pause for Repel's story");
            }
        }
        Phase::CloseRepel if t.updates >= 5 => {
            input::action_story_next(spring, global);
            input::action_story_next(spring, global);
            t.next(Phase::Resumed);
        }
        Phase::Resumed => {
            if !paused(spring) && !story::modal_visible(global) {
                t.suite.check(
                    spring,
                    true,
                    "Continue closes Repel's story and resumes the game",
                );
                story::show(spring, global, STAGE_REPEL);
                input::action_restart(spring, global);
                t.next(Phase::Restarted);
            } else {
                t.waited_too_long(spring, 300, "the game resuming after Continue");
            }
        }
        Phase::Restarted => {
            let new_round = global.game.access().stale_round.is_none();
            if new_round && t.updates >= 30 {
                let stage = global.game.access().story_stage;
                t.suite.check(
                    spring,
                    stage == STAGE_INTRO && story::modal_visible(global) && !paused(spring),
                    format!("A restart opens the new round's intro, unpaused (stage {stage})"),
                );
                t.document_loads = global.document.access().loads();
                let context = global.document.access().context();
                let _ = spring.rml_ui().context_unload_all_documents(context);
                t.next(Phase::DocumentRecovered);
            } else if !new_round {
                t.waited_too_long(spring, 300, "the restarted round");
            }
        }
        Phase::DocumentRecovered => {
            if global.document.access().loads() > t.document_loads {
                t.suite.check(
                    spring,
                    true,
                    "An unloaded document is recreated in the existing context",
                );
                t.finish(spring);
            } else {
                t.waited_too_long(spring, 300, "document recovery");
            }
        }
        _ => {}
    }
}

fn ui_turn(spring: &spring::Spring<'_, spring::Ui>) -> bool {
    let spire_def = crate::defs::defs(spring).spire;
    spring
        .units_query()
        .get_team_units(PLAYER_TEAM_ID)
        .unwrap_or_default()
        .into_iter()
        .filter(|&unit| {
            spring
                .units_info()
                .get_unit_def_id(unit)
                .is_ok_and(|def| def == spire_def)
        })
        .any(|spire| {
            spring
                .rules_params()
                .get_unit_rules_param_f32(spire, flove_shared::suites::UI_TURN_PARAM)
                .is_ok_and(|turn| turn > 0.5)
        })
}

fn paused(spring: &spring::Spring<'_, spring::Ui>) -> bool {
    spring
        .display()
        .get_game_speed()
        .is_ok_and(|speed| speed.paused)
}

fn keep_game_running(spring: &spring::Spring<'_, spring::Ui>, t: &mut UiTestTracker) {
    t.since_resume += 1;
    if t.since_resume >= RESUME_INTERVAL && paused(spring) {
        t.since_resume = 0;
        let _ = spring.messages().send_commands("pause 0", "");
    }
}

fn key_s() -> KeyEvent<'static> {
    KeyEvent {
        key_code: i32::from(b's'),
        alt: false,
        ctrl: false,
        meta: false,
        shift: false,
        is_repeat: false,
        label: b"s",
        utf32_char: i32::from(b's'),
        scan_code: 0,
    }
}
