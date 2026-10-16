use alloc::format;

use spring::ResultExt;

use crate::rml::{set_display, set_text};
use crate::state::{Global, STAGE_FREE_PLAY};

pub struct StoryModal {
    visible: bool,
    full_text: &'static str,
    char_index: usize,
    char_timer: f32,
    char_interval: f32,
    paused_game: bool,
}

impl Default for StoryModal {
    fn default() -> Self {
        Self {
            visible: true,
            full_text: "",
            char_index: 0,
            char_timer: 0.0,
            char_interval: 0.075,
            paused_game: false,
        }
    }
}

pub fn modal_visible(global: &Global) -> bool {
    global.story.access().visible
}

pub fn set_modal(global: &Global, visible: bool) {
    let resume = {
        let mut story = global.story.access_mut();
        story.visible = visible;
        !visible && core::mem::take(&mut story.paused_game)
    };
    set_display(global, "story-modal", visible);
    if resume {
        set_paused(false);
    }
}

fn pause_for(global: &Global, stage: i32) {
    if !flove_shared::story_pauses(stage) {
        return;
    }
    global.story.access_mut().paused_game = true;
    set_paused(true);
}

fn set_paused(paused: bool) {
    let command = if paused { "pause 1" } else { "pause 0" };
    spring::send_command(command).log_err_once("pausing the game for the story");
}

pub fn reset(global: &Global) {
    *global.story.access_mut() = StoryModal::default();
    set_paused(false);
}

pub fn rewind(global: &Global) {
    let mut story = global.story.access_mut();
    story.visible = true;
    story.char_index = 0;
    story.char_timer = 0.0;
}

pub fn reveal_all(global: &Global) -> bool {
    let text = {
        let mut story = global.story.access_mut();
        let total_chars = story.full_text.chars().count();
        if story.visible && story.char_index < total_chars {
            story.char_index = total_chars;
            Some(story.full_text)
        } else {
            None
        }
    };
    match text {
        Some(text) => {
            set_text(global, "story-body", text);
            true
        }
        None => false,
    }
}

#[cfg(feature = "headless-integration")]
pub fn typing_progress(global: &Global) -> (usize, usize) {
    let story = global.story.access();
    (story.char_index, story.full_text.chars().count())
}

pub fn show(global: &Global, stage: i32) {
    if stage == STAGE_FREE_PLAY {
        set_modal(global, false);
        return;
    }
    set_modal(global, true);
    pause_for(global, stage);

    let (body, next_label, duration) = match stage {
        1 => (
            "In times long gone, the trees, flowers and grasses lived peacefully in the secluded mountain groves. \nBut war raged in neighbouring kingdoms, displacing hundreds of ordinary mushrooms from their broken homes.",
            "Next",
            16.0,
        ),
        2 => (
            "Repel borders! Push back the oncoming mushrooms, or they'll take the soil and sunlight from our native fauna!\n\nHint: Left click on mushrooms to send love.",
            "Continue",
            7.0,
        ),
        3 => (
            "They reject our love and kindness, but the trees do not... \nMend the wounds inflicted by the wretched fungi and ease their suffering!\n\nHint: Left click on trees to heal them.",
            "Continue",
            9.5,
        ),
        4 => (
            "Blasted, another wave of enemies approaches. Protect the forest, purify the invaders!",
            "Continue",
            6.5,
        ),
        5 => (
            "This seems to be the end of them for now. We must bolster our defences for what is to come.\n\nMake our little garden stronger\n\nHint: Right click on trees to upgrade them.",
            "Continue",
            7.5,
        ),
        6 => (
            "It's a beautiful sight indeed. But will it be enough for what is to come? Take good care of the fauna. Nurture it and it will grow.\n\nAlso, beware of suicide bombing mushrooms.\n\nHint: Right click on mushrooms to create flower fields that slows them.",
            "Continue",
            18.0,
        ),
        7 => (
            "A vast enemy army approaches, defend the forest at all costs!",
            "Continue",
            6.5,
        ),
        8 => (
            "The mushroom king lies defeated. The day is saved!",
            "Continue",
            6.5,
        ),
        _ => ("", "Continue", 5.0),
    };

    let total_chars = body.chars().count();
    let char_interval = if total_chars > 0 {
        (duration / total_chars as f32).max(0.055)
    } else {
        0.08
    };
    {
        let mut story = global.story.access_mut();
        story.full_text = body;
        story.char_index = 0;
        story.char_timer = 0.0;
        story.char_interval = char_interval;
    }

    set_text(global, "story-title", "Groves:  OPERATION MUSHROOM");
    set_text(global, "story-body", "");
    set_text(global, "btn-story-next", &format!("{next_label} [Space]"));
    let show_skip = flove_shared::skip_offered(stage) && global.game.access().skip_offered;
    if show_skip {
        set_text(global, "btn-story-skip", "Skip Tutorial [S]");
    }
    set_display(global, "btn-story-skip", show_skip);
}

pub fn update_typing(global: &Global, delta_seconds: f32) {
    let dt = delta_seconds.clamp(0.0, 0.1);
    let visible_text = {
        let mut story = global.story.access_mut();
        advance(&mut story, dt)
    };
    if let Some(text) = visible_text {
        set_text(global, "story-body", text);
    }
}

fn advance(story: &mut StoryModal, dt: f32) -> Option<&'static str> {
    if !story.visible || story.full_text.is_empty() {
        return None;
    }
    let total_chars = story.full_text.chars().count();
    if story.char_index >= total_chars {
        return None;
    }
    story.char_timer += dt;
    let interval = story.char_interval.max(0.04);
    let mut advanced = false;
    while story.char_timer >= interval && story.char_index < total_chars {
        story.char_timer -= interval;
        story.char_index += 1;
        advanced = true;
    }
    if !advanced {
        return None;
    }
    let byte_offset = story
        .full_text
        .char_indices()
        .nth(story.char_index)
        .map(|(idx, _)| idx)
        .unwrap_or(story.full_text.len());
    let text: &'static str = story.full_text;
    Some(&text[..byte_offset])
}
