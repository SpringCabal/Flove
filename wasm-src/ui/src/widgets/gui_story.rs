use spring_addons::ui::Widget;

use crate::state::Global;
use crate::story;

#[derive(Default)]
pub struct GuiStoryWidget;

impl GuiStoryWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GuiStoryWidget {
    fn name(&self) -> &'static str {
        "GuiStory"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, dt: f32) {
        story::update_typing(ctx.global(), dt);
    }
}
