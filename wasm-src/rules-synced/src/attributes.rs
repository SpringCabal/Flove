use alloc::vec::Vec;
use spring::ResultExt;
use spring::move_ctrl::MoveTypeNumericField as Field;

const SPEED_FACTOR: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Movement {
    speed: f32,
    reverse_speed: f32,
    turn_rate: f32,
    turn_accel: f32,
    acc_rate: f32,
    dec_rate: f32,
}

impl Movement {
    fn base(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> Option<Self> {
        let def = spring.units_info().get_unit_def_id(unit_id).ok()?;
        let param = |key: &str| {
            spring
                .unit_defs()
                .get_unit_def_param_float(def, key)
                .log_err_once(spring, "reading a unit def's movement")
        };
        let reverse_speed = spring
            .move_ctrl()
            .get_unit_move_type_data(unit_id)
            .log_err_once(spring, "reading a unit's move type")?
            .max_reverse_speed;
        let turn_rate = param("turnRate")?;
        let turn_accel = spring
            .unit_defs()
            .get_unit_def_custom_param(def, "turnaccel")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(turn_rate);
        Some(Self {
            speed: spring.unit_defs().get_unit_def_speed(def).ok()? * SPEED_FACTOR,
            reverse_speed,
            turn_rate,
            turn_accel,
            acc_rate: param("maxAcc")?,
            dec_rate: param("maxDec")?,
        })
    }

    fn scaled(&self, factor: f32) -> Self {
        if factor >= 1.0 {
            return *self;
        }
        let factor = factor.max(0.0);
        Self {
            speed: self.speed * factor,
            reverse_speed: 0.0,
            turn_rate: self.turn_rate * factor.max(0.001),
            turn_accel: self.turn_accel * factor,
            acc_rate: (self.acc_rate * factor * factor.max(0.001)).min(factor),
            dec_rate: self.dec_rate * 1000.0,
        }
    }

    fn apply(&self, spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) {
        for (field, value) in [
            (Field::MoveTypeMaxSpeed, self.speed),
            (Field::MoveTypeMaxWantedSpeed, self.speed),
            (Field::MoveTypeGroundMaxReverseSpeed, self.reverse_speed),
            (Field::MoveTypeGroundTurnRate, self.turn_rate),
            (Field::MoveTypeGroundTurnAccel, self.turn_accel),
            (Field::MoveTypeGroundAccRate, self.acc_rate),
            (Field::MoveTypeGroundDecRate, self.dec_rate),
        ] {
            spring
                .move_ctrl()
                .set_move_type_numeric(unit_id, field, value)
                .log_err_once(spring, "setting a unit's movement");
        }
    }
}

pub fn apply_base_movement(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) {
    if let Some(movement) = Movement::base(spring, unit_id) {
        movement.apply(spring, unit_id);
    }
}

#[derive(Debug, Default)]
pub struct AttributeManager {
    units: Vec<(i32, Movement)>,
}

impl AttributeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_speed_multiplier(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        unit_id: i32,
        factor: f32,
    ) {
        let base = match self.units.iter().find(|(unit, _)| *unit == unit_id) {
            Some((_, base)) => *base,
            None => {
                let Some(base) = Movement::base(spring, unit_id) else {
                    return;
                };
                self.units.push((unit_id, base));
                base
            }
        };
        base.scaled(factor).apply(spring, unit_id);
        if factor >= 1.0 {
            self.remove_unit(unit_id);
        }
    }

    pub fn remove_unit(&mut self, unit_id: i32) {
        self.units.retain(|(unit, _)| *unit != unit_id);
    }
}
