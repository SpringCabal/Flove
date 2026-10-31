use spring_addons::rules::Gadget;
use springcabal::message::SendOnChange;

use crate::mushrooms::{SHROOM_EVENT_KING_DEFEATED, SHROOM_EVENT_KING_SPAWNED};
use crate::state::Global;
use crate::status::{self, Status};
use crate::story;

#[derive(Default)]
pub struct StatusGadget {
    published: [SendOnChange<f32>; Status::PARAMS.len()],
}

impl StatusGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for StatusGadget {
    fn name(&self) -> &'static str {
        "Status"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let spring = ctx.spring();
        let global = ctx.global();
        let victory = global.victory.access();
        let shroom_event = global.mushrooms.access().shroom_event;
        let story = match shroom_event {
            SHROOM_EVENT_KING_SPAWNED => story::STAGE_ARMY_APPROACHES,
            SHROOM_EVENT_KING_DEFEATED => story::STAGE_VICTORY,
            _ => global.story.access().stage,
        };
        let status = Status {
            round: global.round.get() as i32,
            power: global.trees.access().total_power(),
            mana: *global.mana.access(),
            story,
            shroom_event,
            game_over: victory.game_over,
            won: victory.won,
        };
        for ((last, name), value) in self
            .published
            .iter()
            .zip(Status::PARAMS)
            .zip(status.values())
        {
            last.send_if_changed(value, |value| status::publish(spring, name, *value));
        }
    }
}
