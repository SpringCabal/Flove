use alloc::string::String;
use core::cell::Cell;

use spring::mouse::{LEFT as BUTTON_LEFT, RIGHT as BUTTON_RIGHT};
use spring_addons::event::{EventResult, KeyEvent};
use spring_addons::ui::Widget;
use springcabal::message::SendOnChange;

use crate::input;
use crate::state::Global;

#[derive(Clone, Copy, Default)]
struct Mouse {
    x: i32,
    y: i32,
    left_down: bool,
    right_down: bool,
}

#[derive(Default)]
pub struct GuiInputWidget {
    mouse: Cell<Mouse>,
    held: SendOnChange<Option<String>>,
}

impl GuiInputWidget {
    pub fn new() -> Self {
        Self::default()
    }

    fn track(&self, x: i32, y: i32, button: Option<(i32, bool)>) {
        let mut mouse = self.mouse.get();
        mouse.x = x;
        mouse.y = y;
        match button {
            Some((BUTTON_LEFT, down)) => mouse.left_down = down,
            Some((BUTTON_RIGHT, down)) => mouse.right_down = down,
            _ => {}
        }
        self.mouse.set(mouse);
    }

    fn hold(&self, spring: &spring::Spring<'_, spring::Ui>, command: Option<String>) {
        self.held.send_if_changed(command, |command| {
            input::send_hold(spring, command.as_deref())
        });
    }
}

impl Widget<Global> for GuiInputWidget {
    fn name(&self) -> &'static str {
        "GuiInput"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>, _dt: f32) {
        let spring = ctx.spring();
        if spring.selection().get_selected_units_count().unwrap_or(0) > 0 {
            let _ = spring.selection().select_unit_array(&[], false);
        }
        if !input::gameplay_active(ctx.global()) {
            self.hold(spring, None);
            return;
        }
        let mouse = match spring.input().get_mouse_state() {
            Ok(state) => Mouse {
                x: state.x as i32,
                y: state.y as i32,
                left_down: state.left,
                right_down: state.right,
            },
            Err(_) => self.mouse.get(),
        };
        self.hold(
            spring,
            input::spire_command(spring, mouse.x, mouse.y, mouse.left_down, mouse.right_down),
        );
    }

    fn mouse_move(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        x: i32,
        y: i32,
        _dx: i32,
        _dy: i32,
        _button: i32,
    ) -> EventResult {
        self.track(x, y, None);
        EventResult::Ignored
    }

    fn mouse_press(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        x: i32,
        y: i32,
        button: i32,
    ) -> EventResult {
        let spring = ctx.spring();
        self.track(x, y, Some((button, true)));
        if !input::gameplay_active(ctx.global()) {
            return EventResult::Handled;
        }
        let command =
            input::spire_command(spring, x, y, button == BUTTON_LEFT, button == BUTTON_RIGHT);
        if command.is_some() {
            self.hold(spring, command);
        }
        if matches!(button, BUTTON_LEFT | BUTTON_RIGHT) {
            EventResult::Handled
        } else {
            EventResult::Ignored
        }
    }

    fn mouse_release(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        x: i32,
        y: i32,
        button: i32,
    ) -> EventResult {
        self.track(x, y, Some((button, false)));
        EventResult::Handled
    }

    fn key_press(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        event: &KeyEvent<'_>,
    ) -> EventResult {
        let spring = ctx.spring();
        input::key_press(spring, ctx.global(), event).into()
    }
}
