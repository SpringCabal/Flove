extern crate alloc;

mod attributes;
mod definitions;
mod diagnostics;
mod flowers;
mod gadgets;
mod input;
#[cfg(feature = "headless-integration")]
mod integration;
mod mushrooms;
mod projectiles;
mod round;
mod spawn;
mod spawn_data;
mod spire;
mod state;
mod status;
mod story;
mod trees;
mod victory;
mod waves;

spring_addons::export_rules_gadgets! {
    state: state::Global,
    setup: gadgets::setup,
    scratch: 16384,
}
