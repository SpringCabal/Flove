use crate::definitions::DefRegistry;

#[derive(Debug, Clone, Copy)]
pub enum Shroom {
    Normal,
    Small,
    Big,
    Cluster,
    Poison,
    Bomb,
}

impl Shroom {
    pub fn def_id(self, defs: &DefRegistry) -> i32 {
        match self {
            Self::Normal => defs.normal_mushroom,
            Self::Small => defs.small_mushroom,
            Self::Big => defs.big_mushroom,
            Self::Cluster => defs.mushroom_cluster,
            Self::Poison => defs.poison_mushroom,
            Self::Bomb => defs.bomb_mushroom,
        }
    }
}

#[derive(Debug)]
pub struct WaveSpec {
    pub units: &'static [(Shroom, u32)],
    pub time_seconds: i32,
    pub spawn_king: bool,
    pub repeat_seconds: Option<i32>,
}

const fn wave(units: &'static [(Shroom, u32)], time_seconds: i32) -> WaveSpec {
    WaveSpec {
        units,
        time_seconds,
        spawn_king: false,
        repeat_seconds: None,
    }
}

const fn repeating(
    units: &'static [(Shroom, u32)],
    time_seconds: i32,
    repeat_seconds: i32,
) -> WaveSpec {
    WaveSpec {
        units,
        time_seconds,
        spawn_king: false,
        repeat_seconds: Some(repeat_seconds),
    }
}

use Shroom::*;

pub const BASE_WAVES: [WaveSpec; 15] = [
    wave(&[(Normal, 3)], 0),
    wave(&[(Normal, 3), (Small, 3)], 5),
    wave(&[(Big, 1), (Small, 3)], 50),
    wave(&[(Normal, 6)], 100),
    wave(&[(Cluster, 1)], 150),
    wave(&[(Normal, 3), (Big, 1)], 200),
    wave(&[(Poison, 1), (Normal, 10)], 250),
    wave(&[(Normal, 6), (Small, 2)], 300),
    wave(&[(Small, 10), (Big, 2)], 350),
    wave(&[(Normal, 3), (Small, 4)], 400),
    wave(&[(Bomb, 1)], 450),
    wave(&[(Cluster, 3), (Poison, 1)], 500),
    wave(&[(Normal, 5), (Small, 15), (Big, 3)], 550),
    WaveSpec {
        units: &[(Bomb, 3), (Big, 3), (Normal, 3), (Cluster, 2)],
        time_seconds: 600,
        spawn_king: true,
        repeat_seconds: None,
    },
    repeating(&[(Small, 10), (Normal, 7), (Big, 2)], 650, 50),
];

pub const HARD_WAVES: [WaveSpec; 2] = [
    repeating(&[(Normal, 3), (Small, 3)], 75, 100),
    repeating(&[(Cluster, 1)], 90, 100),
];

pub const EXTREME_WAVES: [WaveSpec; 2] = [
    repeating(&[(Poison, 1), (Big, 2)], 230, 60),
    repeating(&[(Big, 3)], 166, 66),
];
