use std::sync::OnceLock;

use flove_shared::grove;
use spring::ResultExt;

use spring::consts::GAME_SPEED;

use spring::Float3;

#[derive(Debug, Clone, Default)]
pub struct SpireState {
    pub unit_id: i32,
    pub last_zap_frame: i32,
    pub last_flower_shot_frame: i32,
    branches: Option<i32>,
}

impl SpireState {
    pub fn new() -> Self {
        Self {
            unit_id: -1,
            last_zap_frame: -1000,
            last_flower_shot_frame: -1000,
            branches: None,
        }
    }

    pub fn update(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
        total_power: i32,
    ) {
        if self.unit_id < 0 {
            return;
        }
        let branches = grove::branch_count(total_power);
        if self.branches != Some(branches)
            && spring
                .rules_params()
                .set_unit_rules_param_f32(
                    self.unit_id,
                    "desiredBranches",
                    branches as f32,
                    spring::rules_params::los::PRIVATE,
                )
                .log_err_once(spring, "telling the spire how many branches to grow")
                .is_some()
        {
            self.branches = Some(branches);
        }
        if frame % 10 == 0 {
            let ceg_name = alloc::format!("mordor{}", grove::flare_level(total_power));
            if let Some(pos) = get_spire_flare_position(spring, self.unit_id) {
                let _ = spring.effects_control().spawn_ceg(
                    &spring::effects_control::DefRef {
                        name: ceg_name.to_owned(),
                        id: -1,
                    },
                    pos,
                    Float3::ZERO,
                    0.0,
                    0.0,
                    0.0,
                );
            }
        }
    }

    pub fn fire_zap(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        target: Float3,
        frame: i32,
        total_power: i32,
    ) -> bool {
        let defs = crate::definitions::defs(spring);
        if self.unit_id < 0 {
            return false;
        }
        let level = grove::flare_level(total_power) as f32;
        let reload = ZAP_RELOAD_SECONDS * FRAMES_PER_SECOND / (1.0 + level * level / 16.0);
        if ((frame - self.last_zap_frame) as f32) < reload {
            return false;
        }
        self.last_zap_frame = frame;

        let Some(spawn_pos) = get_spire_flare_position(spring, self.unit_id) else {
            return false;
        };
        let dir = direction_to(spawn_pos, target);
        spawn_shot(
            spring,
            defs.zap_weapon,
            self.unit_id,
            spawn_pos,
            dir,
            ZAP_PROJECTILE_SPEED,
            ZAP_SPRAY_ANGLE,
        );
        let _ = spring.effects_control().spawn_ceg(
            &spring::effects_control::DefRef {
                name: ("mordor_zap").to_owned(),
                id: -1,
            },
            spawn_pos,
            Float3::ZERO,
            0.0,
            0.0,
            0.0,
        );
        true
    }

    pub fn fire_flower_shot(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        target: Float3,
        frame: i32,
        mana: &mut i32,
    ) -> bool {
        let defs = crate::definitions::defs(spring);
        if self.unit_id < 0 || *mana < FLOWER_SHOT_MANA_COST {
            return false;
        }
        if ((frame - self.last_flower_shot_frame) as f32)
            < FLOWER_SHOT_RELOAD_SECONDS * FRAMES_PER_SECOND
        {
            return false;
        }
        self.last_flower_shot_frame = frame;
        *mana -= FLOWER_SHOT_MANA_COST;

        let Some(spawn_pos) = get_spire_flare_position(spring, self.unit_id) else {
            return false;
        };
        let dir = direction_to(spawn_pos, target);
        spawn_shot(
            spring,
            defs.flower_shot_weapon,
            self.unit_id,
            spawn_pos,
            dir,
            FLOWER_SHOT_PROJECTILE_SPEED,
            FLOWER_SHOT_SPRAY_ANGLE,
        );
        let _ = spring.effects_control().spawn_ceg(
            &spring::effects_control::DefRef {
                name: ("mordor_flowershot").to_owned(),
                id: -1,
            },
            spawn_pos,
            Float3::ZERO,
            0.0,
            0.0,
            0.0,
        );
        true
    }

    pub fn fire_upgrade_shot(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        target_unit_id: i32,
        frame: i32,
        mana: &mut i32,
    ) -> bool {
        let defs = crate::definitions::defs(spring);
        if self.unit_id < 0 || *mana < FLOWER_SHOT_MANA_COST {
            return false;
        }
        if frame - self.last_flower_shot_frame < UPGRADE_SHOT_RELOAD_FRAMES {
            return false;
        }
        let Ok(target_pos) = spring.units_info().get_unit_position(
            target_unit_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        ) else {
            return false;
        };

        self.last_flower_shot_frame = frame;
        *mana -= FLOWER_SHOT_MANA_COST;

        let Some(spawn_pos) = get_spire_flare_position(spring, self.unit_id) else {
            return false;
        };
        let mana_left = *mana;
        spawn_upgrade_projectile(
            spring,
            defs.upgrade_shot_weapon,
            self.unit_id,
            spawn_pos,
            target_pos,
            move |spring, projectile_id| {
                crate::diagnostics::trace!(
                    spring,
                    "[upgrade] shot fired   f={frame} shot={projectile_id:?} aimed_at_tree={target_unit_id} mana_left={mana_left} target=({:.0},{:.0},{:.0})",
                    target_pos.x,
                    target_pos.y,
                    target_pos.z
                );
            },
        );
        let _ = spring.effects_control().spawn_ceg(
            &spring::effects_control::DefRef {
                name: ("mordor_flowershot").to_owned(),
                id: -1,
            },
            spawn_pos,
            Float3::ZERO,
            0.0,
            0.0,
            0.0,
        );
        true
    }
}

pub const FLOWER_SHOT_MANA_COST: i32 = 10;
pub const UPGRADE_SHOT_RELOAD_FRAMES: i32 = 15;

const FRAMES_PER_SECOND: f32 = crate::definitions::FRAMES_PER_SECOND as f32;

const ZAP_RELOAD_SECONDS: f32 = 0.7;
const ZAP_PROJECTILE_SPEED: f32 = 1900.0 / GAME_SPEED as f32;
const ZAP_SPRAY_ANGLE: f32 = 0.0;
const FLOWER_SHOT_RELOAD_SECONDS: f32 = 0.31;
const FLOWER_SHOT_PROJECTILE_SPEED: f32 = 3000.0 / GAME_SPEED as f32;
const FLOWER_SHOT_SPRAY_ANGLE: f32 = 0.0013938562;
const UPGRADE_SHOT_LAUNCH_SPEED: f32 = 75.0;
const UPGRADE_SHOT_MAX_RANGE: f32 = 9000.0;

const GAIA_TEAM: i32 = -1;
const SHOT_TTL: f32 = 9000.0;
const SHOT_MAX_RANGE: f32 = 90000.0;
const GROUND_TARGET_TYPE: i32 = 103;

const FLARE_PIECE_NAME: &str = "Eye";
const FLARE_FALLBACK_HEIGHT: f32 = 740.0;

pub(crate) fn get_spire_flare_position(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    spire_id: i32,
) -> Option<Float3> {
    if let Some(eye) = eye_piece(spring, spire_id)
        && let Ok(pos_dir) = spring.units_pieces().get_unit_piece_pos_dir(spire_id, eye)
    {
        return Some(pos_dir.position);
    }
    let pos = spring
        .units_info()
        .get_unit_position(
            spire_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        )
        .ok()?;
    Some(Float3::new(pos.x, pos.y + FLARE_FALLBACK_HEIGHT, pos.z))
}

fn eye_piece(spring: &spring::Spring<'_, spring::RulesSynced>, spire_id: i32) -> Option<i32> {
    static EYE: OnceLock<i32> = OnceLock::new();
    if let Some(&eye) = EYE.get() {
        return Some(eye);
    }
    let eye = spring
        .units_pieces()
        .get_unit_piece_map(spire_id)
        .ok()?
        .into_iter()
        .find(|piece| piece.name == FLARE_PIECE_NAME)?
        .piece_num;
    Some(*EYE.get_or_init(|| eye))
}

fn direction_to(from: Float3, to: Float3) -> Float3 {
    normalize_direction(Float3::new(to.x - from.x, to.y - from.y, to.z - from.z))
}

fn normalize_direction(v: Float3) -> Float3 {
    let len_sq = v.x * v.x + v.y * v.y + v.z * v.z;
    if len_sq <= 1e-6 {
        return Float3::ZERO;
    }
    let inv_len = 1.0f32 / libm::sqrtf(len_sq);
    Float3::new(v.x * inv_len, v.y * inv_len, v.z * inv_len)
}

fn spawn_shot(
    _spring: &spring::Spring<'_, spring::RulesSynced>,
    weapon_def_id: i32,
    owner_id: i32,
    spawn_pos: Float3,
    dir: Float3,
    speed: f32,
    spray_angle: f32,
) {
    if weapon_def_id < 0 {
        return;
    }
    crate::with_addon_context(|ctx| {
        ctx.commands().run(move |spring| {
            let sprayed = normalize_direction(Float3::new(
                dir.x
                    + (spring
                        .synced_random()
                        .next_float()
                        .expect("synced random float")
                        * 2.0
                        - 1.0)
                        * spray_angle,
                dir.y
                    + (spring
                        .synced_random()
                        .next_float()
                        .expect("synced random float")
                        * 2.0
                        - 1.0)
                        * spray_angle,
                dir.z
                    + (spring
                        .synced_random()
                        .next_float()
                        .expect("synced random float")
                        * 2.0
                        - 1.0)
                        * spray_angle,
            ));
            crate::diagnostics::check(
                spring,
                spring.projectile_control().spawn_projectile(
                    weapon_def_id,
                    &spring::projectile_control::NativeProjectileParams {
                        pos: spawn_pos,
                        end: dir,
                        speed: Float3::new(sprayed.x * speed, sprayed.y * speed, sprayed.z * speed),
                        owner: owner_id,
                        team: GAIA_TEAM,
                        weapon_num: 0,
                        ttl: SHOT_TTL,
                        max_range: SHOT_MAX_RANGE,
                        gravity: 0.0,
                        ..Default::default()
                    },
                ),
                || alloc::format!("firing weapon {weapon_def_id} from the spire"),
            );
        })
    });
}

fn spawn_upgrade_projectile(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    weapon_def_id: i32,
    owner_id: i32,
    spawn_pos: Float3,
    target_pos: Float3,
    fired: impl FnOnce(&spring::Spring<'_, spring::RulesSynced>, Option<i32>) + 'static,
) {
    if weapon_def_id < 0 {
        return fired(spring, None);
    }
    crate::with_addon_context(|ctx| {
        ctx.commands().queue(move |spring| {
            spring.projectile_control().spawn_projectile(
                weapon_def_id,
                &spring::projectile_control::NativeProjectileParams {
                    pos: spawn_pos,
                    end: target_pos,
                    speed: Float3::new(0.0, UPGRADE_SHOT_LAUNCH_SPEED, 0.0),
                    owner: owner_id,
                    team: GAIA_TEAM,
                    weapon_num: 0,
                    ttl: SHOT_TTL,
                    max_range: UPGRADE_SHOT_MAX_RANGE,
                    gravity: 0.0,
                    ..Default::default()
                },
            )
        })
    })
    .then(move |spring, projectile_id| {
        let projectile_id = crate::diagnostics::check(spring, projectile_id, || {
            alloc::format!("firing an upgrade shot at {target_pos:?}")
        });
        if let Some(projectile_id) = projectile_id {
            crate::diagnostics::check(
                spring,
                spring.projectile_control().set_projectile_target(
                    projectile_id,
                    spring::projectile_control::ProjectileTargetRef {
                        target_id: -1,
                        target_type: GROUND_TARGET_TYPE,
                        pos: target_pos,
                        is_ground_target: true,
                    },
                ),
                || alloc::format!("aiming upgrade shot {projectile_id}"),
            );
        }
        fired(spring, projectile_id);
    });
}
