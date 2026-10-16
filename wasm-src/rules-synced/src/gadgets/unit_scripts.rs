use spring::UnitId;
use spring_addons::rules::Gadget;

use crate::state::Global;

#[derive(Default)]
pub struct UnitScriptsGadget;

impl UnitScriptsGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for UnitScriptsGadget {
    fn name(&self) -> &'static str {
        "UnitScripts"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        with_scripts("attaching the existing units' scripts", |host| {
            host.attach_existing();
        });
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        with_scripts("waking the unit scripts", |host| host.game_frame(frame));
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        unit: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        with_scripts("attaching a unit script", |host| {
            host.unit_created(UnitId(unit), def);
        });
        let _ = spring::rules_params::set_unit_rules_param_f32(
            unit,
            "createdFrame",
            spring::game::current_frame() as f32,
            spring::rules_params::los::PUBLIC,
        );
    }
}

fn with_scripts(what: &str, f: impl FnOnce(&mut crate::unit_scripts::FloveCusModule)) {
    if crate::with_cus_module(f).is_none() {
        spring::log_warning!(
            crate::diagnostics::LOG_SECTION,
            "{what}: the unit scripts are busy or not initialized"
        );
    }
}
