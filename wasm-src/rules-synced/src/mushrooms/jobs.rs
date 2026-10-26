mod worker;
use alloc::{rc::Rc, vec::Vec};
use spring::Float3;
use springcabal::jobs::{self, Reader, put_f32, put_u32};
pub(crate) use worker::worker;

pub(super) struct Stats {
    pub total: f32,
    packet: Rc<Vec<u8>>,
    weights_start: usize,
    weights_len: usize,
}
impl Stats {
    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.packet[self.weights_start..self.weights_start + self.weights_len * 4]
            .chunks_exact(4)
            .map(|word| f32::from_le_bytes(word.try_into().expect("four-byte weight")))
    }
}
pub(super) struct Prepared {
    pub eligible: bool,
    pub stats: Option<Stats>,
}

pub(super) fn prepare(
    options: &mut jobs::Options,
    spring: &spring::Spring<'_, spring::RulesSynced>,
    units: &[super::MushroomAiUnit],
    frame: i32,
    small: i32,
    targets: &[(i32, Float3)],
) -> Option<Vec<Prepared>> {
    use spring::ResultExt;
    let mut result = Vec::with_capacity(units.len());
    let mut positions = Vec::new();
    for (index, ai) in units.iter().enumerate() {
        let eligible = if ai.def_id == small {
            frame - ai.last_order_frame >= 30
        } else {
            !spring
                .units_commands()
                .get_unit_command_count(ai.unit_id)
                .log_err_once(spring, "querying mushroom orders")
                .is_some_and(|n| n != 0)
        };
        result.push(Prepared {
            eligible,
            stats: None,
        });
        if eligible
            && let Ok(position) = spring.units_info().get_unit_position(
                ai.unit_id,
                spring::units_info::GetUnitPositionOptions {
                    mid_pos: false,
                    aim_pos: false,
                },
            )
        {
            positions.push((index, position));
        }
    }
    if positions.is_empty() {
        return Some(result);
    }
    let chunk_size = positions.len().div_ceil(jobs::CHUNKS);
    let mut inputs = Vec::new();
    let mut limits = Vec::new();
    for chunk in positions.chunks(chunk_size) {
        let mut bytes = Vec::new();
        put_u32(&mut bytes, targets.len() as u32);
        for (_, p) in targets {
            for v in [p.x, p.y, p.z] {
                put_f32(&mut bytes, v);
            }
        }
        put_u32(&mut bytes, chunk.len() as u32);
        for (_, p) in chunk {
            for v in [p.x, p.y, p.z] {
                put_f32(&mut bytes, v);
            }
        }
        limits.push(chunk.len() * (4 + 4 * targets.len()));
        inputs.push(bytes);
    }
    let output = options.run(
        spring,
        crate::MUSHROOM_TARGETS,
        &inputs,
        &limits,
        "mushroom-targets",
        worker,
    )?;
    let mut offset = 0;
    for bytes in output {
        let count = chunk_size.min(positions.len() - offset);
        let stride = 4 + 4 * targets.len();
        assert_eq!(bytes.len(), count * stride, "mushroom job output length");
        let packet = Rc::new(bytes);
        for row in 0..count {
            let start = row * stride;
            let mut reader = Reader::new(&packet[start..start + 4]);
            let total = reader.f32();
            reader.finish();
            let index = positions[offset].0;
            result[index].stats = Some(Stats {
                total,
                packet: Rc::clone(&packet),
                weights_start: start + 4,
                weights_len: targets.len(),
            });
            offset += 1;
        }
    }
    assert_eq!(offset, positions.len());
    Some(result)
}
