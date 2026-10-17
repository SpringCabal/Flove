use core::cell::Cell;

use spring_addons::AddonContext;
use spring_addons::ui::Widget;
use springcabal::music::MusicLoop;

use crate::state::Global;

const MUSIC_FILE: &str = "music/Rain Rain Go Away.ogg";
const MUSIC_VOLUME: f32 = 0.15;

pub struct SndMusicWidget {
    music: MusicLoop,
    round: Cell<u32>,
}

impl SndMusicWidget {
    pub fn new() -> Self {
        Self {
            music: MusicLoop::new(MUSIC_FILE, MUSIC_VOLUME),
            round: Cell::new(0),
        }
    }
}

impl Widget<Global> for SndMusicWidget {
    fn name(&self) -> &'static str {
        "SndMusic"
    }

    fn update(&self, ctx: &AddonContext<'_, Global, spring::Ui>, dt: f32) {
        let round = ctx.global().game.access().round;
        if round != self.round.replace(round) && round > 1 {
            self.music.restart();
        }
        self.music.update(ctx, dt);
    }

    fn shutdown(&self, ctx: &AddonContext<'_, Global, spring::Ui>) {
        self.music.shutdown(ctx);
    }
}
