use spring_addons::rules::Gadget;

use crate::state::Global;

#[derive(Default)]
pub struct StoryGadget;

impl StoryGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for StoryGadget {
    fn name(&self) -> &'static str {
        "Story"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        crate::story::with_world(ctx.global(), frame, |story, world| story.update(world));
    }
}
