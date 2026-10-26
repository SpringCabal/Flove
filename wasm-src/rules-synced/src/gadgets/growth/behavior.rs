use alloc::vec::Vec;
use core::f32::consts::TAU;

use spring::Float3;

use crate::definitions::{ALL_UNITS, PLAYER_TEAM_ID};
use crate::spawn::create_with_random_yaw;

#[derive(Debug, Clone)]
pub struct GrowthManager {
    next_spawn_frame: i32,
}

impl GrowthManager {
    pub fn starting_at(spring: &spring::Spring<'_, spring::RulesSynced>, frame: i32) -> Self {
        Self {
            next_spawn_frame: next_spawn_frame(spring, frame),
        }
    }

    #[cfg(feature = "headless-integration")]
    pub fn due_frame(&self) -> i32 {
        self.next_spawn_frame
    }

    pub fn update(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
        spire_id: i32,
    ) {
        if frame < self.next_spawn_frame {
            return;
        }
        self.next_spawn_frame = next_spawn_frame(spring, frame);
        spawn_tree(spring, spire_id);
    }
}

fn next_spawn_frame(spring: &spring::Spring<'_, spring::RulesSynced>, frame: i32) -> i32 {
    let delay = (TREE_SPAWN_INTERVAL + TREE_SPAWN_SPREAD * signed_random(spring)).max(1.0);
    frame + libm::ceilf(delay) as i32
}

pub fn spawn_trees(_spring: &spring::Spring<'_, spring::RulesSynced>, spire_id: i32, count: u32) {
    crate::with_addon_context(|ctx| {
        ctx.commands().run(move |spring| {
            for _ in 0..count {
                spawn_tree(spring, spire_id);
            }
        })
    });
}

pub fn spawn_tree(spring: &spring::Spring<'_, spring::RulesSynced>, spire_id: i32) {
    let mut order = living_trees(spring);
    shuffle(spring, &mut order);
    let spots: Vec<(f32, f32)> = order
        .into_iter()
        .filter_map(|tree| spot_near(spring, tree, spire_id))
        .collect();

    let (x, z) = if spots.is_empty() {
        FALLBACK_TREE_SPOT
    } else {
        let Some(spire) = ground_position(spring, spire_id) else {
            return;
        };
        let weights: Vec<f32> = spots
            .iter()
            .map(|&(x, z)| {
                let (dx, dz) = (spire.x - x, spire.z - z);
                let dist_sq = dx * dx + dz * dz;
                if dist_sq > 0.0 {
                    1.0 / (dist_sq * dist_sq)
                } else {
                    0.0
                }
            })
            .collect();
        let total: f32 = weights.iter().sum();
        if total <= 0.0 {
            return;
        }
        let pick = spring
            .synced_random()
            .next_float()
            .expect("synced random float")
            * total;
        let mut sum = 0.0;
        let Some(index) = weights.iter().position(|weight| {
            sum += weight;
            sum > pick
        }) else {
            return;
        };
        spots[index]
    };
    let defs = crate::definitions::defs(spring);
    if create_with_random_yaw(spring, defs.tree_level1, x, z, PLAYER_TEAM_ID).is_some() {
        spawn_grass(spring, x, z, 3, 5, MIN_TREE_DISTANCE, 10.0);
    }
}

fn living_trees(spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<i32> {
    let defs = crate::definitions::defs(spring);
    spring
        .units_query()
        .get_all_units()
        .unwrap_or_default()
        .into_iter()
        .filter(|&unit| {
            spring
                .units_info()
                .get_unit_def_id(unit)
                .is_ok_and(|def| defs.is_tree(def))
                && !spring.units_info().get_unit_is_dead(unit).unwrap_or(true)
        })
        .collect()
}

fn spot_near(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    tree: i32,
    spire_id: i32,
) -> Option<(f32, f32)> {
    let pos = ground_position(spring, tree)?;
    let step = TAU / SPOT_DIRECTIONS as f32;
    let mut theta = spring
        .synced_random()
        .next_float()
        .expect("synced random float")
        * step;
    for _ in 0..SPOT_DIRECTIONS {
        theta += step;
        let r = MIN_TREE_DISTANCE
            + spring
                .synced_random()
                .next_float()
                .expect("synced random float")
                * (MAX_TREE_DISTANCE - MIN_TREE_DISTANCE);
        let x = pos.x + r * libm::cosf(theta);
        let z = pos.z + r * libm::sinf(theta);
        if is_steep(spring, x, z) || is_too_high(spring, x, z) {
            return None;
        }
        if !is_crowded(spring, x, z, spire_id) {
            return Some((x, z));
        }
    }
    None
}

fn is_crowded(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    x: f32,
    z: f32,
    spire_id: i32,
) -> bool {
    let defs = crate::definitions::defs(spring);
    spring
        .units_query()
        .get_units_in_cylinder(x, z, MIN_TREE_DISTANCE, ALL_UNITS)
        .unwrap_or_default()
        .into_iter()
        .any(|unit| {
            unit == spire_id
                || spring
                    .units_info()
                    .get_unit_def_id(unit)
                    .is_ok_and(|def| defs.is_tree(def))
        })
}

fn shuffle(spring: &spring::Spring<'_, spring::RulesSynced>, items: &mut [i32]) {
    let len = items.len() as i32;
    for i in 0..len - 1 {
        let j = spring
            .synced_random()
            .next_int(i, len - 1)
            .expect("synced random int");
        items.swap(i as usize, j as usize);
    }
}

fn ground_position(spring: &spring::Spring<'_, spring::RulesSynced>, unit: i32) -> Option<Float3> {
    spring
        .units_info()
        .get_unit_position(
            unit,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        )
        .ok()
}

pub fn spawn_grass(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    x: f32,
    z: f32,
    min_count: i32,
    max_count: i32,
    side: f32,
    min_radius: f32,
) {
    let defs = crate::definitions::defs(spring);
    for _ in 0..spring
        .synced_random()
        .next_int(min_count, max_count)
        .expect("synced random int")
    {
        let gx = x + spring
            .synced_random()
            .next_float()
            .expect("synced random float")
            * side
            - side / 2.0;
        let gz = z + spring
            .synced_random()
            .next_float()
            .expect("synced random float")
            * side
            - side / 2.0;
        if is_steep(spring, gx, gz)
            || (min_radius > 0.0 && ((x - gx).powi(2) + (z - gz).powi(2)).sqrt() <= min_radius)
        {
            continue;
        }
        let grass = defs.grass[spring
            .synced_random()
            .next_int(0, defs.grass.len() as i32 - 1)
            .expect("synced random int") as usize];
        create_with_random_yaw(spring, grass, gx, gz, PLAYER_TEAM_ID);
    }
}

pub fn spawn_flowers(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    x: f32,
    z: f32,
    spacing: f32,
    radius: f32,
) -> Vec<i32> {
    let defs = crate::definitions::defs(spring);
    let nudge = spacing / 3.0;
    let (origin_x, origin_z) = (x - radius, z - radius);
    let mut spawned = Vec::new();
    let mut cx = origin_x;
    while cx < origin_x + 2.0 * radius {
        let mut cz = origin_z;
        while cz < origin_z + 2.0 * radius {
            cz += spacing;
            let fx = cx + nudge * signed_random(spring);
            let fz = cz + nudge * signed_random(spring);
            let (dx, dz) = (cx - x, cz - z);
            if is_steep(spring, fx, fz) || dx * dx + dz * dz > radius * radius {
                continue;
            }
            let flower = defs.flowers[spring
                .synced_random()
                .next_int(0, defs.flowers.len() as i32 - 1)
                .expect("synced random int") as usize];
            if let Some(unit_id) = create_with_random_yaw(spring, flower, fx, fz, PLAYER_TEAM_ID) {
                spawned.push(unit_id);
            }
        }
        cx += spacing;
    }
    spawned
}

fn signed_random(spring: &spring::Spring<'_, spring::RulesSynced>) -> f32 {
    spring
        .synced_random()
        .next_float()
        .expect("synced random float")
        * 2.0
        - 1.0
}

fn is_steep(spring: &spring::Spring<'_, spring::RulesSynced>, x: f32, z: f32) -> bool {
    let max_angle = libm::acosf(1.0 - 0.05) - 0.02;
    let height = |x: f32, z: f32| spring.terrain().get_ground_height(x, z).unwrap_or(0.0);
    let y = height(x, z);
    [(STEEP_PROBE, 0.0), (0.0, STEEP_PROBE), (-STEEP_PROBE, 0.0)]
        .into_iter()
        .any(|(dx, dz)| {
            let rise = height(x + dx, z + dz) - y;
            rise.abs() > 0.1 && libm::atanf(rise / STEEP_PROBE).abs() > max_angle
        })
}

fn is_too_high(spring: &spring::Spring<'_, spring::RulesSynced>, x: f32, z: f32) -> bool {
    !spring
        .terrain()
        .get_ground_height(x, z)
        .is_ok_and(|y| y <= MAX_TREE_HEIGHT)
}

const TREE_SPAWN_INTERVAL: f32 = 5.0 * spring::consts::GAME_SPEED as f32;
const TREE_SPAWN_SPREAD: f32 = 30.0;
pub const MIN_TREE_DISTANCE: f32 = 225.0;
const MAX_TREE_DISTANCE: f32 = 325.0;
const SPOT_DIRECTIONS: i32 = 5;
const FALLBACK_TREE_SPOT: (f32, f32) = (4688.0 + 200.0, 5552.0);
const MAX_TREE_HEIGHT: f32 = 1600.0;
const STEEP_PROBE: f32 = 5.0;
