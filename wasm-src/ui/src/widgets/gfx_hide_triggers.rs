use spring_addons::ui::Widget;

use crate::defs::defs;
use crate::state::Global;

#[derive(Default)]
pub struct GfxHideTriggersWidget;

impl GfxHideTriggersWidget {
    pub fn new() -> Self {
        Self
    }
}

fn hide_if_trigger(spring: &spring::Spring<'_, spring::Ui>, unit: i32, def: i32) {
    if defs(spring).triggers.contains(&def)
        && let Err(error) = spring.unsynced_ctrl().set_unit_no_draw(unit, true)
    {
        spring::log_warning!(
            spring,
            crate::diagnostics::SECTION,
            "hiding marker unit {unit} failed: {error:?}"
        );
    }
}

impl Widget<Global> for GfxHideTriggersWidget {
    fn name(&self) -> &'static str {
        "GfxHideTriggers"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>) {
        let spring = _ctx.spring();
        for unit in spring.units_query().get_all_units().unwrap_or_default() {
            if let Ok(def) = spring.units_info().get_unit_def_id(unit) {
                hide_if_trigger(spring, unit, def);
            }
        }
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        unit: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        let spring = _ctx.spring();
        hide_if_trigger(spring, unit, def);
    }
}
