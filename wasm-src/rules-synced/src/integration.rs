mod rounds;
mod tutorial;
mod world;

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::fmt::Display;

use spring::Float3;
use spring::rules_params::los;
use spring::rules_params::{RulesParamType, RulesParamValue};
use spring_addons::AddonContext;
use springcabal::check::Suite;

use crate::definitions::{ENEMY_TEAM_ID, defs, unit_create_options};
use crate::state::Global;

const SECTION: &str = "flove-rust-test";

const RUNNING_PARAM: &str = "flove_rust_suite_running";

const EXPECTED_CHECKS: u32 = 66;

enum Step {
    Done,
    Wait,
}

type Run = fn(&mut TestTracker, &AddonContext<'_, Global>, i32) -> Step;

const STEPS: &[(&str, Run, i32)] = &[
    ("world", world::world, 10),
    ("wisps", world::wisps, 1),
    ("zap", world::zap, 1),
    ("zap lands", world::zap_lands, 90),
    ("growth", world::growth, 1),
    ("grown tree", world::grown_tree, 5),
    ("mushroom AI", world::mushroom_ai, 40),
    ("flower shots", world::flower_shots, 1),
    ("flower shot reload", world::flower_shot_reload, 20),
    ("flower field", world::flower_field, 200),
    ("flower slow", world::flower_slow, 60),
    ("poison", world::poison, 1),
    ("intro", tutorial::intro, 1),
    ("repel", tutorial::repel, 200),
    ("heal", tutorial::heal, 200),
    ("first wave", tutorial::first_wave, 200),
    ("upgrade stage", tutorial::upgrade_stage, 10),
    ("upgrade shots", tutorial::upgrade_shots, 400),
    ("flower fields", tutorial::flower_fields, 10),
    ("free play", tutorial::free_play, 1),
    ("skip round", rounds::skip_round, 1),
    ("skip", rounds::skip, 5),
    ("king arrives", rounds::king_arrives, 5),
    ("king defeated", rounds::king_defeated, 5),
    ("victory published", rounds::victory_published, 5),
    ("restart", rounds::restart, 5),
    ("destroy spire", rounds::destroy_spire, 1),
    ("defeat", rounds::defeat, 5),
    ("fresh round", rounds::fresh_round, 5),
];

pub struct TestTracker {
    checks: Checks,
    step: usize,
    step_start: i32,
    ui_done: bool,
    mushroom: i32,
    king: i32,
    tree: i32,
    trees_before: Vec<i32>,
    count: usize,
    power: i32,
    shots: i32,
    shot_frame: i32,
}

impl Default for TestTracker {
    fn default() -> Self {
        Self {
            checks: Checks::new(Suite::new(
                "[Flove-Rust-Test]",
                "FLOVE RUST INTEGRATION TESTS",
                EXPECTED_CHECKS,
            )),
            step: 0,
            step_start: -1,
            ui_done: false,
            mushroom: -1,
            king: -1,
            tree: -1,
            trees_before: Vec::new(),
            count: 0,
            power: 0,
            shots: 0,
            shot_frame: 0,
        }
    }
}

impl TestTracker {
    fn check(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        ok: bool,
        message: impl Display,
    ) -> bool {
        self.checks.check(spring, ok, message)
    }

    fn elapsed(&self, frame: i32) -> i32 {
        frame - self.step_start
    }
}

pub fn announce(spring: &spring::Spring<'_, spring::RulesSynced>) {
    set_running(spring, true);
}

type Waiting = Box<dyn for<'call> FnOnce(&spring::Spring<'call, spring::RulesSynced>, &Checks)>;

#[derive(Clone)]
struct Checks {
    suite: Rc<RefCell<Suite>>,
    waiting: Rc<RefCell<VecDeque<Waiting>>>,
    draining: Rc<Cell<bool>>,
}

impl Checks {
    fn new(suite: Suite) -> Self {
        Self {
            suite: Rc::new(RefCell::new(suite)),
            waiting: Rc::default(),
            draining: Rc::default(),
        }
    }

    fn check(
        &self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        ok: bool,
        message: impl Display,
    ) -> bool {
        self.suite.borrow_mut().check(spring, ok, message)
    }

    fn after_commands(&self, f: impl FnOnce(&Checks, &AddonContext<'_, Global>) + 'static) {
        self.waiting
            .borrow_mut()
            .push_back(Box::new(move |_spring, checks| {
                crate::with_addon_context(|ctx| f(checks, ctx));
            }));
        if !self.draining.replace(true) {
            self.queue_drain();
        }
    }

    fn queue_drain(&self) {
        let checks = self.clone();
        crate::with_addon_context(|ctx| ctx.commands().run(move |spring| checks.drain(spring)));
    }

    fn drain(&self, spring: &spring::Spring<'_, spring::RulesSynced>) {
        loop {
            if !crate::with_addon_context(|ctx| ctx.commands().is_empty()) {
                return self.queue_drain();
            }
            let next = self.waiting.borrow_mut().pop_front();
            let Some(next) = next else {
                self.draining.set(false);
                return;
            };
            next(spring, self);
        }
    }
}

pub fn update(ctx: &AddonContext<'_, Global>, t: &mut TestTracker, frame: i32) {
    let spring = ctx.spring();
    if t.checks.suite.borrow().is_finished() {
        return;
    }
    if t.step_start < 0 {
        spring::log_info!(
            spring,
            SECTION,
            "Starting the Flove Rust integration suite..."
        );
        t.step_start = frame;
    }
    let Some(&(name, run, timeout)) = STEPS.get(t.step) else {
        hand_over_to_ui(spring, ctx.global());
        t.checks.suite.borrow_mut().finish(spring);
        return;
    };
    let timed_out = match run(t, ctx, frame) {
        Step::Done => false,
        Step::Wait if t.elapsed(frame) < timeout => return,
        Step::Wait => true,
    };
    if timed_out {
        t.check(
            spring,
            false,
            format!("step '{name}' finished within {timeout} frames"),
        );
    }
    t.step += 1;
    t.step_start = frame + 1;
}

pub fn ui_suite_done(spring: &spring::Spring<'_, spring::RulesSynced>, t: &mut TestTracker) {
    if !t.ui_done {
        t.ui_done = true;
        set_running(spring, false);
    }
}

fn hand_over_to_ui(spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) {
    let spire = global.spire.access().unit_id;
    let _ = spring.rules_params().set_unit_rules_param_f32(
        spire,
        flove_shared::suites::UI_TURN_PARAM,
        1.0,
        los::PUBLIC,
    );
}

fn set_running(spring: &spring::Spring<'_, spring::RulesSynced>, running: bool) {
    let value = RulesParamValue {
        type_: RulesParamType::RulesparamTypeFloat,
        bool_value: false,
        float_value: if running { 1.0 } else { 0.0 },
        string_value: String::new(),
    };
    let _ = spring
        .rules_params()
        .set_game_rules_param(RUNNING_PARAM, &value, los::PUBLIC);
}

fn send(ctx: &AddonContext<'_, Global>, message: &str) {
    crate::input::handle_command(ctx, message);
}

#[derive(Debug, PartialEq)]
struct SkipSnapshot {
    stage: i32,
    skipped: bool,
    trees_take_no_damage: bool,
    mana: i32,
    trees: usize,
    mushrooms: usize,
    first_wave_spawned: bool,
}

impl SkipSnapshot {
    fn take(spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) -> Self {
        let story = global.story.access();
        Self {
            stage: story.stage,
            skipped: story.tutorial_skipped,
            trees_take_no_damage: story.trees_take_no_damage,
            mana: *global.mana.access(),
            trees: living(spring, |def| defs(spring).is_tree(def)),
            mushrooms: living(spring, |def| defs(spring).is_mushroom(def)),
            first_wave_spawned: global
                .mushrooms
                .access()
                .waves
                .first()
                .is_some_and(|wave| wave.spawned),
        }
    }
}

fn check_skip_refused(
    _spring: &spring::Spring<'_, spring::RulesSynced>,
    t: &TestTracker,
    where_: &'static str,
) {
    t.checks.after_commands(move |checks, ctx| {
        let spring = ctx.spring();
        let before = SkipSnapshot::take(spring, ctx.global());
        send(ctx, flove_shared::command::SKIP_TUTORIAL);
        checks.after_commands(move |checks, ctx| {
            let spring = ctx.spring();
            let after = SkipSnapshot::take(spring, ctx.global());
            checks.check(
                spring,
                after == before,
                format!(
                    "Skip Tutorial changes nothing {where_} (stage {})",
                    after.stage
                ),
            );
        });
    });
}

fn spawn(spring: &spring::Spring<'_, spring::RulesSynced>, def: i32, (x, z): (f32, f32)) -> i32 {
    spring
        .unit_control()
        .create_unit(
            &spring::unit_control::DefRef {
                name: String::new(),
                id: def,
            },
            Float3::new(x, 0.0, z),
            0,
            ENEMY_TEAM_ID,
            unit_create_options(),
        )
        .unwrap_or(-1)
}

fn destroy(spring: &spring::Spring<'_, spring::RulesSynced>, unit: i32) {
    let _ = spring.unit_control().destroy_unit(
        unit,
        spring::unit_control::DestroyUnitOptions {
            selfd: false,
            reclaimed: false,
            attacker_id: -1,
            recycle_id: false,
        },
    );
}

fn published(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    _global: &Global,
    name: &str,
) -> i32 {
    spring
        .rules_params()
        .get_game_rules_param_f32(name)
        .map_or(-1, |value| value as i32)
}

fn living_units(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    matches: impl Fn(i32) -> bool,
) -> Vec<i32> {
    spring
        .units()
        .get_all_units()
        .unwrap_or_default()
        .into_iter()
        .filter(|&unit| {
            spring
                .units_info()
                .get_unit_def_id(unit)
                .is_ok_and(&matches)
                && !spring.units_info().get_unit_is_dead(unit).unwrap_or(true)
        })
        .collect()
}

fn living(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    matches: impl Fn(i32) -> bool,
) -> usize {
    living_units(spring, matches).len()
}

fn ground_position(spring: &spring::Spring<'_, spring::RulesSynced>, unit: i32) -> Option<Float3> {
    spring
        .units_info()
        .get_unit_position(unit, Default::default())
        .ok()
}

fn distance_2d(spring: &spring::Spring<'_, spring::RulesSynced>, a: i32, b: i32) -> Option<f32> {
    let (a, b) = (ground_position(spring, a)?, ground_position(spring, b)?);
    let (dx, dz) = (a.x - b.x, a.z - b.z);
    Some(libm::sqrtf(dx * dx + dz * dz))
}

fn xyz(pos: Float3) -> String {
    format!("{}|{}|{}", pos.x, pos.y, pos.z)
}
