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
