pub(crate) mod jobs;
use alloc::vec::Vec;
use core::f32::consts::TAU;

use spring::{Float3, ResultExt};

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
}

#[derive(Debug, Default)]
pub struct MushroomManager {
    target_weights: Vec<f32>,
    jobs: springcabal::jobs::Options,
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
            target_weights: Vec::new(),
            jobs: Default::default(),
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

    pub fn register_mushroom(&mut self, unit_id: i32, def_id: i32, _created_frame: i32) {
        if let Err(index) = self.ai_units.binary_search_by_key(&unit_id, |u| u.unit_id) {
            self.ai_units.insert(
                index,
                MushroomAiUnit {
                    unit_id,
                    def_id,
                    last_order_frame: 0,
                },
            );
        }
    }

    pub fn unregister_mushroom(&mut self, unit_id: i32) {
        if let Some(pos) = self.ai_units.iter().position(|u| u.unit_id == unit_id) {
            self.ai_units.remove(pos);
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
        // Lua orders existing actors before creating the next wave.
        self.update_ai(spring, frame, spire_id, trees);
        let wave = if free_play {
            self.due_wave(frame)
        } else {
            None
        };
        if let Some(wave) = wave {
            let spawn_points = self.spawn_point_units.clone();
            crate::with_addon_context(|ctx| {
                ctx.commands().run(move |spring| {
                    spawn_wave_at(spring, &spawn_points, wave.units, wave.spawn_king);
                })
            });
        }
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
                game_frame - last > repeat * FRAMES_PER_SECOND
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
        _trees: &TreeManager,
    ) {
        if !is_ai_frame(frame) {
            return;
        }
        self.order_idle(spring, frame, spire_id);
    }

    fn order_idle(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
        spire_id: i32,
    ) {
        let mut targets = None;
        let defs = crate::definitions::defs(spring);
        let prepared = if self.jobs.enabled(spring) {
            jobs::prepare(
                &mut self.jobs,
                spring,
                &self.ai_units,
                frame,
                defs.small_mushroom,
                &mut targets,
            )
        } else {
            None
        };
        for (index, ai) in self.ai_units.iter_mut().enumerate() {
            if ai.def_id == defs.small_mushroom {
                if frame - ai.last_order_frame >= 30 {
                    ai.last_order_frame = frame;
                    // Preserve Lua's private parameter as well as the local cache.
                    spring
                        .rules_params()
                        .set_unit_rules_param_f32(
                            ai.unit_id,
                            "lastOrderFrame",
                            frame as f32,
                            spring::rules_params::los::PRIVATE,
                        )
                        .log_err_once(spring, "publishing a small mushroom order frame");
                    if let Some((_target_id, target_pos)) = select_random_target(
                        spring,
                        &mut targets,
                        ai.unit_id,
                        spire_id,
                        &mut self.target_weights,
                        prepared.as_ref().map(|all| &all[index].stats),
                    ) {
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
                let has_commands = prepared.as_ref().map_or_else(
                    || {
                        spring
                            .units_commands()
                            .get_unit_command_count(ai.unit_id)
                            .log_err_once(spring, "querying mushroom orders")
                            .map(|count| count != 0)
                            .unwrap_or(false)
                    },
                    |all| !all[index].eligible,
                );

                if !has_commands {
                    if let Some((_target_id, target_pos)) = select_random_target(
                        spring,
                        &mut targets,
                        ai.unit_id,
                        spire_id,
                        &mut self.target_weights,
                        prepared.as_ref().map(|all| &all[index].stats),
                    ) {
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
    let mut units: Vec<_> = units
        .iter()
        .map(|&(kind, count)| (kind.def_id(defs), count))
        .collect();
    units.sort_unstable_by_key(|&(id, _)| id);
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

        for &(def_id, count) in &units {
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

fn collect_targets(spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<(i32, Float3)> {
    // Query the engine's indexed live roster. TreeManager admission is deferred
    // during a creation callback, so it can omit a tree born earlier this tick.
    let defs = crate::definitions::defs(spring);
    let wanted = [
        defs.tree_level1,
        defs.tree_level2,
        defs.tree_level3,
        defs.spire,
    ];
    let mut targets = Vec::new();
    for &team in &defs.target_teams {
        if spring
            .teams()
            .are_teams_allied(ENEMY_TEAM_ID, team)
            .unwrap_or(false)
        {
            continue;
        }
        for id in spring
            .units_query()
            .get_team_units_by_defs(team, &wanted)
            .log_err_once(spring, "querying mushroom targets")
            .unwrap_or_default()
        {
            if let Ok(pos) = spring
                .units_info()
                .get_unit_position(id, Default::default())
            {
                targets.push((id, pos));
            }
        }
    }
    targets.sort_unstable_by_key(|(id, _)| *id);
    targets
}

fn select_random_target(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    targets: &mut Option<Vec<(i32, Float3)>>,
    mushroom_id: i32,
    spire_id: i32,
    weights: &mut Vec<f32>,
    prepared: Option<&Option<jobs::Stats>>,
) -> Option<(i32, Float3)> {
    if spring
        .synced_random()
        .next_float()
        .expect("synced random float")
        < 0.5
    {
        let nearest = spring
            .units_query()
            .get_unit_nearest_enemy(
                mushroom_id,
                10240.0,
                spring::units_query::GetUnitNearestEnemyOptions {
                    use_los: true,
                    sphere_dist_test: false,
                    check_sight_dist: false,
                },
            )
            .log_err_once(spring, "finding a mushroom's nearest enemy")
            .unwrap_or(-1);
        // Lua returns nil immediately when the nearest-enemy query finds none.
        // Only a spire result falls through to the weighted choice.
        if nearest < 0 {
            return None;
        }
        if nearest != spire_id {
            return spring
                .units_info()
                .get_unit_position(nearest, Default::default())
                .ok()
                .map(|position| (nearest, position));
        }
    }
    // Lua snapshots targets only after the nearest-enemy shortcut needs it.
    let targets = targets.get_or_insert_with(|| collect_targets(spring));
    if let Some(stats) = prepared {
        let stats = stats.as_ref()?;
        if stats.total <= 0.0 {
            return None;
        }
        let p = spring
            .synced_random()
            .next_float()
            .expect("synced random float")
            * stats.total;
        return targets.get(stats.choose_index(p)?).copied();
    }
    let Ok(u_pos) = spring.units_info().get_unit_position(
        mushroom_id,
        spring::units_info::GetUnitPositionOptions {
            mid_pos: false,
            aim_pos: false,
        },
    ) else {
        return None;
    };

    let weight = |pos: &Float3| {
        let dx = pos.x - u_pos.x;
        let dy = pos.y - u_pos.y;
        let dz = pos.z - u_pos.z;
        let dist_sq = dx * dx + dy * dy + dz * dz;
        if dist_sq > 100.0 { 1.0 / dist_sq } else { 0.0 }
    };
    weights.clear();
    let mut total_weight = 0.0_f32;
    weights.extend(targets.iter().map(|(_, pos)| {
        total_weight += weight(pos);
        total_weight
    }));
    if total_weight <= 0.0 {
        return None;
    }

    let p = spring
        .synced_random()
        .next_float()
        .expect("synced random float")
        * total_weight;
    let index = flove_shared::choose_cumulative_index(weights.len(), |i| weights[i], p)?;
    targets.get(index).copied()
}
