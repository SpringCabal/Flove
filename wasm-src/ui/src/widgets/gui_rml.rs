use core::cell::Cell;

use spring_addons::ui::Widget;

use crate::rml;
use crate::state::Global;
use crate::story;

const LIVENESS_CHECK_INTERVAL: u32 = 30;

#[derive(Default)]
pub struct GuiRmlWidget {
    since_check: Cell<u32>,
}

impl GuiRmlWidget {
    pub fn new() -> Self {
        Self::default()
    }

    fn ensure_document(&self, global: &Global) {
        if global.document.access().is_loaded() {
            let since = self.since_check.get() + 1;
            if since < LIVENESS_CHECK_INTERVAL {
                self.since_check.set(since);
                return;
            }
            self.since_check.set(0);
            if rml::is_live(global) != Some(false) {
                return;
            }
            rml::forget(global);
            return;
        }
        if !rml::load(global) {
            return;
        }
        let (started, stage) = {
            let game = global.game.access();
            (game.game_started, game.story_stage)
        };
        if started {
            rml::set_display(global, "start-screen", false);
            if story::modal_visible(global) {
                story::show(global, stage);
            } else {
                story::set_modal(global, false);
            }
        } else {
            rml::set_display(global, "start-screen", true);
            rml::set_display(global, "story-modal", false);
        }
    }
}

impl Widget<Global> for GuiRmlWidget {
    fn name(&self) -> &'static str {
        "GuiRml"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        self.ensure_document(ctx.global());
    }

    fn draw_screen(&self, ctx: &spring_addons::AddonContext<'_, Global>, width: i32, height: i32) {
        let context = ctx.global().document.access().context();
        if context == 0 {
            return;
        }
        let _ = spring::context_set_dimensions(context, width, height);
        let _ = spring::context_update(context);
        let _ = spring::context_render(context);
    }
}
