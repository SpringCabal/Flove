#![no_std]

pub const STAGE_FREE_PLAY: i32 = 0;
pub const STAGE_INTRO: i32 = 1;
pub const STAGE_REPEL: i32 = 2;
pub const STAGE_HEAL: i32 = 3;
pub const STAGE_FIRST_WAVE: i32 = 4;
pub const STAGE_UPGRADE: i32 = 5;
pub const STAGE_FLOWER_FIELDS: i32 = 6;
pub const STAGE_ARMY_APPROACHES: i32 = 7;
pub const STAGE_VICTORY: i32 = 8;

pub const SHROOM_EVENT_NONE: i32 = 0;
pub const SHROOM_EVENT_KING_SPAWNED: i32 = 1;
pub const SHROOM_EVENT_KING_DEFEATED: i32 = 2;

pub const fn skip_offered(stage: i32) -> bool {
    stage == STAGE_INTRO
}

pub const fn story_pauses(stage: i32) -> bool {
    stage > STAGE_INTRO && stage <= STAGE_VICTORY
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status {
    pub round: i32,
    pub power: i32,
    pub mana: i32,
    pub story: i32,
    pub shroom_event: i32,
    pub game_over: bool,
    pub won: bool,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            round: 0,
            power: 0,
            mana: 0,
            story: STAGE_INTRO,
            shroom_event: SHROOM_EVENT_NONE,
            game_over: false,
            won: false,
        }
    }
}

impl Status {
    pub const PARAMS: [&'static str; 7] = [
        "round",
        "power",
        "mana",
        "story",
        "shroom_event",
        "game_over",
        "won",
    ];

    pub fn values(&self) -> [f32; 7] {
        let flag = |on: bool| if on { 1.0 } else { 0.0 };
        [
            self.round as f32,
            self.power as f32,
            self.mana as f32,
            self.story as f32,
            self.shroom_event as f32,
            flag(self.game_over),
            flag(self.won),
        ]
    }

    pub fn read(mut read: impl FnMut(&str) -> Option<f32>) -> Self {
        let default = Self::default();
        let mut value = |index: usize, default: f32| read(Self::PARAMS[index]).unwrap_or(default);
        let defaults = default.values();
        Self {
            round: value(0, defaults[0]) as i32,
            power: value(1, defaults[1]) as i32,
            mana: value(2, defaults[2]) as i32,
            story: value(3, defaults[3]) as i32,
            shroom_event: value(4, defaults[4]) as i32,
            game_over: value(5, defaults[5]) > 0.5,
            won: value(6, defaults[6]) > 0.5,
        }
    }
}

pub mod command {
    pub const DIFFICULTY: &str = "difficulty";
    pub const STORY: &str = "story";
    pub const SKIP_TUTORIAL: &str = "skip_tutorial";
    pub const RESTART: &str = "restart";
    pub const ZAP: &str = "zap";
    pub const RMB: &str = "rmb";
    pub const HOLD: &str = "hold";
    pub const RELEASE: &str = "release";
}

pub mod grove {
    pub fn power_level(total_power: i32) -> i32 {
        (1.0 + sqrt(total_power.max(0) as f32) / 5.0) as i32
    }

    pub fn flare_level(total_power: i32) -> i32 {
        power_level(total_power).min(8)
    }

    pub fn branch_count(total_power: i32) -> i32 {
        (power_level(total_power).min(10) - 2).max(0)
    }

    fn sqrt(value: f32) -> f32 {
        if value <= 0.0 {
            return 0.0;
        }
        let mut root = value.max(1.0);
        for _ in 0..32 {
            root = 0.5 * (root + value / root);
        }
        root
    }
}

pub mod suites {
    pub const UI_TURN_PARAM: &str = "flove_ui_suite_turn";
    pub const UI_DONE: &str = "flove_ui_suite_done";
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    #[test]
    fn only_the_intro_offers_skip() {
        assert!(skip_offered(STAGE_INTRO));
        for stage in [STAGE_FREE_PLAY, STAGE_REPEL, STAGE_HEAL, STAGE_FIRST_WAVE]
            .into_iter()
            .chain([STAGE_UPGRADE, STAGE_FLOWER_FIELDS])
            .chain([STAGE_ARMY_APPROACHES, STAGE_VICTORY])
        {
            assert!(!skip_offered(stage), "stage {stage}");
        }
    }

    #[test]
    fn every_story_after_the_intro_pauses() {
        assert!(!story_pauses(STAGE_FREE_PLAY));
        assert!(!story_pauses(STAGE_INTRO));
        for stage in STAGE_REPEL..=STAGE_VICTORY {
            assert!(story_pauses(stage), "stage {stage}");
        }
    }

    fn lua_branches(total_power: i32) -> i32 {
        let level = libm_free_level(total_power);
        let desired = (level - 2).clamp(0, 9);
        let mut branches = 0;
        loop {
            let before = branches;
            let mut i = branches + 1;
            let shown_from = branches;
            while i <= desired {
                branches += 1;
                i += 1;
            }
            if branches == shown_from {
                let mut i = branches;
                while i > desired {
                    branches -= 1;
                    i -= 1;
                }
            }
            if branches == before {
                return branches;
            }
        }
    }

    fn libm_free_level(total_power: i32) -> i32 {
        let root = (total_power as f64).sqrt();
        ((1.0 + root / 5.0).floor() as i32).min(10)
    }

    #[test]
    fn branches_match_the_lua_controller() {
        for power in 0..4000 {
            assert_eq!(
                grove::branch_count(power),
                lua_branches(power),
                "power {power}"
            );
        }
    }

    #[test]
    fn power_levels() {
        assert_eq!(grove::power_level(0), 1);
        assert_eq!(grove::power_level(24), 1);
        assert_eq!(grove::power_level(25), 2);
        assert_eq!(grove::power_level(100), 3);
        assert_eq!(grove::flare_level(10_000), 8);
    }

    #[test]
    fn status_round_trip() {
        let status = Status {
            round: 3,
            power: 51,
            mana: 7,
            story: STAGE_VICTORY,
            shroom_event: SHROOM_EVENT_KING_DEFEATED,
            game_over: true,
            won: true,
        };
        let values = status.values();
        let read = Status::read(|name| {
            let index = Status::PARAMS.iter().position(|param| *param == name)?;
            Some(values[index])
        });
        assert_eq!(read, status);
    }

    #[test]
    fn missing_params_read_as_the_intro() {
        assert_eq!(Status::read(|_| None), Status::default());
        assert_eq!(Status::default().story, STAGE_INTRO);
    }
}
