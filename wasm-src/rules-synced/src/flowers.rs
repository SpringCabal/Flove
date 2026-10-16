use alloc::vec::Vec;
use spring::commands;
use spring::random::synced as random;
use spring::types::Float3;

use crate::attributes::AttributeManager;
use crate::definitions::ALL_UNITS;
use crate::gadgets::growth::behavior::spawn_flowers;

#[derive(Debug, Clone)]
pub struct SlowCenter {
    pub x: f32,
    pub z: f32,
    pub start_frame: i32,
}

#[derive(Debug, Clone)]
pub struct TempFlower {
    pub unit_id: i32,
    pub expiry_frame: i32,
}

#[derive(Debug, Default)]
pub struct FlowerFieldManager {
    pub centers: Vec<SlowCenter>,
    pub temp_flowers: Vec<TempFlower>,
    pub slowed_units: Vec<i32>,
    attributes: AttributeManager,
}

impl FlowerFieldManager {
    pub fn new() -> Self {
        Self {
            centers: Vec::new(),
            temp_flowers: Vec::new(),
            slowed_units: Vec::new(),
            attributes: AttributeManager::new(),
        }
    }

    pub fn trigger_field_at(&mut self, x: f32, z: f32, frame: i32) {
        let _ = spring::sound::play_sound_file(
            "sounds/wind+leaves.wav",
            20.0,
            Float3::new(x, 0.0, z),
            Float3::ZERO,
            0,
        );
        self.centers.push(SlowCenter {
            x,
            z,
            start_frame: frame,
        });

        commands::run(move || {
            let spawned = spawn_flowers(x, z, 75.0, FLOWER_FIELD_RADIUS);
            let wilting: Vec<TempFlower> = spawned
                .into_iter()
                .map(|unit_id| {
                    let offset = random::int(1, 30) - 15;
                    TempFlower {
                        unit_id,
                        expiry_frame: frame + FLOWER_FIELD_DURATION + offset,
                    }
                })
                .collect();
            crate::with_global(|global| global.flowers.access_mut().temp_flowers.extend(wilting));
        });
    }

    pub fn shot_landed(&mut self, pos: Float3, frame: i32) {
        self.trigger_field_at(pos.x, pos.z, frame);
    }

    pub fn update(&mut self, frame: i32) {
        self.slow_mushrooms(frame);
        self.wilt(frame);
    }

    fn slow_mushrooms(&mut self, frame: i32) {
        let attributes = &mut self.attributes;
        let defs = crate::definitions::defs();
        let mut currently_covered = Vec::new();

        for center in &self.centers {
            if (frame - center.start_frame) % 30 == 0 {
                let y = spring::get_ground_height(center.x, center.z).unwrap_or(0.0);
                let _ = spring::spawn_ceg(
                    "grassfield_aura",
                    Float3::new(center.x, y, center.z),
                    Float3::ZERO,
                    0.0,
                    0.0,
                    0.0,
                );
            }

            let units_in_area = spring::units_query::get_units_in_cylinder(
                center.x,
                center.z,
                FLOWER_FIELD_RADIUS,
                ALL_UNITS,
            )
            .unwrap_or_default();

            for unit_id in units_in_area {
                let Ok(def_id) = spring::units_info::get_unit_def_id(unit_id) else {
                    continue;
                };
                if defs.is_mushroom(def_id) && is_alive(unit_id) {
                    currently_covered.push(unit_id);
                }
            }
        }
        currently_covered.sort_unstable();
        currently_covered.dedup();
        for &unit_id in &currently_covered {
            if !self.slowed_units.contains(&unit_id) {
                self.slowed_units.push(unit_id);
                attributes.set_speed_multiplier(unit_id, SLOW_SPEED_FACTOR);
            }
        }

        self.slowed_units.retain(|&unit_id| {
            if currently_covered.binary_search(&unit_id).is_ok() {
                true
            } else {
                if is_alive(unit_id) {
                    attributes.set_speed_multiplier(unit_id, 1.0);
                }
                attributes.remove_unit(unit_id);
                false
            }
        });

        self.centers
            .retain(|c| frame - c.start_frame < FLOWER_FIELD_DURATION);
    }

    fn wilt(&mut self, frame: i32) {
        self.temp_flowers.retain(|flower| {
            if frame >= flower.expiry_frame {
                let unit_id = flower.unit_id;
                commands::destroy_unit(
                    unit_id,
                    spring::owned::unit_control::DestroyUnitOptions {
                        selfd: false,
                        reclaimed: false,
                        attacker_id: -1,
                        recycle_id: false,
                    },
                )
                .then(move |wilted| {
                    crate::diagnostics::check(wilted, || {
                        alloc::format!("wilting flower {unit_id}")
                    });
                });
                false
            } else {
                true
            }
        });
    }

    pub fn remove_unit(&mut self, unit_id: i32) {
        if let Some(pos) = self.slowed_units.iter().position(|&id| id == unit_id) {
            self.slowed_units.swap_remove(pos);
        }
        if let Some(pos) = self.temp_flowers.iter().position(|f| f.unit_id == unit_id) {
            self.temp_flowers.swap_remove(pos);
        }
        self.attributes.remove_unit(unit_id);
    }
}

fn is_alive(unit_id: i32) -> bool {
    !spring::units_info::get_unit_is_dead(unit_id).unwrap_or(true)
}

pub const FLOWER_FIELD_RADIUS: f32 = 300.0;
pub const FLOWER_FIELD_DURATION: i32 = 360;
pub const SLOW_SPEED_FACTOR: f32 = 0.25;
