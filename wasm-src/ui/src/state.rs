use spring_addons::Resource;

use crate::rml::Document;
use crate::story::StoryModal;

pub use flove_shared::{
    SHROOM_EVENT_KING_SPAWNED, SHROOM_EVENT_NONE, STAGE_ARMY_APPROACHES, STAGE_FREE_PLAY,
    STAGE_INTRO, STAGE_VICTORY,
};

#[derive(Debug)]
pub struct GameState {
    pub round: u32,
    pub selected_difficulty: i32,
    pub game_started: bool,
    pub tutorial_skipped: bool,
    pub skip_offered: bool,
    pub story_stage: i32,
    pub game_over: bool,
    pub rules_round: i32,
    pub stale_round: Option<i32>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            round: 0,
            selected_difficulty: 1,
            game_started: false,
            tutorial_skipped: false,
            skip_offered: false,
            story_stage: STAGE_INTRO,
            game_over: false,
            rules_round: 0,
            stale_round: None,
        }
    }
}

#[derive(Default)]
pub struct Global {
    pub game: Resource<GameState>,
    pub story: Resource<StoryModal>,
    pub document: Resource<Document>,
}
