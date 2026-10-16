use spring::rules_params::los;

pub use flove_shared::Status;

use crate::definitions::PLAYER_TEAM_ID;

pub fn carrier(spire_id: i32, fallback_unit_id: Option<i32>) -> i32 {
    if spire_id >= 0 {
        return spire_id;
    }
    fallback_unit_id
        .or_else(|| {
            spring::units_query::get_team_units(PLAYER_TEAM_ID)
                .ok()
                .and_then(|units| units.first().copied())
        })
        .unwrap_or(-1)
}

pub fn publish(unit: i32, status: &Status) {
    if unit < 0 {
        return;
    }
    for (name, value) in Status::PARAMS.into_iter().zip(status.values()) {
        crate::diagnostics::check(
            spring::rules_params::set_unit_rules_param_f32(unit, name, value, los::PUBLIC),
            || alloc::format!("publishing {name} on unit {unit}"),
        );
    }
}
