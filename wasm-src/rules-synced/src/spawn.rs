use alloc::vec::Vec;
use core::f32::consts::PI;

use spring::Float3;

use crate::definitions::{ENEMY_TEAM_ID, PLAYER_TEAM_ID};
use crate::gadgets::growth::behavior::spawn_grass;
use crate::spawn_data::{INITIAL_UNITS, Marker};

const SPIRE_AIM_HEIGHT: f32 = 200.0;

pub fn populate_world(spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<i32> {
    let defs = crate::definitions::defs(spring);
    reveal_map(spring);

    let mut spawn_point_units = Vec::new();

    for point in &INITIAL_UNITS {
        let def_id = match point.marker {
            Marker::Spire => defs.spire,
            Marker::Base => defs.base,
            Marker::Expansion1 => defs.expansion1,
            Marker::Expansion2 => defs.expansion2,
            Marker::Expansion3 => defs.expansion3,
            Marker::MushroomSpawn => defs.mushroom_spawn,
        };
        if def_id <= 0 {
            continue;
        }
        // All scenario markers belong to the player; enemies spawn later.
        let team = PLAYER_TEAM_ID;

        if let Some(unit_id) = crate::diagnostics::check(
            spring,
            spring.unit_control().create_unit(
                &spring::unit_control::DefRef {
                    name: String::new(),
                    id: def_id,
                },
                Float3::new(point.x, point.y, point.z),
                0,
                team,
                crate::definitions::unit_create_options(),
            ),
            || alloc::format!("spawning the {:?} marker", point.marker),
        ) {
            if point.marker == Marker::Spire {
                let aim = Float3::new(0.0, SPIRE_AIM_HEIGHT, 0.0);
                crate::diagnostics::check(
                    spring,
                    spring
                        .unit_control()
                        .set_unit_mid_and_aim_pos(unit_id, aim, aim, true),
                    || alloc::format!("moving the aim point of spire {unit_id}"),
                );
            } else {
                let _ = spring.unit_control().set_unit_neutral(unit_id, true);
                let _ = spring.unit_control().set_unit_blocking(
                    unit_id,
                    spring::unit_control::SetUnitBlockingOptions {
                        blocking: false,
                        solid_objects: false,
                        projectiles: false,
                        quad_map_rays: false,
                        crushable: false,
                        block_enemy_pushing: false,
                        block_height_changes: false,
                    },
                );
                if point.marker == Marker::MushroomSpawn {
                    spawn_point_units.push(unit_id);
                }
            }
        }
    }

    spawn_point_units
}

/// The original grove and mushroom gadgets populate on their fourth tick,
/// with every base grove before the tunnel tubes. Keep the shared RNG order.
pub fn populate_decorations(spring: &spring::Spring<'_, spring::RulesSynced>) {
    let defs = crate::definitions::defs(spring);
    let mut bases = Vec::new();
    let mut spawns = Vec::new();
    for unit in spring.units_query().get_all_units().unwrap_or_default() {
        let Ok(def) = spring.units_info().get_unit_def_id(unit) else {
            continue;
        };
        if def != defs.base && def != defs.mushroom_spawn {
            continue;
        }
        let Ok(position) = spring
            .units_info()
            .get_unit_position(unit, Default::default())
        else {
            continue;
        };
        if def == defs.base {
            bases.push(position);
        } else {
            spawns.push(position);
        }
    }
    for position in bases {
        spawn_base_tree(spring, position.x, position.z);
    }
    for position in spawns {
        spawn_tunnel_tubes(spring, position.x, position.z);
    }
}

fn reveal_map(spring: &spring::Spring<'_, spring::RulesSynced>) {
    for ally_team in 0..3 {
        let _ = spring.team_control().set_global_los(ally_team, true);
    }
}

fn spawn_base_tree(spring: &spring::Spring<'_, spring::RulesSynced>, x: f32, z: f32) {
    let defs = crate::definitions::defs(spring);
    if defs.tree_level1 <= 0 {
        return;
    }
    if create_with_random_yaw(spring, defs.tree_level1, x, z, PLAYER_TEAM_ID).is_some() {
        spawn_grass(spring, x, z, 8, 12, 500.0, 0.0);
    }
}

fn spawn_tunnel_tubes(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    center_x: f32,
    center_z: f32,
) {
    let defs = crate::definitions::defs(spring);
    let tube_radius = 800.0f32;
    for _ in 0..12 {
        let tube_idx = spring
            .synced_random()
            .next_int(0, 2)
            .expect("synced random int") as usize;
        let tube_def = defs.tunnel_tubes[tube_idx];
        if tube_def > 0 {
            let ux = center_x
                + spring
                    .synced_random()
                    .next_float()
                    .expect("synced random float")
                    * tube_radius
                - tube_radius * 0.5;
            let uz = center_z
                + spring
                    .synced_random()
                    .next_float()
                    .expect("synced random float")
                    * tube_radius
                - tube_radius * 0.5;
            let ux = ux
                + 10.0
                    * (2.0
                        * spring
                            .synced_random()
                            .next_float()
                            .expect("synced random float")
                        - 1.0);
            let uz = uz
                + 10.0
                    * (2.0
                        * spring
                            .synced_random()
                            .next_float()
                            .expect("synced random float")
                        - 1.0);
            create_with_random_yaw(spring, tube_def, ux, uz, ENEMY_TEAM_ID);
        }
    }
}

pub fn create_with_random_yaw(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    def_id: i32,
    x: f32,
    z: f32,
    team: i32,
) -> Option<i32> {
    let y = spring
        .terrain()
        .get_ground_height(x, z)
        .expect("ground height for a spawned decoration");
    let unit_id = crate::diagnostics::check(
        spring,
        spring.unit_control().create_unit(
            &spring::unit_control::DefRef {
                name: String::new(),
                id: def_id,
            },
            Float3::new(x, y, z),
            0,
            team,
            crate::definitions::unit_create_options(),
        ),
        || alloc::format!("spawning a unit of def {def_id}"),
    )?;
    // UnitCreated (including the tree's wisp timer) consumes RNG before yaw.
    let yaw = spring
        .synced_random()
        .next_float()
        .expect("synced random float")
        * 2.0
        * PI;
    let _ = spring
        .unit_control()
        .set_unit_rotation(unit_id, Float3::new(0.0, yaw, 0.0));
    Some(unit_id)
}
