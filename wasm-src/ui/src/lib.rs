extern crate alloc;

mod audio;
mod camera;
mod defs;
mod diagnostics;
mod input;
#[cfg(feature = "headless-integration")]
mod integration;
mod rml;
mod state;
mod story;
mod widgets;

spring_addons::export_ui_widgets! {
    state: state::Global,
    setup: widgets::setup,
    scratch: 4096,
}
