use alloc::string::String;

use flove_shared::command;
use spring_addons::event::KeyEvent;
use springcabal::menu;
use springcabal::message::{Encoder, send_to_rules};

use crate::audio;
use crate::rml::set_display;
use crate::state::{Global, STAGE_FREE_PLAY, STAGE_INTRO};
use crate::story;

pub fn gameplay_active(global: &Global) -> bool {
    global.game.access().game_started && !story::modal_visible(global)
}

pub fn action_start_difficulty(
    spring: &spring::Spring<'_, spring::Ui>,
    global: &Global,
    diff: i32,
) {
    {
        let mut game = global.game.access_mut();
        game.round += 1;
        game.selected_difficulty = diff;
        game.game_started = true;
        game.tutorial_skipped = false;
        game.skip_offered = true;
        game.story_stage = STAGE_INTRO;
    }
    story::rewind(global);
    set_display(spring, global, "start-screen", false);
    audio::play_narration(spring, STAGE_INTRO);
    story::show(spring, global, STAGE_INTRO);
    send_to_rules(
        spring,
        &Encoder::new(command::DIFFICULTY).field(diff).finish(),
    );
}

pub fn action_story_next(spring: &spring::Spring<'_, spring::Ui>, global: &Global) -> bool {
    if story::reveal_all(spring, global) {
        return false;
    }
    let intro = {
        let mut game = global.game.access_mut();
        game.skip_offered = false;
        game.story_stage == STAGE_INTRO
    };
    if intro {
        set_display(spring, global, "btn-story-skip", false);
    } else {
        story::set_modal(spring, global, false);
    }
    send_to_rules(spring, command::STORY)
}

pub fn action_story_skip(spring: &spring::Spring<'_, spring::Ui>, global: &Global) -> bool {
    {
        let mut game = global.game.access_mut();
        if !game.skip_offered {
            return false;
        }
        game.skip_offered = false;
        game.story_stage = STAGE_FREE_PLAY;
        game.tutorial_skipped = true;
    }
    story::set_modal(spring, global, false);
    send_to_rules(spring, command::SKIP_TUTORIAL)
}

pub fn action_restart(spring: &spring::Spring<'_, spring::Ui>, global: &Global) {
    let diff = {
        let mut game = global.game.access_mut();
        game.round += 1;
        game.game_over = false;
        game.story_stage = STAGE_INTRO;
        game.tutorial_skipped = false;
        game.skip_offered = true;
        game.stale_round = Some(game.rules_round);
        game.selected_difficulty
    };
    story::reset(spring, global);
    set_display(spring, global, "endgame-modal", false);
    story::set_modal(spring, global, true);
    story::show(spring, global, STAGE_INTRO);
    audio::play_narration(spring, STAGE_INTRO);
    send_to_rules(spring, &Encoder::new(command::RESTART).field(diff).finish());
}

pub fn key_press(
    spring: &spring::Spring<'_, spring::Ui>,
    global: &Global,
    event: &KeyEvent<'_>,
) -> bool {
    if event.ctrl && (event.matches("a") || event.matches("d")) {
        let _ = spring.selection().select_unit_array(&[], false);
        return true;
    }
    if menu::is_quit_to_menu(event) {
        menu::quit_to_menu(spring);
        return true;
    }

    let (started, game_over) = {
        let game = global.game.access();
        (game.game_started, game.game_over)
    };

    if game_over {
        if event.matches("space") || event.matches("enter") || event.matches("r") {
            action_restart(spring, global);
            return true;
        }
        if event.matches("escape") {
            menu::quit_to_menu(spring);
            return true;
        }
    }

    if !started {
        if event.matches("escape") {
            menu::quit_to_menu(spring);
            return true;
        }
        let difficulty = if event.matches("space") || event.matches("enter") || event.matches("1") {
            1
        } else if event.matches("2") {
            2
        } else if event.matches("3") {
            3
        } else {
            return false;
        };
        action_start_difficulty(spring, global, difficulty);
        return true;
    }

    if story::modal_visible(global) {
        if event.matches("space") || event.matches("enter") {
            action_story_next(spring, global);
            return true;
        }
        if event.matches("s") {
            return action_story_skip(spring, global);
        }
    }

    false
}

const PLAYABLE_MIN: f32 = 2048.0;
const PLAYABLE_MAX: f32 = 8192.0;

fn trace_ground(
    spring: &spring::Spring<'_, spring::Ui>,
    screen_x: i32,
    screen_y: i32,
) -> Option<spring::Float3> {
    let pos = spring
        .camera()
        .ground_at(screen_x as f32, screen_y as f32)
        .ok()
        .flatten()?;
    let playable = PLAYABLE_MIN..=PLAYABLE_MAX;
    (playable.contains(&pos.x) && playable.contains(&pos.z)).then_some(pos)
}

pub fn spire_command(
    spring: &spring::Spring<'_, spring::Ui>,
    x: i32,
    y: i32,
    left: bool,
    right: bool,
) -> Option<String> {
    if left && let Some(target) = trace_ground(spring, x, y) {
        return Some(Encoder::new(command::ZAP).fields(xyz(target)).finish());
    }
    if !right {
        return None;
    }
    let hit_unit = spring.camera().unit_at(x as f32, y as f32).ok().flatten();
    let target = trace_ground(spring, x, y)?;
    let message = Encoder::new(command::RMB);
    let message = match hit_unit {
        Some(unit_id) => message.field(unit_id),
        None => message,
    };
    Some(message.fields(xyz(target)).finish())
}

fn xyz(pos: spring::Float3) -> [f32; 3] {
    [pos.x, pos.y, pos.z]
}

pub fn send_hold(spring: &spring::Spring<'_, spring::Ui>, held: Option<&str>) -> bool {
    match held {
        Some(held) => send_to_rules(spring, &Encoder::new(command::HOLD).rest(held).finish()),
        None => send_to_rules(spring, command::RELEASE),
    }
}
