#![cfg_attr(feature = "animation-trace", allow(dead_code, unused_imports))]

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
pub mod unit_scripts;
mod victory;
mod waves;

spring_addons::export_rules_gadgets! {
    state: state::Global,
    setup: gadgets::setup,
    scratch: 16384,
}

spring::export_core_cus!(unit_scripts::FloveCusModule);
