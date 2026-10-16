use spring::commands;

use crate::flowers::FlowerFieldManager;
use crate::mushrooms::MushroomManager;
use crate::spire::SpireState;
use crate::state::Global;
use crate::story::StoryManager;
use crate::trees::TreeManager;
use crate::victory::VictoryManager;

pub fn populate(global: &Global) {
    let spawn_points = crate::spawn::populate_world();
    let mut mushrooms = global.mushrooms.access_mut();
    mushrooms.spawn_point_units = spawn_points;
    mushrooms.initialize_waves();
    drop(mushrooms);
    global.round.set(global.round.get() + 1);
}

pub fn restart(difficulty: Option<i32>) {
    for unit_id in spring::units_query::get_all_units().unwrap_or_default() {
        commands::destroy_unit(
            unit_id,
            spring::rules_synced::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: true,
                attacker_id: -1,
                recycle_id: false,
            },
        )
        .then(move |removed| {
            crate::diagnostics::check(removed, || {
                alloc::format!("removing unit {unit_id} for the restart")
            });
        });
    }
    commands::run(move || crate::with_global(|global| reset(global, difficulty)));
}

fn reset(global: &Global, difficulty: Option<i32>) {
    let difficulty = difficulty.unwrap_or_else(|| global.mushrooms.access().difficulty);
    *global.mana.access_mut() = 0;
    *global.spire.access_mut() = SpireState::new();
    *global.trees.access_mut() = TreeManager::new();
    let mut mushrooms = MushroomManager::new();
    mushrooms.difficulty = difficulty;
    *global.mushrooms.access_mut() = mushrooms;
    *global.flowers.access_mut() = FlowerFieldManager::new();
    *global.story.access_mut() = StoryManager::new();
    *global.victory.access_mut() = VictoryManager::default();

    let frame = spring::game::current_frame();
    populate(global);
    spring::log_info!(
        crate::diagnostics::LOG_SECTION,
        "Flove restarted at frame {frame} (difficulty={difficulty})"
    );
}
