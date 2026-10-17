#[cfg(feature = "animation-trace")]
use alloc::format;

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
        let _ = ctx.with_spring(|spring| {
            spring
                .messages()
                .echo("[FloveAnimTrace] CUS_MOVE_COMMAND", &details)
        });
    }
    if speed == 0.0 {
        ctx.move_now(piece, axis, target);
    } else {
        ctx.move_piece(piece, axis, target, speed);
    }
}

pub(super) fn emit_ceg(ctx: &UnitCtx, piece: Piece, effect: &str) {
    if let Some((position, direction)) = piece_position_and_direction(ctx, piece) {
        let _ = ctx.with_spring(|spring| {
            spring.effects_control().spawn_ceg(
                &spring::effects_control::DefRef {
                    name: (effect).to_owned(),
                    id: -1,
                },
                position,
                direction,
                0.0,
                0.0,
                0.0,
            )
        });
    }
}

pub(super) fn emit_ceg_with_params(
    ctx: &UnitCtx,
    piece: Piece,
    effect: &str,
    direction: spring::Float3,
    radius: f32,
    damage: f32,
    damage_mod: f32,
) {
    if let Some((position, _)) = piece_position_and_direction(ctx, piece) {
        let _ = ctx.with_spring(|spring| {
            spring.effects_control().spawn_ceg(
                &spring::effects_control::DefRef {
                    name: (effect).to_owned(),
                    id: -1,
                },
                position,
                direction,
                radius,
                damage,
                damage_mod,
            )
        });
    }
}

fn piece_position_and_direction(
    ctx: &UnitCtx,
    piece: Piece,
) -> Option<(spring::Float3, spring::Float3)> {
    if piece == Piece::INVALID {
        return None;
    }
    let pos_dir = ctx
        .with_spring(|spring| {
            spring
                .units_pieces()
                .get_unit_piece_pos_dir(ctx.unit().0, piece.0 + 1)
        })
        .ok()?;
    Some((pos_dir.position, pos_dir.direction))
}

pub(super) fn unit_position(ctx: &UnitCtx) -> spring::Float3 {
    ctx.with_spring(|spring| {
        spring
            .units_info()
            .get_unit_position(ctx.unit().0, Default::default())
    })
    .unwrap_or_default()
}

pub(super) fn sleep_millis(ctx: &UnitCtx, millis: f32) -> spring::cus::Sleep {
    ctx.sleep(spring::cus::Duration::from_millis(millis.max(1.0) as u64))
}
