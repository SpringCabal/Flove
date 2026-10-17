use alloc::rc::Rc;
use core::cell::RefCell;

use spring::ResultExt;
use spring::cus::{Axis, InitCtx, TaskDefinition, UnitCtx, WreckLevel};

use super::common::{emit_ceg, unit_position};

pub(super) fn flower_create(ctx: &UnitCtx) {
    ctx.with_spring(|spring| {
        spring
            .unit_control()
            .set_unit_collision_volume_data(
                ctx.unit().0,
                spring::Float3::ZERO,
                spring::Float3::ZERO,
                -1,
                0,
                0,
            )
            .log_err_once(spring, "removing a flower's collision volume");
        Ok(())
    })
    .expect("active unit-script scope");
    let position = unit_position(ctx);
    let _ = ctx.with_spring(|spring| {
        spring.effects_control().spawn_ceg(
            &spring::effects_control::DefRef {
                name: ("dirtfling").to_owned(),
                id: -1,
            },
            position,
            spring::Float3::ZERO,
            0.0,
            0.0,
            0.0,
        )
    });
}

pub(super) fn flower_killed(ctx: &UnitCtx) -> WreckLevel {
    let position = unit_position(ctx);
    let _ = ctx.with_spring(|spring| {
        spring.effects_control().spawn_ceg(
            &spring::effects_control::DefRef {
                name: ("dirtfling").to_owned(),
                id: -1,
            },
            position,
            spring::Float3::ZERO,
            0.0,
            0.0,
            0.0,
        )
    });
    WreckLevel::None
}

pub(super) fn tree_killed(ctx: &UnitCtx, trunk: spring::cus::Piece) -> WreckLevel {
    let position = unit_position(ctx);
    let _ = ctx.with_spring(|spring| {
        spring.effects_control().spawn_ceg(
            &spring::effects_control::DefRef {
                name: ("blackpop").to_owned(),
                id: -1,
            },
            position,
            spring::Float3::new(0.0, 2.0, 0.0),
            10.0,
            10.0,
            0.0,
        )
    });
    if trunk != spring::cus::Piece::INVALID {
        ctx.turn(
            trunk,
            Axis::Z,
            spring::cus::Angle::radians(2.0),
            spring::cus::AngularSpeed::radians_per_second(1.0),
        );
    }
    WreckLevel::None
}

pub(super) fn trigger_killed(_ctx: &UnitCtx) -> WreckLevel {
    WreckLevel::None
}

pub(super) fn spire_create(ctx: &UnitCtx, init: &SpirePieces) {
    for piece in init.leaves.iter().chain(init.branches.iter()) {
        if *piece != spring::cus::Piece::INVALID {
            ctx.hide(*piece);
        }
    }
    let _ = ctx.with_spring(|spring| {
        spring.rules_params().set_unit_rules_param_f32(
            ctx.unit().0,
            "branches",
            0.0,
            spring::rules_params::los::PRIVATE,
        )
    });
    ctx.spawn(TaskDefinition::with_state(
        "flove-spire-branches",
        Rc::new(RefCell::new(BranchState {
            pieces: *init,
            visible: 0,
        })),
        spire_branches,
    ));
}

pub(super) fn spire_killed(ctx: &UnitCtx, pieces: &SpirePieces) -> WreckLevel {
    let position = unit_position(ctx);
    let _ = ctx.with_spring(|spring| {
        spring.effects_control().spawn_ceg(
            &spring::effects_control::DefRef {
                name: ("blackpop").to_owned(),
                id: -1,
            },
            position,
            spring::Float3::new(0.0, 2.0, 0.0),
            10.0,
            10.0,
            0.0,
        )
    });
    if pieces.stalk != spring::cus::Piece::INVALID {
        ctx.turn(
            pieces.stalk,
            Axis::Z,
            spring::cus::Angle::radians(4.0),
            spring::cus::AngularSpeed::radians_per_second(1.0),
        );
    }
    WreckLevel::None
}

#[derive(Clone, Copy)]
pub(super) struct SpirePieces {
    pub(super) stalk: spring::cus::Piece,
    pub(super) leaves: [spring::cus::Piece; 9],
    pub(super) branches: [spring::cus::Piece; 9],
}

impl SpirePieces {
    pub(super) fn from_init(ctx: &InitCtx<'_>) -> Self {
        let mut leaves = [spring::cus::Piece::INVALID; 9];
        let mut branches = [spring::cus::Piece::INVALID; 9];
        for (index, piece) in leaves.iter_mut().enumerate() {
            *piece = ctx.piece(piece_name("Leaves", index));
        }
        for (index, piece) in branches.iter_mut().enumerate() {
            *piece = ctx.piece(piece_name("Branch", index));
        }
        Self {
            stalk: ctx.piece("Stalk"),
            leaves,
            branches,
        }
    }
}

#[derive(Clone, Copy)]
struct BranchState {
    pieces: SpirePieces,
    visible: i32,
}

async fn spire_branches(state: Rc<RefCell<BranchState>>, ctx: UnitCtx) {
    loop {
        let desired = ctx
            .with_spring(|spring| {
                spring
                    .rules_params()
                    .get_unit_rules_param_f32(ctx.unit().0, "desiredBranches")
            })
            .unwrap_or(0.0)
            .round()
            .clamp(0.0, 9.0) as i32;
        let current = state.borrow().visible;

        if current < desired {
            let next = current + 1;
            let pieces = state.borrow().pieces;
            state.borrow_mut().visible = next;
            let _ = ctx.with_spring(|spring| {
                spring.rules_params().set_unit_rules_param_f32(
                    ctx.unit().0,
                    "branches",
                    next as f32,
                    spring::rules_params::los::PRIVATE,
                )
            });
            emit_ceg(&ctx, pieces.branches[current as usize], "dirtfling");
            if pieces.branches[current as usize] != spring::cus::Piece::INVALID {
                ctx.show(pieces.branches[current as usize]);
            }
            ctx.sleep(spring::cus::Duration::from_millis(500)).await;
            emit_ceg(&ctx, pieces.leaves[current as usize], "dirtfling");
            if pieces.leaves[current as usize] != spring::cus::Piece::INVALID {
                ctx.show(pieces.leaves[current as usize]);
            }
        } else if current > desired {
            let index = current - 1;
            let pieces = state.borrow().pieces;
            state.borrow_mut().visible = index;
            let _ = ctx.with_spring(|spring| {
                spring.rules_params().set_unit_rules_param_f32(
                    ctx.unit().0,
                    "branches",
                    index as f32,
                    spring::rules_params::los::PRIVATE,
                )
            });
            emit_ceg(&ctx, pieces.leaves[index as usize], "dirtfling");
            if pieces.leaves[index as usize] != spring::cus::Piece::INVALID {
                ctx.hide(pieces.leaves[index as usize]);
            }
            ctx.sleep(spring::cus::Duration::from_millis(500)).await;
            emit_ceg(&ctx, pieces.branches[index as usize], "dirtfling");
            if pieces.branches[index as usize] != spring::cus::Piece::INVALID {
                ctx.hide(pieces.branches[index as usize]);
            }
        } else {
            ctx.sleep_frames(1).await;
        }
    }
}

fn piece_name(prefix: &str, index: usize) -> &'static str {
    match (prefix, index) {
        ("Leaves", 0) => "Leaves1",
        ("Leaves", 1) => "Leaves2",
        ("Leaves", 2) => "Leaves3",
        ("Leaves", 3) => "Leaves4",
        ("Leaves", 4) => "Leaves5",
        ("Leaves", 5) => "Leaves6",
        ("Leaves", 6) => "Leaves7",
        ("Leaves", 7) => "Leaves8",
        ("Leaves", 8) => "Leaves9",
        ("Branch", 0) => "Branch1",
        ("Branch", 1) => "Branch2",
        ("Branch", 2) => "Branch3",
        ("Branch", 3) => "Branch4",
        ("Branch", 4) => "Branch5",
        ("Branch", 5) => "Branch6",
        ("Branch", 6) => "Branch7",
        ("Branch", 7) => "Branch8",
        _ => "Branch9",
    }
}
