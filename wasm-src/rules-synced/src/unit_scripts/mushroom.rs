use alloc::rc::Rc;
use core::cell::RefCell;

use spring::Float3;
use spring::UnitId;
use spring::cus::{Axis, Piece, UnitCtx, WeaponId, WreckLevel};

use super::common::{
    SIG_MOVE, emit_ceg, emit_ceg_with_params, move_piece, sleep_millis, turn, unit_position,
};

const SIG_AIM: spring::cus::SignalMask = spring::cus::SignalMask::new(2);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MushroomKind {
    Normal,
    Small,
    Big,
    Cluster,
    Poison,
    Bomb,
    King,
}

#[derive(Clone, Copy)]
pub(super) struct MushroomPieces {
    pub(super) trunk: Piece,
    pub(super) left_foot: Piece,
    pub(super) right_foot: Piece,
    pub(super) king: Piece,
    pub(super) king_head: Piece,
    pub(super) king_fl: Piece,
    pub(super) king_fr: Piece,
    pub(super) king_rl: Piece,
    pub(super) king_rr: Piece,
    pub(super) muzzle: Piece,
    pub(super) cap: Piece,
}

impl MushroomPieces {
    pub(super) fn from_init(ctx: &spring::cus::InitCtx<'_>) -> Self {
        Self {
            trunk: ctx.piece("Trunk"),
            left_foot: ctx.piece("leftFoot"),
            right_foot: ctx.piece("rightFoot"),
            king: ctx.piece("King"),
            king_head: ctx.piece("KingHead"),
            king_fl: ctx.piece("FL"),
            king_fr: ctx.piece("FR"),
            king_rl: ctx.piece("RL"),
            king_rr: ctx.piece("RR"),
            muzzle: ctx.piece("Muzzle"),
            cap: ctx.piece("Cap"),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct MovementState {
    pub(super) kind: MushroomKind,
    pub(super) pieces: MushroomPieces,
    pub(super) speed: f32,
    pub(super) moving: bool,
}

#[derive(Clone, Copy)]
pub(super) struct AimState {
    pub(super) pieces: MushroomPieces,
    pub(super) weapon: WeaponId,
}

impl MovementState {
    pub(super) fn new(pieces: MushroomPieces) -> Self {
        Self {
            kind: MushroomKind::Normal,
            pieces,
            speed: 1.0,
            moving: false,
        }
    }
}

pub(super) fn configure(
    ctx: &UnitCtx,
    state: &Rc<RefCell<MovementState>>,
    kind: MushroomKind,
    unit: UnitId,
) {
    let speed = ctx
        .with_spring(|spring| spring.move_ctrl().get_unit_move_type_data(unit.0))
        .map(|data| (data.max_speed / spring::consts::GAME_SPEED as f32).max(0.1))
        .unwrap_or(1.0);
    #[cfg(feature = "animation-trace")]
    {
        let details = alloc::format!("unit={} kind={kind:?} speed={speed:.6}", unit.0);
        let _ = ctx.with_spring(|spring| {
            spring
                .messages()
                .echo("[FloveAnimTrace] CUS_MOVEMENT_CONFIG", &details)
        });
    }
    let mut state = state.borrow_mut();
    state.kind = kind;
    state.speed = speed;
}

pub(super) fn create(ctx: &UnitCtx, pieces: MushroomPieces) {
    turn(ctx, pieces.muzzle, Axis::Y, 0.0, 0.0);
    turn(ctx, pieces.muzzle, Axis::X, 0.0, 0.0);

    turn(
        ctx,
        pieces.trunk,
        Axis::X,
        (-5.0f32).to_radians(),
        395.0f32.to_radians(),
    );
    turn(ctx, pieces.king_rr, Axis::X, 0.0, 70.0f32.to_radians());
    turn(ctx, pieces.king_fl, Axis::X, 0.0, 70.0f32.to_radians());
    turn(ctx, pieces.king_rl, Axis::X, 0.0, 70.0f32.to_radians());
    turn(ctx, pieces.king_fr, Axis::X, 0.0, 70.0f32.to_radians());
    move_piece(ctx, pieces.trunk, Axis::Y, -2.8, 0.0);
    move_piece(ctx, pieces.king, Axis::Y, -2.8, 0.0);
}

pub(super) fn start_moving(ctx: &UnitCtx, state: Rc<RefCell<MovementState>>) {
    #[cfg(feature = "animation-trace")]
    let _ = ctx.with_spring(|spring| {
        spring
            .messages()
            .echo("[FloveAnimTrace] CUS_START_MOVING", "")
    });
    state.borrow_mut().moving = true;
    ctx.signal(SIG_MOVE);
    ctx.spawn(spring::cus::TaskDefinition::with_state(
        "flove-mushroom-movement",
        state,
        movement,
    ));
}

pub(super) fn stop_moving(_ctx: &UnitCtx, state: Rc<RefCell<MovementState>>) {
    #[cfg(feature = "animation-trace")]
    let _ = _ctx.with_spring(|spring| {
        spring
            .messages()
            .echo("[FloveAnimTrace] CUS_STOP_MOVING", "")
    });
    state.borrow_mut().moving = false;
}

async fn movement(state: Rc<RefCell<MovementState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_MOVE);
    let (kind, pieces, speed) = {
        let state = state.borrow();
        (state.kind, state.pieces, state.speed)
    };
    loop {
        if !state.borrow().moving {
            return;
        }
        #[cfg(feature = "animation-trace")]
        let _ =
            ctx.with_spring(|spring| spring.messages().echo("[FloveAnimTrace] CUS_MOVE_STEP", ""));
        match kind {
            MushroomKind::King => king_step(&ctx, pieces, speed).await,
            MushroomKind::Big | MushroomKind::Cluster => {
                biped_step(&ctx, pieces, speed, true).await
            }
            _ => biped_step(&ctx, pieces, speed, false).await,
        }
        if !state.borrow().moving {
            stop(Rc::clone(&state), ctx.clone()).await;
            return;
        }
    }
}

const GAIT_DIRT: &str = "dirt";

async fn biped_step(ctx: &UnitCtx, pieces: MushroomPieces, speed: f32, heavy: bool) {
    let foot_speed = 70.0f32.to_radians() * speed;
    let sway_speed = 10.0f32.to_radians() * speed;
    let planted_height = if heavy { 4.0 } else { 0.4 };
    let plant = |ctx: &UnitCtx| {
        move_piece(ctx, pieces.trunk, Axis::Y, planted_height, 0.0);
        if heavy {
            emit_ceg(ctx, pieces.left_foot, GAIT_DIRT);
            emit_ceg(ctx, pieces.right_foot, GAIT_DIRT);
        }
    };

    emit_ceg(ctx, pieces.right_foot, GAIT_DIRT);
    turn(
        ctx,
        pieces.right_foot,
        Axis::X,
        (-60.0f32).to_radians(),
        foot_speed,
    );
    turn(
        ctx,
        pieces.left_foot,
        Axis::X,
        30.0f32.to_radians(),
        foot_speed,
    );
    turn(ctx, pieces.trunk, Axis::Z, 0.0, sway_speed);
    move_piece(ctx, pieces.trunk, Axis::Y, 2.2, 60.0);
    sleep_millis(ctx, 650.0 / speed).await;

    turn(
        ctx,
        pieces.trunk,
        Axis::Z,
        (-6.0f32).to_radians(),
        sway_speed,
    );
    plant(ctx);
    sleep_millis(ctx, 500.0 / speed).await;

    turn(ctx, pieces.trunk, Axis::Z, 6.0f32.to_radians(), sway_speed);
    turn(
        ctx,
        pieces.left_foot,
        Axis::X,
        (-60.0f32).to_radians(),
        foot_speed,
    );
    turn(
        ctx,
        pieces.right_foot,
        Axis::X,
        30.0f32.to_radians(),
        foot_speed,
    );
    move_piece(ctx, pieces.trunk, Axis::Y, 2.5, 60.0);
    sleep_millis(ctx, 658.0 / speed).await;

    plant(ctx);
    sleep_millis(ctx, 500.0 / speed).await;
}

async fn king_step(ctx: &UnitCtx, pieces: MushroomPieces, speed: f32) {
    let deg = |d: f32| d.to_radians();
    let slow_leg = deg(40.0) * speed;
    let fast_leg = deg(60.0) * speed;
    let sway = deg(10.0) * speed;
    let nod = deg(5.0) * speed;
    let head_sway = deg(12.0) * speed;

    emit_ceg(ctx, pieces.king_rr, GAIT_DIRT);
    emit_ceg(ctx, pieces.king_rl, GAIT_DIRT);
    turn(ctx, pieces.king_rl, Axis::X, deg(-40.0), slow_leg);
    turn(ctx, pieces.king_fr, Axis::X, deg(-40.0), slow_leg);
    turn(ctx, pieces.king_fl, Axis::X, deg(20.0), fast_leg);
    turn(ctx, pieces.king_rr, Axis::X, deg(20.0), fast_leg);
    turn(ctx, pieces.king, Axis::Z, 0.0, sway);
    move_piece(ctx, pieces.king, Axis::Y, 4.2, 100.0);
    sleep_millis(ctx, 650.0 / speed).await;

    turn(ctx, pieces.king_head, Axis::X, deg(3.0), nod);
    turn(ctx, pieces.king_head, Axis::Z, deg(7.0), head_sway);
    turn(ctx, pieces.king, Axis::Z, deg(-6.0), sway);
    turn(ctx, pieces.king, Axis::X, deg(2.0), sway);
    move_piece(ctx, pieces.king, Axis::Y, 2.4, 0.0);
    sleep_millis(ctx, 500.0 / speed).await;

    turn(ctx, pieces.king_head, Axis::X, deg(-4.0), nod);
    turn(ctx, pieces.king_head, Axis::Z, deg(-7.0), head_sway);
    turn(ctx, pieces.king, Axis::Z, deg(6.0), sway);
    turn(ctx, pieces.king, Axis::X, deg(-2.0), sway);
    emit_ceg(ctx, pieces.king_fr, GAIT_DIRT);
    emit_ceg(ctx, pieces.king_fl, GAIT_DIRT);
    turn(ctx, pieces.king_rr, Axis::X, deg(-40.0), fast_leg);
    turn(ctx, pieces.king_fl, Axis::X, deg(-40.0), fast_leg);
    turn(ctx, pieces.king_rl, Axis::X, deg(20.0), slow_leg);
    turn(ctx, pieces.king_fr, Axis::X, deg(20.0), slow_leg);
    move_piece(ctx, pieces.king, Axis::Y, 1.0, 100.0);
    sleep_millis(ctx, 658.0 / speed).await;

    move_piece(ctx, pieces.king, Axis::Y, 0.0, 0.0);
    sleep_millis(ctx, 500.0 / speed).await;
}

async fn stop(state: Rc<RefCell<MovementState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_MOVE);
    let (kind, pieces) = {
        let state = state.borrow();
        (state.kind, state.pieces)
    };
    match kind {
        MushroomKind::King => {
            turn(&ctx, pieces.king_rr, Axis::X, 0.0, 70.0f32.to_radians());
            turn(&ctx, pieces.king_fl, Axis::X, 0.0, 70.0f32.to_radians());
            turn(&ctx, pieces.king_rl, Axis::X, 0.0, 70.0f32.to_radians());
            turn(&ctx, pieces.king_fr, Axis::X, 0.0, 70.0f32.to_radians());
            sleep_millis(&ctx, 50.0).await;
            move_piece(&ctx, pieces.king, Axis::Y, -2.8, 0.0);
            for foot in [
                pieces.king_fr,
                pieces.king_fl,
                pieces.king_fl,
                pieces.king_fr,
            ] {
                emit_ceg(&ctx, foot, GAIT_DIRT);
            }
        }
        _ => {
            turn(
                &ctx,
                pieces.trunk,
                Axis::X,
                (-5.0f32).to_radians(),
                395.0f32.to_radians(),
            );
            sleep_millis(&ctx, 50.0).await;
            move_piece(&ctx, pieces.trunk, Axis::Y, -2.8, 0.0);
        }
    }
}

pub(super) fn query_weapon(pieces: MushroomPieces, kind: MushroomKind) -> Piece {
    if kind == MushroomKind::King {
        pieces.king
    } else {
        pieces.trunk
    }
}

pub(super) fn aim_from_weapon(pieces: MushroomPieces, kind: MushroomKind) -> Piece {
    if kind == MushroomKind::King {
        pieces.king_head
    } else {
        pieces.trunk
    }
}

pub(super) fn aim_weapon(
    ctx: &UnitCtx,
    state: Rc<RefCell<AimState>>,
    pieces: MushroomPieces,
    kind: MushroomKind,
    weapon: WeaponId,
    heading: f32,
    pitch: f32,
) -> Option<bool> {
    if kind == MushroomKind::King {
        return Some(true);
    }
    #[cfg(feature = "animation-trace")]
    {
        let aim_debug = alloc::format!("heading={heading:.6} pitch={pitch:.6}");
        let _ = ctx.with_spring(|spring| {
            spring
                .messages()
                .echo("[FloveAnimTrace] CUS_AIM_REQUEST", &aim_debug)
        });
    }
    {
        *state.borrow_mut() = AimState { pieces, weapon };
    }
    ctx.signal(SIG_AIM);
    turn(ctx, pieces.muzzle, Axis::Y, heading, 40.0f32.to_radians());
    turn(ctx, pieces.muzzle, Axis::X, -pitch, 100.0f32.to_radians());
    ctx.spawn(spring::cus::TaskDefinition::with_state(
        "flove-mushroom-aim",
        state,
        aim_task,
    ));
    None
}

async fn aim_task(state: Rc<RefCell<AimState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_AIM);
    #[cfg(feature = "animation-trace")]
    let _ = ctx.with_spring(|spring| {
        spring
            .messages()
            .echo("[FloveAnimTrace] CUS_AIM_TASK_START", "")
    });
    let request = *state.borrow();
    ctx.wait_for_turn(request.pieces.muzzle, Axis::X).await;
    #[cfg(feature = "animation-trace")]
    let _ = ctx.with_spring(|spring| {
        spring
            .messages()
            .echo("[FloveAnimTrace] CUS_AIM_X_DONE", "")
    });
    ctx.wait_for_turn(request.pieces.muzzle, Axis::Y).await;
    #[cfg(feature = "animation-trace")]
    let _ = ctx.with_spring(|spring| {
        spring
            .messages()
            .echo("[FloveAnimTrace] CUS_AIM_Y_DONE", "")
    });
    ctx.set_aim_ready(request.weapon, true);
    #[cfg(feature = "animation-trace")]
    let _ = ctx.with_spring(|spring| spring.messages().echo("[FloveAnimTrace] CUS_AIM_READY", ""));
}

pub(super) fn shot(ctx: &UnitCtx, pieces: MushroomPieces, kind: MushroomKind) {
    if kind == MushroomKind::King {
        emit_ceg(ctx, pieces.king_head, "greymuzzle");
    } else {
        emit_ceg(ctx, pieces.muzzle, "greymuzzle");
    }
}

pub(super) fn killed(ctx: &UnitCtx, pieces: MushroomPieces, kind: MushroomKind) -> WreckLevel {
    let unit = ctx.unit();
    let position = unit_position(ctx);
    let _ = ctx.with_spring(|spring| {
        spring.effects_control().spawn_ceg(
            &spring::effects_control::DefRef {
                name: ("red_pop").to_owned(),
                id: -1,
            },
            position,
            Float3::ZERO,
            0.0,
            0.0,
            0.0,
        )
    });
    match kind {
        MushroomKind::Bomb => {
            emit_ceg_with_params(
                ctx,
                pieces.trunk,
                "red_pop",
                Float3::new(0.0, 2.0, 0.0),
                10.0,
                10.0,
                0.0,
            );
            WreckLevel::None
        }
        MushroomKind::Cluster => {
            emit_ceg_with_params(
                ctx,
                pieces.cap,
                "red_pop",
                Float3::new(0.0, 2.0, 0.0),
                10.0,
                10.0,
                0.0,
            );
            let _ = ctx.with_spring(|spring| {
                spring.sound().play_sound_file(
                    "sounds/clusterexplosion.wav",
                    10.0,
                    position,
                    Float3::ZERO,
                    0,
                )
            });
            spawn_cluster_children(ctx, unit, position);
            WreckLevel::None
        }
        _ => WreckLevel::Three,
    }
}

fn spawn_cluster_children(ctx: &UnitCtx, unit: UnitId, position: Float3) {
    let def_id = ctx
        .with_spring(|spring| Ok(crate::definitions::defs(spring).small_mushroom))
        .expect("unit-script engine scope");
    let Ok(team) = ctx.with_spring(|spring| spring.units_info().get_unit_team(unit.0)) else {
        return;
    };
    const SCATTER: f32 = 300.0;
    crate::with_addon_context(|ctx| {
        ctx.commands().run(move |spring| {
            for _ in 0..10 {
                let x = position.x
                    + spring
                        .synced_random()
                        .next_float()
                        .expect("synced random float")
                        * SCATTER
                    - SCATTER / 2.0;
                let z = position.z
                    + spring
                        .synced_random()
                        .next_float()
                        .expect("synced random float")
                        * SCATTER
                    - SCATTER / 2.0;
                crate::diagnostics::check(
                    spring,
                    spring.unit_control().create_unit(
                        &spring::unit_control::DefRef {
                            name: String::new(),
                            id: def_id,
                        },
                        Float3::new(x, position.y + 20.0, z),
                        0,
                        team,
                        crate::definitions::unit_create_options(),
                    ),
                    || alloc::format!("spawning a cluster child of unit {}", unit.0),
                );
            }
        })
    });
}
