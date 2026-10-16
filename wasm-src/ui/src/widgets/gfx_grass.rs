use spring_addons::ui::Widget;

use crate::state::Global;

const GRASS_DETAIL: i32 = 2000;

#[derive(Default)]
pub struct GfxGrassWidget;

impl GfxGrassWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GfxGrassWidget {
    fn name(&self) -> &'static str {
        "GfxGrass"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        if let Err(error) = spring::config::set_config_int("GrassDetail", GRASS_DETAIL, true) {
            spring::log_warning!(
                crate::diagnostics::SECTION,
                "setting GrassDetail failed (error code {})",
                error.code
            );
        }
    }
}
