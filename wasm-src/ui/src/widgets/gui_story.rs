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

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>, dt: f32) {
        let spring = ctx.spring();
        story::update_typing(spring, ctx.global(), dt);
    }
}
