#[cfg(feature = "animation-trace")]
use alloc::format;

use spring::UnitId;
use spring::cus::{Angle, Axis, Piece, UnitCtx};

pub(super) const SIG_MOVE: spring::cus::SignalMask = spring::cus::SignalMask::new(1);

pub(super) fn turn(ctx: &UnitCtx, piece: Piece, axis: Axis, target: f32, speed: f32) {
    if piece == Piece::INVALID {
        return;
    }
    if speed == 0.0 {
        ctx.turn_now(piece, axis, Angle::radians(target));
    } else {
        ctx.turn(
            piece,
            axis,
            Angle::radians(target),
            spring::cus::AngularSpeed::radians_per_second(speed),
        );
    }
}

pub(super) fn move_piece(ctx: &UnitCtx, piece: Piece, axis: Axis, target: f32, speed: f32) {
    if piece == Piece::INVALID {
        return;
    }
    #[cfg(feature = "animation-trace")]
    {
        let details = format!(
            "frame={} unit={} piece={} axis={axis:?} target={target:.3} speed={speed:.3}",
            ctx.frame(),
            ctx.unit().0,
            piece.0
        );
        let _ = spring::echo("[FloveAnimTrace] CUS_MOVE_COMMAND", &details);
    }
    if speed == 0.0 {
        ctx.move_now(piece, axis, target);
    } else {
        ctx.move_piece(piece, axis, target, speed);
    }
}

pub(super) fn emit_ceg(unit: UnitId, piece: Piece, effect: &str) {
    if let Some((position, direction)) = piece_position_and_direction(unit, piece) {
        let _ = spring::spawn_ceg(effect, position, direction, 0.0, 0.0, 0.0);
    }
}

pub(super) fn emit_ceg_with_params(
    unit: UnitId,
    piece: Piece,
    effect: &str,
    direction: spring::types::Float3,
    radius: f32,
    damage: f32,
    damage_mod: f32,
) {
    if let Some((position, _)) = piece_position_and_direction(unit, piece) {
        let _ = spring::spawn_ceg(effect, position, direction, radius, damage, damage_mod);
    }
}

fn piece_position_and_direction(
    unit: UnitId,
    piece: Piece,
) -> Option<(spring::types::Float3, spring::types::Float3)> {
    if piece == Piece::INVALID {
        return None;
    }
    let pos_dir = spring::get_unit_piece_pos_dir(unit.0, piece.0 + 1).ok()?;
    Some((pos_dir.position, pos_dir.direction))
}

pub(super) fn unit_position(unit: UnitId) -> spring::types::Float3 {
    spring::get_unit_position(unit, false, false).unwrap_or_default()
}

pub(super) fn sleep_millis(ctx: &UnitCtx, millis: f32) -> spring::cus::Sleep {
    ctx.sleep(spring::cus::Duration::from_millis(millis.max(1.0) as u64))
}
