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
}

const fn at(marker: Marker, x: f32, y: f32, z: f32) -> SpawnPoint {
    SpawnPoint { marker, x, y, z }
}

use Marker::*;

#[rustfmt::skip]
pub const INITIAL_UNITS: [SpawnPoint; 40] = [
    at(Expansion3, 4780.0, 1389.1, 6028.0),
    at(Expansion3, 4916.0, 1401.0, 5892.0),
    at(MushroomSpawn, 6844.0, 1537.1, 7028.0),
    at(Base, 5028.0, 1407.9, 5740.0),
    at(Base, 5060.0, 1397.1, 5436.0),
    at(Expansion2, 4084.0, 1369.0, 6268.0),
    at(Expansion1, 5116.0, 1382.5, 5100.0),
    at(Base, 4396.0, 1369.7, 5508.0),
    at(Expansion1, 4788.0, 1376.3, 5148.0),
    at(Expansion3, 5092.0, 1416.0, 5796.0),
    at(Expansion3, 4884.0, 1400.6, 6116.0),
    at(MushroomSpawn, 1820.0, 1591.0, 5108.0),
    at(Expansion1, 5308.0, 1421.4, 5476.0),
    at(Expansion2, 4140.0, 1371.7, 5764.0),
    at(Expansion1, 5372.0, 1410.4, 5220.0),
    at(Expansion3, 5084.0, 1421.7, 6196.0),
    at(Expansion2, 3948.0, 1394.1, 6076.0),
    at(Base, 4788.0, 1388.1, 5900.0),
    at(Expansion1, 5188.0, 1402.5, 5316.0),
    at(Expansion2, 4116.0, 1368.9, 6124.0),
    at(MushroomSpawn, 6796.0, 1569.4, 5412.0),
    at(Expansion1, 4988.0, 1374.3, 4948.0),
    at(Expansion1, 5236.0, 1392.7, 5108.0),
    at(MushroomSpawn, 5316.0, 1439.8, 2508.0),
    at(Base, 4484.0, 1384.3, 5308.0),
    at(Expansion2, 4372.0, 1371.9, 5948.0),
    at(Expansion3, 5196.0, 1430.5, 6012.0),
    at(Expansion3, 5324.0, 1442.8, 5956.0),
    at(MushroomSpawn, 2884.0, 1434.5, 7484.0),
    at(Expansion3, 5180.0, 1424.5, 5820.0),
    at(MushroomSpawn, 4116.0, 1388.9, 3012.0),
    at(Expansion3, 5180.0, 1429.7, 6140.0),
    at(Expansion1, 5044.0, 1382.0, 5196.0),
    at(Base, 4860.0, 1372.3, 5284.0),
    at(Expansion2, 3996.0, 1389.9, 5844.0),
    at(Base, 4476.0, 1369.9, 5756.0),
    at(Expansion3, 5036.0, 1414.1, 6020.0),
    at(Expansion2, 4276.0, 1369.5, 6116.0),
    at(Expansion2, 4380.0, 1373.1, 6236.0),
    at(Spire, 4688.0, 1370.6, 5552.0),
];
