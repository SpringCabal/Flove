use spring_addons::ui::Widget;

use crate::state::Global;

#[derive(Default)]
pub struct StartForceWidget;

impl StartForceWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for StartForceWidget {
    fn name(&self) -> &'static str {
        "StartForce"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>) {
        let spring = _ctx.spring();
        if let Err(error) = spring.messages().send_commands("forcestart", "") {
            spring::log_warning!(
                spring,
                crate::diagnostics::SECTION,
                "forcestart failed: {error:?}"
            );
        }
    }
}
