use spring::rules_params::{RulesParamType, RulesParamValue, los};

pub use flove_shared::Status;

pub fn publish(spring: &spring::Spring<'_, spring::RulesSynced>, name: &str, value: f32) -> bool {
    crate::diagnostics::check(
        spring,
        spring.rules_params().set_game_rules_param(
            name,
            &RulesParamValue {
                type_: RulesParamType::RulesparamTypeFloat,
                float_value: value,
                bool_value: false,
                string_value: alloc::string::String::new(),
            },
            los::PUBLIC,
        ),
        || alloc::format!("publishing {name} as a game rules parameter"),
    )
    .unwrap_or(false)
}

// Lua updates power when a tree enters/leaves the live roster. Keep the HUD
// publication at that same boundary instead of waiting for the next frame.
pub fn publish_power(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &crate::state::Global,
) {
    let power = global.trees.access().total_power() as f32;
    let index = Status::PARAMS
        .iter()
        .position(|&name| name == "power")
        .expect("power status field");
    global.published_status[index].send_if_changed(power, |value| publish(spring, "power", *value));
}
