#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    Spire,
    Base,
    Expansion1,
    Expansion2,
    Expansion3,
    MushroomSpawn,
}

pub struct SpawnPoint {
    pub marker: Marker,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
}

const fn at(marker: Marker, x: f32, y: f32, z: f32, angle: f32) -> SpawnPoint {
    SpawnPoint {
        marker,
        x,
        y,
        z,
        angle,
    }
}

include!(concat!(env!("OUT_DIR"), "/spawn_points.rs"));
