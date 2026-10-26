use spring::{ResultExt, UnitId};
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
        crate::with_cus_module(|host, spring| host.attach_existing(spring))
            .log_err_once(_ctx.spring(), "attaching existing unit scripts");
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        crate::with_cus_module(|host, _| {
            host.game_frame(frame);
            Ok(())
        })
        .log_err_once(_ctx.spring(), "waking unit scripts");
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        unit: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        crate::definitions::defs(_ctx.spring()).configure_geometry(_ctx.spring(), unit, def);
        crate::with_cus_module(|host, spring| host.unit_created(spring, UnitId(unit), def))
            .log_err_once(_ctx.spring(), "attaching a unit script");
        let spring = _ctx.spring();
        let _ = spring.rules_params().set_unit_rules_param_f32(
            unit,
            "createdFrame",
            spring.game().current_frame().unwrap_or(0) as f32,
            spring::rules_params::los::PUBLIC,
        );
    }
}
