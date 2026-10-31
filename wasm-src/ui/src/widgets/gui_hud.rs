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

#[derive(Default)]
struct Hud {
    shown: Cell<Status>,
    shown_in: Cell<u32>,
}

impl Hud {
    fn poll(&self, spring: &spring::Spring<'_, spring::Ui>, global: &Global) {
        let Some(status) = read_status(spring) else {
            return;
        };
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
            set_hud_int(spring, global, |hud| hud.power, status.power);
        }
        if reloaded || status.mana != shown.mana {
            set_hud_int(spring, global, |hud| hud.mana, status.mana);
        }
        update_story(
            spring,
            global,
            &status,
            prev_stage,
            shown.shroom_event,
            game_started,
        );
        if reloaded || status.game_over != prev_game_over {
            update_endgame(spring, global, &status);
        }
    }
}

fn read_status(spring: &spring::Spring<'_, spring::Ui>) -> Option<Status> {
    let params = spring.rules_params();
    // A missing round means the first synced status has not been published.
    // Reuse this read rather than querying it twice on every HUD update.
    let round = params.get_game_rules_param_f32("round").ok()?;
    Some(Status::read(|name| {
        if name == "round" {
            Some(round)
        } else {
            params.get_game_rules_param_f32(name).ok()
        }
    }))
}

fn update_story(
    spring: &spring::Spring<'_, spring::Ui>,
    global: &Global,
    status: &Status,
    prev_stage: i32,
    prev_shroom_event: i32,
    game_started: bool,
) {
    let story = status.story;
    if story == STAGE_FREE_PLAY && !game_started {
        global.game.access_mut().game_started = true;
        set_display(spring, global, "start-screen", false);
        story::set_modal(spring, global, false);
    }

    let tutorial_skipped = global.game.access().tutorial_skipped;
    let is_king_approach = (status.shroom_event == SHROOM_EVENT_KING_SPAWNED
        && prev_shroom_event != SHROOM_EVENT_KING_SPAWNED)
        || (story == STAGE_ARMY_APPROACHES && prev_stage != STAGE_ARMY_APPROACHES);

    if story == STAGE_VICTORY && prev_stage != STAGE_VICTORY {
        audio::play_narration(spring, STAGE_VICTORY);
        story::show(spring, global, STAGE_VICTORY);
    } else if is_king_approach {
        audio::play_narration(spring, STAGE_ARMY_APPROACHES);
        story::show(spring, global, STAGE_ARMY_APPROACHES);
    } else if tutorial_skipped {
        if story < STAGE_ARMY_APPROACHES && status.shroom_event == SHROOM_EVENT_NONE {
            story::set_modal(spring, global, false);
        }
    } else if game_started
        && story != prev_stage
        && !(story == STAGE_FREE_PLAY && prev_stage == STAGE_INTRO)
    {
        if story > STAGE_FREE_PLAY && story < STAGE_VICTORY {
            audio::play_narration(spring, story);
            story::show(spring, global, story);
        } else {
            story::set_modal(spring, global, false);
        }
    }
}

fn update_endgame(spring: &spring::Spring<'_, spring::Ui>, global: &Global, status: &Status) {
    if !status.game_over {
        set_display(spring, global, "endgame-modal", false);
        return;
    }
    if status.won {
        set_text(
            spring,
            global,
            "endgame-title",
            "The mushroom king lies defeated. The day is saved!",
        );
    } else {
        set_text(spring, global, "endgame-title", "Game Over");
    }
    set_display(spring, global, "endgame-modal", true);
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

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>, _dt: f32) {
        let spring = ctx.spring();
        self.hud.poll(spring, ctx.global());
    }
}
