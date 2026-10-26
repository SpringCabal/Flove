use alloc::vec::Vec;
use spring::Float3;
use springcabal::jobs::{Reader, put_f32};
pub(crate) fn worker(bytes: &[u8]) -> Vec<u8> {
    let mut reader = Reader::new(bytes);
    let n = reader.u32() as usize;
    let targets: Vec<_> = (0..n)
        .map(|_| Float3::new(reader.f32(), reader.f32(), reader.f32()))
        .collect();
    let count = reader.u32() as usize;
    let mut output = Vec::with_capacity(count * (4 + 4 * n));
    let mut weights = Vec::with_capacity(n);
    for _ in 0..count {
        let p = Float3::new(reader.f32(), reader.f32(), reader.f32());
        weights.clear();
        weights.extend(targets.iter().map(|t| {
            let dx = t.x - p.x;
            let dy = t.y - p.y;
            let dz = t.z - p.z;
            let distance = dx * dx + dy * dy + dz * dz;
            if distance > 100.0 {
                1.0 / distance
            } else {
                0.0
            }
        }));
        put_f32(&mut output, weights.iter().copied().sum());
        for &weight in &weights {
            put_f32(&mut output, weight);
        }
    }
    reader.finish();
    output
}
