use alloc::vec::Vec;
use spring::Float3;
use springcabal::jobs::{Reader, put_f32, put_u32};
pub(crate) fn worker(bytes: &[u8]) -> Vec<u8> {
    let mut reader = Reader::new(bytes);
    let n = reader.u32() as usize;
    let targets: Vec<_> = (0..n)
        .map(|_| Float3::new(reader.f32(), reader.f32(), reader.f32()))
        .collect();
    let count = reader.u32() as usize;
    let mut output = Vec::with_capacity(count * (8 + 4 * n));
    for _ in 0..count {
        let p = Float3::new(reader.f32(), reader.f32(), reader.f32());
        let mut nearest = u32::MAX;
        let mut minimum = f32::MAX;
        let weights: Vec<_> = targets
            .iter()
            .enumerate()
            .map(|(index, t)| {
                let dx = t.x - p.x;
                let dy = t.y - p.y;
                let dz = t.z - p.z;
                let distance = dx * dx + dy * dy + dz * dz;
                if distance < minimum {
                    minimum = distance;
                    nearest = index as u32;
                }
                if distance > 100.0 {
                    1.0 / distance
                } else {
                    0.0
                }
            })
            .collect();
        put_u32(&mut output, nearest);
        put_f32(&mut output, weights.iter().copied().sum());
        for weight in weights {
            put_f32(&mut output, weight);
        }
    }
    reader.finish();
    output
}
