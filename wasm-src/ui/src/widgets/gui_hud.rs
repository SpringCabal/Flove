use core::cell::Cell;

use flove_shared::Status;
use spring_addons::ui::Widget;

use crate::audio;
use crate::rml::{set_display, set_hud_int, set_text};
use crate::state::{
    Global, SHROOM_EVENT_KING_SPAWNED, SHROOM_EVENT_NONE, STAGE_ARMY_APPROACHES, STAGE_FREE_PLAY,
    STAGE_INTRO, STAGE_VICTORY,
};
use crate::story;

const PLAYER_TEAM_ID: i32 = 1;

#[derive(Default)]
struct Hud {
    carrier: Cell<Option<i32>>,
    shown: Cell<Status>,
    shown_in: Cell<u32>,
}

impl Hud {
    fn carrier(&self) -> Option<i32> {
        let alive = |unit: i32| !spring::units_info::get_unit_is_dead(unit).unwrap_or(true);
        if let Some(unit) = self.carrier.get()
            && alive(unit)
        {
            return Some(unit);
        }
        let units = spring::units_query::get_team_units(PLAYER_TEAM_ID).ok()?;
        let spire_def = crate::defs::defs().spire;
        let is_spire =
            |unit: i32| spring::units_info::get_unit_def_id(unit).is_ok_and(|d| d == spire_def);
        let found = units
            .iter()
            .copied()
            .find(|&unit| is_spire(unit) && alive(unit))
            .or_else(|| {
                units.iter().copied().find(|&unit| {
                    alive(unit)
                        && spring::rules_params::get_unit_rules_param_f32(unit, "power").is_ok()
                })
            });
        self.carrier.set(found);
        found
    }

    fn poll(&self, global: &Global) {
        let Some(unit) = self.carrier() else {
            return;
        };
        let status = read_status(unit);
        {
            let mut game = global.game.access_mut();
            game.rules_round = status.round;
            if game.stale_round == Some(status.round) {
                return;
            }
            game.stale_round = None;
        }
        let shown = self.shown.replace(status);
        let loads = global.document.access().loads();
        let reloaded = self.shown_in.replace(loads) != loads;
        let (prev_stage, prev_game_over, game_started) = {
            let mut game = global.game.access_mut();
            let prev = (game.story_stage, game.game_over, game.game_started);
            if status.story > STAGE_FREE_PLAY {
                game.story_stage = status.story;
            }
            game.game_over = status.game_over;
            prev
        };

        if reloaded || status.power != shown.power {
            set_hud_int(global, |hud| hud.power, status.power);
        }
        if reloaded || status.mana != shown.mana {
            set_hud_int(global, |hud| hud.mana, status.mana);
        }
        update_story(
            global,
            &status,
            prev_stage,
            shown.shroom_event,
            game_started,
        );
        if reloaded || status.game_over != prev_game_over {
            update_endgame(global, &status);
        }
    }
}

fn read_status(unit: i32) -> Status {
    Status::read(|name| spring::rules_params::get_unit_rules_param_f32(unit, name).ok())
}

fn update_story(
    global: &Global,
    status: &Status,
    prev_stage: i32,
    prev_shroom_event: i32,
    game_started: bool,
) {
    let story = status.story;
    if story == STAGE_FREE_PLAY && !game_started {
        global.game.access_mut().game_started = true;
        set_display(global, "start-screen", false);
        story::set_modal(global, false);
    }

    let tutorial_skipped = global.game.access().tutorial_skipped;
    let is_king_approach = (status.shroom_event == SHROOM_EVENT_KING_SPAWNED
        && prev_shroom_event != SHROOM_EVENT_KING_SPAWNED)
        || (story == STAGE_ARMY_APPROACHES && prev_stage != STAGE_ARMY_APPROACHES);

    if story == STAGE_VICTORY && prev_stage != STAGE_VICTORY {
        audio::play_narration(STAGE_VICTORY);
        story::show(global, STAGE_VICTORY);
    } else if is_king_approach {
        audio::play_narration(STAGE_ARMY_APPROACHES);
        story::show(global, STAGE_ARMY_APPROACHES);
    } else if tutorial_skipped {
        if story < STAGE_ARMY_APPROACHES && status.shroom_event == SHROOM_EVENT_NONE {
            story::set_modal(global, false);
        }
    } else if game_started
        && story != prev_stage
        && !(story == STAGE_FREE_PLAY && prev_stage == STAGE_INTRO)
    {
        if story > STAGE_FREE_PLAY && story < STAGE_VICTORY {
            audio::play_narration(story);
            story::show(global, story);
        } else {
            story::set_modal(global, false);
        }
    }
}

fn update_endgame(global: &Global, status: &Status) {
    if !status.game_over {
        set_display(global, "endgame-modal", false);
        return;
    }
    if status.won {
        set_text(
            global,
            "endgame-title",
            "The mushroom king lies defeated. The day is saved!",
        );
    } else {
        set_text(global, "endgame-title", "Game Over");
    }
    set_display(global, "endgame-modal", true);
}

#[derive(Default)]
pub struct GuiHudWidget {
    hud: Hud,
}

impl GuiHudWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for GuiHudWidget {
    fn name(&self) -> &'static str {
        "GuiHud"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        self.hud.poll(ctx.global());
    }
}
