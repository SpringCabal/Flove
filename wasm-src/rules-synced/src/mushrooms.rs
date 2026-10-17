use alloc::vec::Vec;
use core::f32::consts::TAU;

use spring::Float3;

use crate::definitions::{ENEMY_TEAM_ID, FRAMES_PER_SECOND};
use crate::spawn::create_with_random_yaw;
use crate::trees::TreeManager;
use crate::waves::{BASE_WAVES, EXTREME_WAVES, HARD_WAVES, Shroom, WaveSpec};

pub use flove_shared::{SHROOM_EVENT_KING_DEFEATED, SHROOM_EVENT_KING_SPAWNED, SHROOM_EVENT_NONE};

#[derive(Debug, Clone)]
pub struct WaveEntry {
    pub spec: &'static WaveSpec,
    pub spawned: bool,
    pub last_frame: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct MushroomAiUnit {
    pub unit_id: i32,
    pub def_id: i32,
    pub last_order_frame: i32,
    pub created_frame: i32,
}

#[derive(Debug, Default)]
pub struct MushroomManager {
    pub waves: Vec<WaveEntry>,
    pub spawn_point_units: Vec<i32>,
    pub ai_units: Vec<MushroomAiUnit>,
    pub story_mushrooms: Vec<i32>,
    pub start_spawn_frame: Option<i32>,
    pub difficulty: i32,
    pub shroom_event: i32,
}

impl MushroomManager {
    pub fn new() -> Self {
        Self {
            waves: Vec::new(),
            spawn_point_units: Vec::new(),
            ai_units: Vec::new(),
            story_mushrooms: Vec::new(),
            start_spawn_frame: None,
            difficulty: 1,
            shroom_event: SHROOM_EVENT_NONE,
        }
    }

    pub fn initialize_waves(&mut self) {
        let hard: &[WaveSpec] = if self.difficulty >= 2 {
            &HARD_WAVES
        } else {
            &[]
        };
        let extreme: &[WaveSpec] = if self.difficulty >= 3 {
            &EXTREME_WAVES
        } else {
            &[]
        };
        self.waves = BASE_WAVES
            .iter()
            .chain(hard)
            .chain(extreme)
            .map(|spec| WaveEntry {
                spec,
                spawned: false,
                last_frame: None,
            })
            .collect();
    }

    pub fn set_difficulty(&mut self, difficulty: i32) {
        self.difficulty = difficulty;
        self.initialize_waves();
    }

    pub fn register_mushroom(&mut self, unit_id: i32, def_id: i32, created_frame: i32) {
        if self.ai_units.iter().all(|u| u.unit_id != unit_id) {
            self.ai_units.push(MushroomAiUnit {
                unit_id,
                def_id,
                last_order_frame: 0,
                created_frame,
            });
        }
    }

    pub fn unregister_mushroom(&mut self, unit_id: i32) {
        if let Some(pos) = self.ai_units.iter().position(|u| u.unit_id == unit_id) {
            self.ai_units.swap_remove(pos);
        }
        if let Some(pos) = self.story_mushrooms.iter().position(|&id| id == unit_id) {
            self.story_mushrooms.swap_remove(pos);
        }
    }

    pub fn update(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
        spire_id: i32,
        trees: &TreeManager,
        free_play: bool,
    ) {
        let wave = if free_play {
            self.due_wave(frame)
        } else {
            None
        };
        let Some(wave) = wave else {
            self.update_ai(spring, frame, spire_id, trees);
            return;
        };
        let spawn_points = self.spawn_point_units.clone();
        let targets = is_ai_frame(frame).then(|| collect_targets(spring, spire_id, trees));
        crate::with_addon_context(|ctx| {
            ctx.commands().run(move |spring| {
                spawn_wave_at(spring, &spawn_points, wave.units, wave.spawn_king);
                if let Some(targets) = targets {
                    crate::with_global(|global| {
                        global
                            .mushrooms
                            .access_mut()
                            .order_idle(spring, frame, spire_id, &targets);
                    });
                }
            })
        });
    }

    fn due_wave(&mut self, frame: i32) -> Option<&'static WaveSpec> {
        let game_frame = frame - self.start_spawn_frame?;
        if game_frame < 0 {
            return None;
        }

        let mut to_spawn = None;

        for wave in &mut self.waves {
            let should_spawn = if !wave.spawned {
                game_frame >= wave.spec.time_seconds * FRAMES_PER_SECOND
            } else if let (Some(repeat), Some(last)) = (wave.spec.repeat_seconds, wave.last_frame) {
                game_frame - last >= repeat * FRAMES_PER_SECOND
            } else {
                false
            };

            if should_spawn {
                wave.spawned = true;
                wave.last_frame = Some(game_frame);
                to_spawn = Some(wave.spec);
                break;
            }
        }
        to_spawn
    }

    pub fn mark_first_wave_spawned(&mut self) {
        if let Some(wave) = self.waves.first_mut() {
            wave.spawned = true;
        }
    }

    pub fn force_spawn_wave_1(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>) {
        if let Some(wave) = self.waves.first_mut() {
            wave.spawned = true;
            let units = wave.spec.units;
            self.spawn_wave_units(spring, units, false);
        }
    }

    fn update_ai(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
        spire_id: i32,
        trees: &TreeManager,
    ) {
        if !is_ai_frame(frame) {
            return;
        }
        let targets = collect_targets(spring, spire_id, trees);
        self.order_idle(spring, frame, spire_id, &targets);
    }

    fn order_idle(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
        spire_id: i32,
        targets: &[(i32, Float3)],
    ) {
        let defs = crate::definitions::defs(spring);
        for ai in &mut self.ai_units {
            if ai.created_frame == frame {
                continue;
            }
            if ai.def_id == defs.small_mushroom {
                if frame - ai.last_order_frame >= 30 {
                    ai.last_order_frame = frame;
                    if let Some((_target_id, target_pos)) =
                        select_random_target(spring, targets, ai.unit_id, spire_id)
                    {
                        let d = 500.0f32;
                        let dx = target_pos.x
                            + spring
                                .synced_random()
                                .next_float()
                                .expect("synced random float")
                                * d
                            - d * 0.5;
                        let dz = target_pos.z
                            + spring
                                .synced_random()
                                .next_float()
                                .expect("synced random float")
                                * d
                            - d * 0.5;
                        let dy = spring.terrain().get_ground_height(dx, dz).unwrap_or(0.0);
                        order(spring, ai.unit_id, spring::cmd::MOVE, &[dx, dy, dz]);
                    }
                }
            } else {
                let has_commands = spring
                    .units_commands()
                    .get_unit_commands(ai.unit_id, 2)
                    .map(|cmds| !cmds.is_empty())
                    .unwrap_or(false);

                if !has_commands {
                    if let Some((_target_id, target_pos)) =
                        select_random_target(spring, targets, ai.unit_id, spire_id)
                    {
                        order(
                            spring,
                            ai.unit_id,
                            spring::cmd::FIGHT,
                            &[target_pos.x, target_pos.y, target_pos.z],
                        );
                    } else if let Ok(u_pos) = spring.units_info().get_unit_position(
                        ai.unit_id,
                        spring::units_info::GetUnitPositionOptions {
                            mid_pos: false,
                            aim_pos: false,
                        },
                    ) {
                        let theta = spring
                            .synced_random()
                            .next_int(1, 360)
                            .expect("synced random int") as f32
                            / 360.0
                            * TAU;
                        let d = 256.0f32;
                        let nx = u_pos.x + d * libm::sinf(theta);
                        let nz = u_pos.z + d * libm::cosf(theta);
                        let ny = spring.terrain().get_ground_height(nx, nz).unwrap_or(0.0);
                        order(spring, ai.unit_id, spring::cmd::FIGHT, &[nx, ny, nz]);
                    }
                }
            }
        }
    }

    pub fn count_living_mushrooms(&self) -> usize {
        self.ai_units.len()
    }

    pub fn spawn_story_tutorial_mushrooms(
        &self,
        _spring: &spring::Spring<'_, spring::RulesSynced>,
    ) {
        crate::with_addon_context(|ctx| {
            ctx.commands().run(|spring| {
                let defs = crate::definitions::defs(spring);
                let sx = 4688.0f32;
                let sz = 5552.0f32;
                for _ in 0..5 {
                    let x = sx - 400.0
                        + spring
                            .synced_random()
                            .next_float()
                            .expect("synced random float")
                            * 800.0;
                    let z = sz
                        + 600.0
                        + spring
                            .synced_random()
                            .next_float()
                            .expect("synced random float")
                            * 100.0;
                    if let Some(unit_id) = spawn_mushroom_unit(spring, defs.normal_mushroom, x, z) {
                        crate::with_global(|global| {
                            global.mushrooms.access_mut().story_mushrooms.push(unit_id);
                        });
                    }
                }
            })
        });
    }
}

impl MushroomManager {
    fn spawn_wave_units(
        &self,
        _spring: &spring::Spring<'_, spring::RulesSynced>,
        units: &'static [(Shroom, u32)],
        spawn_king: bool,
    ) {
        let spawn_points = self.spawn_point_units.clone();
        crate::with_addon_context(|ctx| {
            ctx.commands()
                .run(move |spring| spawn_wave_at(spring, &spawn_points, units, spawn_king))
        });
    }
}

fn spawn_wave_at(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    spawn_points: &[i32],
    units: &[(Shroom, u32)],
    spawn_king: bool,
) {
    let defs = crate::definitions::defs(spring);
    for (idx, spawn_id) in spawn_points.iter().enumerate() {
        let Ok(spawn_pos) = spring.units_info().get_unit_position(
            *spawn_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        ) else {
            continue;
        };

        if spawn_king && idx == 1 && defs.king_shroom > 0 {
            spawn_mushroom_unit(spring, defs.king_shroom, spawn_pos.x, spawn_pos.z);
        }

        for &(shroom, count) in units {
            let def_id = shroom.def_id(defs);
            if def_id <= 0 {
                continue;
            }
            for _ in 0..count {
                spawn_mushroom_unit(spring, def_id, spawn_pos.x, spawn_pos.z);
            }
        }
    }
}

fn spawn_mushroom_unit(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    def_id: i32,
    x: f32,
    z: f32,
) -> Option<i32> {
    let px = x + 10.0
        * (spring
            .synced_random()
            .next_float()
            .expect("synced random float")
            * 2.0
            - 1.0);
    let pz = z + 10.0
        * (spring
            .synced_random()
            .next_float()
            .expect("synced random float")
            * 2.0
            - 1.0);
    create_with_random_yaw(spring, def_id, px, pz, ENEMY_TEAM_ID)
}

fn is_ai_frame(frame: i32) -> bool {
    frame % 15 == 0
}

fn order(
    _spring: &spring::Spring<'_, spring::RulesSynced>,
    mushroom: i32,
    command: i32,
    params: &[f32],
) {
    crate::with_addon_context(|ctx| {
        ctx.commands()
            .give_order_to_unit(mushroom, command, params, 0, -1)
    })
    .then(move |spring, ordered| {
        crate::diagnostics::check(spring, ordered, || {
            alloc::format!("ordering mushroom {mushroom}")
        });
    });
}

fn collect_targets(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    spire_id: i32,
    trees: &TreeManager,
) -> Vec<(i32, Float3)> {
    let mut targets = Vec::new();
    if spire_id >= 0
        && let Ok(pos) = spring.units_info().get_unit_position(
            spire_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: true,
                aim_pos: false,
            },
        )
    {
        targets.push((spire_id, pos));
    }
    for tree_id in trees.trees.iter().map(|tree| tree.unit_id) {
        if let Ok(pos) = spring.units_info().get_unit_position(
            tree_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: true,
                aim_pos: false,
            },
        ) {
            targets.push((tree_id, pos));
        }
    }
    targets
}

fn select_random_target(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    targets: &[(i32, Float3)],
    mushroom_id: i32,
    spire_id: i32,
) -> Option<(i32, Float3)> {
    let Ok(u_pos) = spring.units_info().get_unit_position(
        mushroom_id,
        spring::units_info::GetUnitPositionOptions {
            mid_pos: true,
            aim_pos: false,
        },
    ) else {
        return None;
    };

    if spring
        .synced_random()
        .next_float()
        .expect("synced random float")
        < 0.5
    {
        let mut closest = None;
        let mut min_dist_sq = f32::MAX;
        for target in targets {
            let dx = target.1.x - u_pos.x;
            let dy = target.1.y - u_pos.y;
            let dz = target.1.z - u_pos.z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                closest = Some(*target);
            }
        }
        if let Some(closest) = closest
            && closest.0 != spire_id
        {
            return Some(closest);
        }
    }

    let weight = |pos: &Float3| {
        let dx = pos.x - u_pos.x;
        let dy = pos.y - u_pos.y;
        let dz = pos.z - u_pos.z;
        let dist_sq = dx * dx + dy * dy + dz * dz;
        if dist_sq > 100.0 { 1.0 / dist_sq } else { 0.0 }
    };
    let total_weight: f32 = targets.iter().map(|(_, pos)| weight(pos)).sum();
    if total_weight <= 0.0 {
        return None;
    }

    let p = spring
        .synced_random()
        .next_float()
        .expect("synced random float")
        * total_weight;
    let mut q = 0.0f32;
    for target in targets {
        q += weight(&target.1);
        if q >= p {
            return Some(*target);
        }
    }
    None
}
