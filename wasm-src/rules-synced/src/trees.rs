use alloc::vec::Vec;
use spring::commands;
use spring::random::synced as random;
use spring::rules_params::los;
use spring::types::Float3;

use crate::definitions::PLAYER_TEAM_ID;

#[derive(Debug, Clone)]
pub struct Wisp {
    pub stage: i32,
    pub stage_frame: i32,
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub unit_id: i32,
    pub level: i32,
    pub upgrade_progress: i32,
    pub create_wisp_frame: i32,
    pub wisps: Vec<Wisp>,
}

#[derive(Debug, Default)]
pub struct TreeManager {
    pub trees: Vec<Tree>,
}

impl TreeManager {
    pub fn new() -> Self {
        Self { trees: Vec::new() }
    }

    pub fn add_tree(&mut self, unit_id: i32, level: i32, frame: i32) {
        if self.trees.iter().any(|t| t.unit_id == unit_id) {
            return;
        }
        let config = tree_config(level);
        let next_wisp = frame + random::int(config.min_wisp_interval, config.max_wisp_interval);
        self.trees.push(Tree {
            unit_id,
            level,
            upgrade_progress: 0,
            create_wisp_frame: next_wisp,
            wisps: Vec::new(),
        });
        let _ = spring::set_unit_rules_param_f32(unit_id, "upgradeProgress", 0.0, los::PUBLIC);
    }

    pub fn remove_tree(&mut self, unit_id: i32) -> bool {
        if let Some(pos) = self.trees.iter().position(|t| t.unit_id == unit_id) {
            self.trees.swap_remove(pos);
            true
        } else {
            false
        }
    }

    pub fn trees_near_sorted(&self, pos: spring::types::Float3, radius: f32) -> Vec<i32> {
        let mut found: Vec<(f32, i32)> = Vec::new();
        for tree in &self.trees {
            let Ok(tree_pos) = spring::units_info::get_unit_position(
                tree.unit_id,
                spring::units_info::GetUnitPositionOptions {
                    mid_pos: false,
                    aim_pos: false,
                },
            ) else {
                continue;
            };
            let dx = tree_pos.x - pos.x;
            let dz = tree_pos.z - pos.z;
            if dx * dx + dz * dz > radius * radius {
                continue;
            }
            let dy = tree_pos.y - pos.y;
            found.push((dx * dx + dy * dy + dz * dz, tree.unit_id));
        }
        found.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        found.into_iter().map(|(_, id)| id).collect()
    }

    pub fn find_tree_near(&self, x: f32, z: f32, max_radius: f32) -> Option<i32> {
        let mut closest = None;
        let mut min_dist_sq = max_radius * max_radius;
        for tree in &self.trees {
            if let Ok(pos) = spring::units_info::get_unit_position(
                tree.unit_id,
                spring::units_info::GetUnitPositionOptions {
                    mid_pos: true,
                    aim_pos: false,
                },
            ) {
                let dx = pos.x - x;
                let dz = pos.z - z;
                let dist_sq = dx * dx + dz * dz;
                if dist_sq < min_dist_sq {
                    min_dist_sq = dist_sq;
                    closest = Some(tree.unit_id);
                }
            }
        }
        closest
    }

    pub fn is_upgradable(&self, unit_id: i32) -> bool {
        self.trees
            .iter()
            .any(|t| t.unit_id == unit_id && t.level < MAX_TREE_LEVEL)
    }

    pub fn total_power(&self) -> i32 {
        self.trees.iter().map(|t| tree_config(t.level).power).sum()
    }

    pub fn update_wisps(&mut self, frame: i32, mana: &mut i32) {
        let mut mana_gained = 0;
        for tree in &mut self.trees {
            let config = tree_config(tree.level);

            for (i, wisp) in tree.wisps.iter_mut().enumerate() {
                if frame - wisp.stage_frame >= WISP_STAGE_DURATION {
                    wisp.stage_frame = frame;
                    if wisp.stage >= WISP_TOTAL_STAGES {
                        wisp.stage = 1;
                        mana_gained += 1;
                    } else {
                        wisp.stage += 1;
                    }
                    let key = alloc::format!("wispStage{}", i + 1);
                    let _ = spring::set_unit_rules_param_f32(
                        tree.unit_id,
                        &key,
                        wisp.stage as f32,
                        los::PUBLIC,
                    );
                }
            }

            if frame >= tree.create_wisp_frame && tree.wisps.len() < config.max_wisps as usize {
                tree.create_wisp_frame =
                    frame + random::int(config.min_wisp_interval, config.max_wisp_interval);
                let wisp_idx = tree.wisps.len() + 1;
                let offset = random::float() * 0.25;
                tree.wisps.push(Wisp {
                    stage: 1,
                    stage_frame: frame,
                });
                let stage_key = alloc::format!("wispStage{}", wisp_idx);
                let offset_key = alloc::format!("wispOffset{}", wisp_idx);
                let _ =
                    spring::set_unit_rules_param_f32(tree.unit_id, &stage_key, 1.0, los::PUBLIC);
                let _ = spring::set_unit_rules_param_f32(
                    tree.unit_id,
                    &offset_key,
                    offset,
                    los::PUBLIC,
                );
                let _ = spring::set_unit_rules_param_f32(
                    tree.unit_id,
                    "wispCount",
                    tree.wisps.len() as f32,
                    los::PUBLIC,
                );
            }
        }

        if mana_gained > 0 {
            *mana += mana_gained;
        }
    }

    pub fn add_upgrade_progress(&mut self, unit_id: i32, frame: i32, pro_id: i32) -> bool {
        let Some(pos) = self.trees.iter().position(|t| t.unit_id == unit_id) else {
            crate::diagnostics::trace!(
                "[upgrade] ignored     f={frame} shot={pro_id} tree={unit_id} (not a tracked tree)"
            );
            return false;
        };
        if self.trees[pos].level >= MAX_TREE_LEVEL {
            crate::diagnostics::trace!(
                "[upgrade] ignored     f={frame} shot={pro_id} tree={unit_id} (already max level)"
            );
            return false;
        }

        self.trees[pos].upgrade_progress += 1;
        let progress = self.trees[pos].upgrade_progress;
        let current_level = self.trees[pos].level;
        let required = UPGRADE_PROGRESS_PER_LEVEL * current_level;

        if progress >= required {
            let _ = spring::set_unit_rules_param_f32(unit_id, "upgradeProgress", 0.0, los::PUBLIC);
            self.promote_tree(pos, frame);
        } else {
            let _ = spring::set_unit_rules_param_f32(
                unit_id,
                "upgradeProgress",
                progress as f32,
                los::PUBLIC,
            );
        }
        crate::diagnostics::trace!(
            "[upgrade] tree charged f={frame} shot={pro_id} tree={unit_id} level={current_level} charge={progress}/{required}"
        );
        true
    }

    pub fn get_tree_ids(&self) -> Vec<i32> {
        self.trees.iter().map(|t| t.unit_id).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TreeConfig {
    pub power: i32,
    pub max_wisps: u32,
    pub min_wisp_interval: i32,
    pub max_wisp_interval: i32,
}

pub fn tree_config(level: i32) -> TreeConfig {
    match level {
        1 => TreeConfig {
            power: 3,
            max_wisps: 3,
            min_wisp_interval: 66,
            max_wisp_interval: 99,
        },
        2 => TreeConfig {
            power: 15,
            max_wisps: 7,
            min_wisp_interval: 50,
            max_wisp_interval: 82,
        },
        _ => TreeConfig {
            power: 50,
            max_wisps: 12,
            min_wisp_interval: 33,
            max_wisp_interval: 66,
        },
    }
}

impl TreeManager {
    fn promote_tree(&mut self, index: usize, frame: i32) {
        let defs = crate::definitions::defs();
        let old_id = self.trees[index].unit_id;
        let next_level = self.trees[index].level + 1;

        let Ok(pos) = spring::units_info::get_unit_position(
            old_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        ) else {
            return;
        };

        let target_def = if next_level == 2 {
            defs.tree_level2
        } else {
            defs.tree_level3
        };

        let team = spring::units_info::get_unit_team(old_id).unwrap_or(PLAYER_TEAM_ID);
        let front = spring::units_info::get_unit_direction(old_id).ok();

        let _ = spring::set_unit_rules_param_f32(old_id, "upgradeProgress", 0.0, los::PUBLIC);
        commands::run(move || {
            crate::diagnostics::check(
                spring::destroy_unit(
                    old_id,
                    spring::owned::unit_control::DestroyUnitOptions {
                        selfd: false,
                        reclaimed: true,
                        attacker_id: -1,
                        recycle_id: false,
                    },
                ),
                || alloc::format!("replacing tree {old_id}"),
            );

            if target_def > 0
                && let Some(new_unit_id) = crate::diagnostics::check(
                    spring::create_unit(
                        spring::UnitDefRef {
                            name: "",
                            id: target_def,
                        },
                        pos,
                        team,
                        crate::definitions::unit_create_options(),
                    ),
                    || alloc::format!("growing tree {old_id} to level {next_level}"),
                )
            {
                if let Some(front) = front {
                    let _ = spring::unit_control::set_unit_direction(
                        new_unit_id,
                        front,
                        right_from_front(front),
                    );
                }
                crate::diagnostics::trace!(
                    "[upgrade] TREE UPGRADED f={frame} old={old_id} new={new_unit_id} level={next_level} team={team}"
                );
            }
        });
    }
}

fn right_from_front(front: Float3) -> Float3 {
    let aux = if libm::fabsf(front.y) >= 0.999 {
        Float3::new(0.0, 0.0, -1.0)
    } else {
        Float3::new(0.0, 1.0, 0.0)
    };
    let right = Float3::new(
        front.y * aux.z - front.z * aux.y,
        front.z * aux.x - front.x * aux.z,
        front.x * aux.y - front.y * aux.x,
    );
    let len = libm::sqrtf(right.x * right.x + right.y * right.y + right.z * right.z);
    if len <= 1e-6 {
        return Float3::new(1.0, 0.0, 0.0);
    }
    Float3::new(right.x / len, right.y / len, right.z / len)
}

pub const MAX_TREE_LEVEL: i32 = 3;
pub const WISP_STAGE_DURATION: i32 = 165;
pub const WISP_TOTAL_STAGES: i32 = 3;
pub const UPGRADE_PROGRESS_PER_LEVEL: i32 = 3;
