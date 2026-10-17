use crate::flowers::FlowerFieldManager;
use crate::mushrooms::MushroomManager;
use crate::spire::SpireState;
use crate::state::Global;
use crate::story::StoryManager;
use crate::trees::TreeManager;
use crate::victory::VictoryManager;

pub fn populate(spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) {
    let spawn_points = crate::spawn::populate_world(spring);
    let mut mushrooms = global.mushrooms.access_mut();
    mushrooms.spawn_point_units = spawn_points;
    mushrooms.initialize_waves();
    drop(mushrooms);
    global.round.set(global.round.get() + 1);
}

pub fn restart(spring: &spring::Spring<'_, spring::RulesSynced>, difficulty: Option<i32>) {
    for unit_id in spring.units_query().get_all_units().unwrap_or_default() {
        crate::with_addon_context(|ctx| {
            ctx.commands().destroy_unit(
                unit_id,
                spring::unit_control::DestroyUnitOptions {
                    selfd: false,
                    reclaimed: true,
                    attacker_id: -1,
                    recycle_id: false,
                },
            )
        })
        .then(move |spring, removed| {
            crate::diagnostics::check(spring, removed, || {
                alloc::format!("removing unit {unit_id} for the restart")
            });
        });
    }
    crate::with_addon_context(|ctx| {
        ctx.commands()
            .run(move |spring| crate::with_global(|global| reset(spring, global, difficulty)))
    });
}

fn reset(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &Global,
    difficulty: Option<i32>,
) {
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

    let frame = spring.game().current_frame().unwrap_or(0);
    populate(spring, global);
    spring::log_info!(
        spring,
        crate::diagnostics::LOG_SECTION,
        "Flove restarted at frame {frame} (difficulty={difficulty})"
    );
}
