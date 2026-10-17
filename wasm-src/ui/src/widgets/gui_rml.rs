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

    fn ensure_document(&self, spring: &spring::Spring<'_, spring::Ui>, global: &Global) {
        if global.document.access().is_loaded() {
            let since = self.since_check.get() + 1;
            if since < LIVENESS_CHECK_INTERVAL {
                self.since_check.set(since);
                return;
            }
            self.since_check.set(0);
            if rml::is_live(spring, global) != Some(false) {
                return;
            }
            rml::forget(spring, global);
            return;
        }
        if !rml::load(spring, global) {
            return;
        }
        let (started, stage) = {
            let game = global.game.access();
            (game.game_started, game.story_stage)
        };
        if started {
            rml::set_display(spring, global, "start-screen", false);
            if story::modal_visible(global) {
                story::show(spring, global, stage);
            } else {
                story::set_modal(spring, global, false);
            }
        } else {
            rml::set_display(spring, global, "start-screen", true);
            rml::set_display(spring, global, "story-modal", false);
        }
    }
}

impl Widget<Global> for GuiRmlWidget {
    fn name(&self) -> &'static str {
        "GuiRml"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>, _dt: f32) {
        let spring = ctx.spring();
        self.ensure_document(spring, ctx.global());
    }

    fn draw_screen(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        width: i32,
        height: i32,
    ) {
        let spring = ctx.spring();
        let context = ctx.global().document.access().context();
        if context == 0 {
            return;
        }
        let _ = spring
            .rml_ui()
            .context_set_dimensions(context, width, height);
        let _ = spring.rml_ui().context_update(context);
        let _ = spring.rml_ui().context_render(context);
    }
}
