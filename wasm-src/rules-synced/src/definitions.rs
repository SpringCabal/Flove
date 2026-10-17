use std::sync::OnceLock;

pub const ENEMY_TEAM_ID: i32 = 0;
pub const PLAYER_TEAM_ID: i32 = 1;
pub const ALL_UNITS: i32 = -1;

pub const FRAMES_PER_SECOND: i32 = 33;

pub const SPIRE_NAME: &str = "spire";
pub const TREE_LEVEL1_NAME: &str = "treelevel1";
pub const TREE_LEVEL2_NAME: &str = "treelevel2";
pub const TREE_LEVEL3_NAME: &str = "treelevel3";

pub const FLOWER_NAMES: [&str; 6] = [
    "flower1", "flower2", "flower3", "flower4", "flower5", "flower6",
];

pub const GRASS_NAMES: [&str; 5] = ["grass1", "grass2", "grass3", "grass4", "grass5"];

pub const TUNNEL_TUBE_NAMES: [&str; 3] = ["tunneltube1", "tunneltube2", "tunneltube3"];

pub const NORMAL_MUSHROOM_NAME: &str = "normalmushroom";
pub const SMALL_MUSHROOM_NAME: &str = "smallmushroom";
pub const BIG_MUSHROOM_NAME: &str = "bigmushroom";
pub const MUSHROOM_CLUSTER_NAME: &str = "mushroomcluster";
pub const POISON_MUSHROOM_NAME: &str = "poisonmushroom";
pub const BOMB_MUSHROOM_NAME: &str = "bombmushroom";
pub const KING_SHROOM_NAME: &str = "kingshroom";
pub const MUSHROOM_SPAWN_NAME: &str = "mushroomspawn";

pub const BASE_NAME: &str = "base";
pub const EXPANSION1_NAME: &str = "expansion1";
pub const EXPANSION2_NAME: &str = "expansion2";
pub const EXPANSION3_NAME: &str = "expansion3";

pub const ZAP_WEAPON_NAME: &str = "zap";
pub const ZAP_ECHO_WEAPON_NAME: &str = "zapecho";
pub const FLOWER_SHOT_WEAPON_NAME: &str = "flowershot";
pub const UPGRADE_SHOT_WEAPON_NAME: &str = "upgradeshot";
pub const POISON_WEAPON_NAME: &str = "poison";

#[inline]
pub fn unit_create_options() -> spring::unit_control::CreateUnitOptions {
    spring::unit_control::CreateUnitOptions {
        build: false,
        flatten_ground: false,
        unit_id: -1,
        builder_id: -1,
    }
}

pub fn defs(spring: &spring::Spring<'_, spring::RulesSynced>) -> &'static DefRegistry {
    static DEFS: OnceLock<DefRegistry> = OnceLock::new();
    DEFS.get_or_init(|| DefRegistry::resolve(spring))
}

#[derive(Debug, Clone)]
pub struct DefRegistry {
    pub spire: i32,
    pub tree_level1: i32,
    pub tree_level2: i32,
    pub tree_level3: i32,
    pub flowers: [i32; 6],
    pub grass: [i32; 5],
    pub tunnel_tubes: [i32; 3],
    pub normal_mushroom: i32,
    pub small_mushroom: i32,
    pub big_mushroom: i32,
    pub mushroom_cluster: i32,
    pub poison_mushroom: i32,
    pub bomb_mushroom: i32,
    pub king_shroom: i32,
    pub mushroom_spawn: i32,
    pub base: i32,
    pub expansion1: i32,
    pub expansion2: i32,
    pub expansion3: i32,
    pub zap_weapon: i32,
    pub zap_echo_weapon: i32,
    pub flower_shot_weapon: i32,
    pub upgrade_shot_weapon: i32,
    pub poison_weapon: i32,
}

impl DefRegistry {
    fn resolve(spring: &spring::Spring<'_, spring::RulesSynced>) -> Self {
        let defs = Self {
            spire: resolve_unit_def(spring, SPIRE_NAME),
            tree_level1: resolve_unit_def(spring, TREE_LEVEL1_NAME),
            tree_level2: resolve_unit_def(spring, TREE_LEVEL2_NAME),
            tree_level3: resolve_unit_def(spring, TREE_LEVEL3_NAME),
            flowers: FLOWER_NAMES.map(|name| resolve_unit_def(spring, name)),
            grass: GRASS_NAMES.map(|name| resolve_unit_def(spring, name)),
            tunnel_tubes: TUNNEL_TUBE_NAMES.map(|name| resolve_unit_def(spring, name)),
            normal_mushroom: resolve_unit_def(spring, NORMAL_MUSHROOM_NAME),
            small_mushroom: resolve_unit_def(spring, SMALL_MUSHROOM_NAME),
            big_mushroom: resolve_unit_def(spring, BIG_MUSHROOM_NAME),
            mushroom_cluster: resolve_unit_def(spring, MUSHROOM_CLUSTER_NAME),
            poison_mushroom: resolve_unit_def(spring, POISON_MUSHROOM_NAME),
            bomb_mushroom: resolve_unit_def(spring, BOMB_MUSHROOM_NAME),
            king_shroom: resolve_unit_def(spring, KING_SHROOM_NAME),
            mushroom_spawn: resolve_unit_def(spring, MUSHROOM_SPAWN_NAME),
            base: resolve_unit_def(spring, BASE_NAME),
            expansion1: resolve_unit_def(spring, EXPANSION1_NAME),
            expansion2: resolve_unit_def(spring, EXPANSION2_NAME),
            expansion3: resolve_unit_def(spring, EXPANSION3_NAME),
            zap_weapon: resolve_weapon_def(spring, ZAP_WEAPON_NAME),
            zap_echo_weapon: resolve_weapon_def(spring, ZAP_ECHO_WEAPON_NAME),
            flower_shot_weapon: resolve_weapon_def(spring, FLOWER_SHOT_WEAPON_NAME),
            upgrade_shot_weapon: resolve_weapon_def(spring, UPGRADE_SHOT_WEAPON_NAME),
            poison_weapon: resolve_weapon_def(spring, POISON_WEAPON_NAME),
        };
        spring::log_info!(
            spring,
            crate::diagnostics::LOG_SECTION,
            "Weapons resolved: zap={}, flower={}, upgrade={}, poison={}",
            defs.zap_weapon,
            defs.flower_shot_weapon,
            defs.upgrade_shot_weapon,
            defs.poison_weapon
        );
        defs
    }

    pub fn is_tree(&self, def_id: i32) -> bool {
        def_id == self.tree_level1 || def_id == self.tree_level2 || def_id == self.tree_level3
    }

    pub fn is_mushroom(&self, def_id: i32) -> bool {
        def_id == self.normal_mushroom
            || def_id == self.small_mushroom
            || def_id == self.big_mushroom
            || def_id == self.mushroom_cluster
            || def_id == self.poison_mushroom
            || def_id == self.bomb_mushroom
            || def_id == self.king_shroom
    }

    pub fn is_flower(&self, def_id: i32) -> bool {
        self.flowers.contains(&def_id)
    }

    pub fn is_shrub(&self, def_id: i32) -> bool {
        self.grass.contains(&def_id) || self.tunnel_tubes.contains(&def_id)
    }

    pub fn is_trigger(&self, def_id: i32) -> bool {
        def_id == self.mushroom_spawn
            || def_id == self.base
            || def_id == self.expansion1
            || def_id == self.expansion2
            || def_id == self.expansion3
    }

    pub fn is_spire(&self, def_id: i32) -> bool {
        def_id == self.spire
    }

    pub fn tree_level(&self, def_id: i32) -> Option<i32> {
        if def_id == self.tree_level1 {
            Some(1)
        } else if def_id == self.tree_level2 {
            Some(2)
        } else if def_id == self.tree_level3 {
            Some(3)
        } else {
            None
        }
    }
}

fn resolve_unit_def(spring: &spring::Spring<'_, spring::RulesSynced>, name: &str) -> i32 {
    spring
        .unit_defs()
        .get_unit_def_id_by_name(name)
        .unwrap_or(-1)
}

fn resolve_weapon_def(spring: &spring::Spring<'_, spring::RulesSynced>, name: &str) -> i32 {
    spring.weapon_defs().get_weapon_def_id(name).unwrap_or(-1)
}
