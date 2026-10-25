mod worker;
use alloc::vec::Vec;
use spring::Float3;
use springcabal::jobs::{self, Reader, put_f32, put_u32};
pub(crate) use worker::worker;

pub(super) struct Stats {
    pub nearest: Option<usize>,
    pub total: f32,
    pub weights: Vec<f32>,
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
        let eligible = ai.created_frame != frame
            && if ai.def_id == small {
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
                    mid_pos: true,
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
        limits.push(chunk.len() * (8 + 4 * targets.len()));
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
        let mut reader = Reader::new(&bytes);
        for _ in 0..chunk_size.min(positions.len() - offset) {
            let nearest = reader.u32() as usize;
            let total = reader.f32();
            let weights = (0..targets.len()).map(|_| reader.f32()).collect();
            let index = positions[offset].0;
            result[index].stats = Some(Stats {
                nearest: (nearest < targets.len()).then_some(nearest),
                total,
                weights,
            });
            offset += 1;
        }
        reader.finish();
    }
    assert_eq!(offset, positions.len());
    Some(result)
}
