use spring_addons::rules::{CommandEvent, Gadget};

use crate::state::Global;

#[derive(Default)]
pub struct CommandPolicyGadget;

impl CommandPolicyGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for CommandPolicyGadget {
    fn name(&self) -> &'static str {
        "CommandPolicy"
    }

    fn allow_command(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &CommandEvent<'_>,
    ) -> bool {
        event.from_synced && event.command_id != spring::cmd::SELFD
    }
}
