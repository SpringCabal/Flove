use alloc::string::String;
use core::cell::RefCell;

use flove_shared::command;
use spring_addons::rules::Gadget;
use springcabal::message;

use crate::input;
use crate::state::Global;

#[derive(Default)]
pub struct InputGadget {
    held: RefCell<Option<String>>,
}

impl InputGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for InputGadget {
    fn name(&self) -> &'static str {
        "Input"
    }

    fn handle_lua_msg(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        data: &[u8],
    ) {
        let Ok(text) = core::str::from_utf8(data) else {
            return;
        };
        if text == command::RELEASE {
            *self.held.borrow_mut() = None;
        } else if let Some(command) =
            message::fields(text, command::HOLD).and_then(|mut f| f.rest())
            && input::is_spire_command(command)
        {
            *self.held.borrow_mut() = Some(String::from(command));
            input::handle_command(ctx, command);
        } else {
            input::handle_command(ctx, text);
        }
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let held = self.held.borrow().clone();
        if let Some(command) = held {
            input::handle_command(ctx, &command);
        }
    }
}
