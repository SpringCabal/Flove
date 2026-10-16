mod common;
mod mushroom;
mod simple;

use alloc::rc::Rc;
use core::cell::RefCell;

use spring::UnitId;
use spring::cus::host::{CusHost, UnitDefScript};
use spring::cus::{InitCtx, Piece, ScriptCapabilities, UnitCtx, UnitScript, WeaponId, WreckLevel};

use self::mushroom::{AimState, MovementState, MushroomKind, MushroomPieces};
use self::simple::SpirePieces;

pub type FloveCusModule = CusHost<FloveUnitScript>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScriptKind {
    Mushroom(MushroomKind),
    Tree,
    Flower,
    Spire,
    Trigger,
}

fn script_kind_for_def(def: i32) -> Option<ScriptKind> {
    let defs = crate::definitions::defs();
    let mushroom = [
        (defs.normal_mushroom, MushroomKind::Normal),
        (defs.small_mushroom, MushroomKind::Small),
        (defs.big_mushroom, MushroomKind::Big),
        (defs.mushroom_cluster, MushroomKind::Cluster),
        (defs.poison_mushroom, MushroomKind::Poison),
        (defs.bomb_mushroom, MushroomKind::Bomb),
        (defs.king_shroom, MushroomKind::King),
    ]
    .into_iter()
    .find_map(|(id, kind)| (id == def).then_some(kind));
    if let Some(kind) = mushroom {
        Some(ScriptKind::Mushroom(kind))
    } else if defs.is_spire(def) {
        Some(ScriptKind::Spire)
    } else if defs.is_tree(def) {
        Some(ScriptKind::Tree)
    } else if defs.is_flower(def) || defs.is_shrub(def) {
        Some(ScriptKind::Flower)
    } else if defs.is_trigger(def) {
        Some(ScriptKind::Trigger)
    } else {
        None
    }
}

fn capabilities(kind: ScriptKind) -> ScriptCapabilities {
    let mut bits = ScriptCapabilities::KILLED;
    match kind {
        ScriptKind::Mushroom(_) => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::START_MOVING
                | ScriptCapabilities::STOP_MOVING
                | ScriptCapabilities::QUERY_WEAPON
                | ScriptCapabilities::AIM_WEAPON
                | ScriptCapabilities::AIM_FROM_WEAPON
                | ScriptCapabilities::FIRE_WEAPON
                | ScriptCapabilities::SHOT;
        }
        ScriptKind::Flower | ScriptKind::Spire => bits |= ScriptCapabilities::CREATE,
        ScriptKind::Tree | ScriptKind::Trigger => {}
    }
    ScriptCapabilities::new(bits)
}

enum ScriptState {
    Mushroom {
        kind: MushroomKind,
        pieces: MushroomPieces,
        movement: Rc<RefCell<MovementState>>,
        aiming: Rc<RefCell<AimState>>,
    },
    Tree(Piece),
    Flower,
    Spire(SpirePieces),
    Trigger,
}

pub struct FloveUnitScript {
    unit: UnitId,
    state: Option<ScriptState>,
}

impl UnitDefScript for FloveUnitScript {
    fn capabilities(def: i32) -> Option<ScriptCapabilities> {
        script_kind_for_def(def).map(capabilities)
    }

    fn for_def(def: i32, ctx: &mut InitCtx<'_>, _: &mut ()) -> Self {
        let state = script_kind_for_def(def).map(|kind| match kind {
            ScriptKind::Mushroom(kind) => {
                let pieces = MushroomPieces::from_init(ctx);
                ScriptState::Mushroom {
                    kind,
                    pieces,
                    movement: Rc::new(RefCell::new(MovementState::new(pieces))),
                    aiming: Rc::new(RefCell::new(AimState {
                        pieces,
                        weapon: WeaponId(0),
                    })),
                }
            }
            ScriptKind::Tree => ScriptState::Tree(ctx.piece("Trunk")),
            ScriptKind::Flower => ScriptState::Flower,
            ScriptKind::Spire => ScriptState::Spire(SpirePieces::from_init(ctx)),
            ScriptKind::Trigger => ScriptState::Trigger,
        });
        Self {
            unit: ctx.unit(),
            state,
        }
    }
}

impl UnitScript for FloveUnitScript {
    fn create(&mut self, ctx: &UnitCtx) {
        match &self.state {
            Some(ScriptState::Mushroom {
                kind,
                pieces,
                movement,
                ..
            }) => {
                mushroom::configure(movement, *kind, self.unit);
                mushroom::create(ctx, *pieces);
            }
            Some(ScriptState::Flower) => simple::flower_create(ctx),
            Some(ScriptState::Spire(pieces)) => simple::spire_create(ctx, pieces),
            _ => {}
        }
    }

    fn start_moving(&mut self, ctx: &UnitCtx, _reversing: bool) {
        if let Some(ScriptState::Mushroom { movement, .. }) = &self.state {
            mushroom::start_moving(ctx, Rc::clone(movement));
        }
    }

    fn stop_moving(&mut self, ctx: &UnitCtx) {
        if let Some(ScriptState::Mushroom { movement, .. }) = &self.state {
            mushroom::stop_moving(ctx, Rc::clone(movement));
        }
    }

    fn query_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) -> Piece {
        match &self.state {
            Some(ScriptState::Mushroom { kind, pieces, .. }) => {
                mushroom::query_weapon(*pieces, *kind)
            }
            _ => Piece::INVALID,
        }
    }

    fn aim_from_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) -> Piece {
        match &self.state {
            Some(ScriptState::Mushroom { kind, pieces, .. }) => {
                mushroom::aim_from_weapon(*pieces, *kind)
            }
            _ => Piece::INVALID,
        }
    }

    fn aim_weapon(
        &mut self,
        ctx: &UnitCtx,
        weapon: WeaponId,
        heading: f32,
        pitch: f32,
    ) -> Option<bool> {
        match &self.state {
            Some(ScriptState::Mushroom {
                kind,
                pieces,
                aiming,
                ..
            }) => mushroom::aim_weapon(
                ctx,
                Rc::clone(aiming),
                *pieces,
                *kind,
                weapon,
                heading,
                pitch,
            ),
            _ => None,
        }
    }

    fn fire_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) {}

    fn shot(&mut self, ctx: &UnitCtx, _weapon: WeaponId) {
        if let Some(ScriptState::Mushroom { kind, pieces, .. }) = &self.state {
            mushroom::shot(ctx, *pieces, *kind);
        }
    }

    fn killed(&mut self, ctx: &UnitCtx, _recent_damage: f32, _max_health: f32) -> WreckLevel {
        match &self.state {
            Some(ScriptState::Mushroom { kind, pieces, .. }) => {
                mushroom::killed(ctx, *pieces, *kind)
            }
            Some(ScriptState::Tree(trunk)) => simple::tree_killed(ctx, *trunk),
            Some(ScriptState::Flower) => simple::flower_killed(ctx),
            Some(ScriptState::Spire(pieces)) => simple::spire_killed(ctx, pieces),
            Some(ScriptState::Trigger) | None => simple::trigger_killed(ctx),
        }
    }
}
