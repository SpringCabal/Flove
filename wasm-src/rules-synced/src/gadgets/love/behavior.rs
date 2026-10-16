use spring::commands;
use spring::random::synced as random;
use spring::types::Float3;

const FLOWER_KILLING_DAMAGE: f32 = 9999.0;

pub fn handle_pre_damage(
    unit_id: i32,
    unit_def_id: i32,
    damage: f32,
    weapon_def_id: i32,
    trees_take_no_damage: bool,
) -> spring::DamageResult {
    let defs = crate::definitions::defs();
    let is_zap = weapon_def_id == defs.zap_weapon;
    let is_zap_echo = weapon_def_id == defs.zap_echo_weapon;

    if is_zap || is_zap_echo {
        let Ok(pos) = spring::units_info::get_unit_position(
            unit_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: true,
                aim_pos: false,
            },
        ) else {
            return spring::DamageResult::unchanged(damage);
        };

        if defs.is_tree(unit_def_id) {
            let _ = spring::spawn_ceg("love_heals", pos, Float3::ZERO, 0.0, 0.0, 0.0);
            if let Ok(health) = spring::units_info::get_unit_health(unit_id)
                && health.health < health.max_health
            {
                play_sigh_sound(pos);
            }
            return spring::DamageResult {
                new_damage: -damage,
                impulse_mult: 1.0,
            };
        }

        if defs.is_flower(unit_def_id) {
            if is_zap {
                let _ = spring::spawn_ceg("love_heals", pos, Float3::ZERO, 0.0, 0.0, 0.0);
                if defs.zap_echo_weapon > 0 {
                    spawn_zap_echo(defs.zap_echo_weapon, unit_id, pos);
                }
            }
            return spring::DamageResult {
                new_damage: -damage,
                impulse_mult: 1.0,
            };
        }

        if defs.is_shrub(unit_def_id) {
            let _ = spring::spawn_ceg("love_heals", pos, Float3::ZERO, 0.0, 0.0, 0.0);
            return spring::DamageResult {
                new_damage: 0.0,
                impulse_mult: 0.0,
            };
        }

        if defs.is_mushroom(unit_def_id) {
            play_squeak_sound(pos);
            let _ = spring::spawn_ceg("love_hurts", pos, Float3::ZERO, 0.0, 0.0, 0.0);
            return spring::DamageResult {
                new_damage: damage,
                impulse_mult: 1.0,
            };
        }

        return spring::DamageResult {
            new_damage: 0.0,
            impulse_mult: 0.0,
        };
    }

    if weapon_def_id == defs.poison_weapon {
        if !defs.is_flower(unit_def_id) {
            return spring::DamageResult {
                new_damage: 0.0,
                impulse_mult: 0.0,
            };
        }
        if let Ok(pos) = spring::units_info::get_unit_position(
            unit_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: true,
                aim_pos: false,
            },
        ) {
            let _ = spring::spawn_ceg("barftrail", pos, Float3::ZERO, 0.0, 0.0, 0.0);
        }
        return spring::DamageResult {
            new_damage: FLOWER_KILLING_DAMAGE,
            impulse_mult: 0.0,
        };
    }

    if defs.is_tree(unit_def_id) && trees_take_no_damage {
        return spring::DamageResult {
            new_damage: 0.0,
            impulse_mult: 0.0,
        };
    }

    spring::DamageResult::unchanged(damage)
}

pub fn zap_landed(projectile_id: i32) {
    if let Ok(pos) = spring::projectiles::get_projectile_position(projectile_id) {
        let _ = spring::sound::play_sound_file(ZAP_LANDED_SOUND, 5.0, pos, Float3::ZERO, 0);
    }
}

const ZAP_LANDED_SOUND: &str = "sounds/fairy dust.wav";

fn play_sigh_sound(pos: Float3) {
    let sounds = ["sounds/sigh2.wav", "sounds/sigh3.wav", "sounds/sigh4.wav"];
    let idx = random::int(0, 2) as usize;
    let _ = spring::sound::play_sound_file(sounds[idx], 3.0, pos, Float3::ZERO, 0);
}

fn play_squeak_sound(pos: Float3) {
    let sounds = [
        "sounds/mushroomsqueak.wav",
        "sounds/mushroomsqueak2.wav",
        "sounds/mushroomsqueak3.wav",
        "sounds/mushroomsqueak4.wav",
        "sounds/mushroomsqueak5.wav",
    ];
    let idx = random::int(0, 4) as usize;
    let _ = spring::sound::play_sound_file(sounds[idx], 3.0, pos, Float3::ZERO, 0);
}

fn spawn_zap_echo(weapon_def_id: i32, owner_id: i32, pos: Float3) {
    commands::run(move || {
        crate::diagnostics::check(
            spring::spawn_projectile(
                weapon_def_id,
                spring::ProjectileParams {
                    position: pos,
                    end: pos,
                    speed: Float3::ZERO,
                    owner: owner_id,
                    team: -1,
                    weapon_number: -1,
                    ttl: 1.0,
                    max_range: 2.0,
                    up_time: -1.0,
                    end_alpha: 1.0,
                    ..Default::default()
                },
            ),
            || alloc::format!("zap echo on flower {owner_id}"),
        );
    });
}
