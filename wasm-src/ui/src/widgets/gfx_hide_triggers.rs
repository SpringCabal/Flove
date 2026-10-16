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

fn hide_if_trigger(unit: i32, def: i32) {
    if defs().triggers.contains(&def)
        && let Err(error) = spring::unsynced_ctrl::set_unit_no_draw(unit, true)
    {
        spring::log_warning!(
            crate::diagnostics::SECTION,
            "hiding marker unit {unit} failed: {error:?}"
        );
    }
}

impl Widget<Global> for GfxHideTriggersWidget {
    fn name(&self) -> &'static str {
        "GfxHideTriggers"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        for unit in spring::units_query::get_all_units().unwrap_or_default() {
            if let Ok(def) = spring::units_info::get_unit_def_id(unit) {
                hide_if_trigger(unit, def);
            }
        }
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        unit: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        hide_if_trigger(unit, def);
    }
}
