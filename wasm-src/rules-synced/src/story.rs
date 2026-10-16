use crate::mushrooms::MushroomManager;
use crate::state::Global;
use crate::trees::TreeManager;
use spring::ResultExt;
use spring::random::synced as random;

pub use flove_shared::{
    SHROOM_EVENT_KING_DEFEATED, STAGE_ARMY_APPROACHES, STAGE_FIRST_WAVE, STAGE_FLOWER_FIELDS,
    STAGE_FREE_PLAY, STAGE_HEAL, STAGE_INTRO, STAGE_REPEL, STAGE_UPGRADE, STAGE_VICTORY,
};

const UPGRADE_STAGE_MANA: i32 = 100;
const SKIPPED_TUTORIAL_MANA: i32 = 30;

pub fn next(global: &Global, frame: i32) {
    if global.mushrooms.access().shroom_event == SHROOM_EVENT_KING_DEFEATED {
        global.victory.access_mut().declare_victory();
        return;
    }
    with_world(global, frame, |story, world| {
        story.handle_next_story_stage(world)
    });
}

pub struct StoryWorld<'a> {
    pub trees: &'a TreeManager,
    pub mushrooms: &'a mut MushroomManager,
    pub spire_id: i32,
    pub mana: &'a mut i32,
    pub frame: i32,
}

pub fn with_world<R>(
    global: &Global,
    frame: i32,
    f: impl FnOnce(&mut StoryManager, &mut StoryWorld<'_>) -> R,
) -> R {
    let spire_id = global.spire.access().unit_id;
    let mut story = global.story.access_mut();
    let trees = global.trees.access();
    let mut mushrooms = global.mushrooms.access_mut();
    let mut mana = global.mana.access_mut();
    let mut world = StoryWorld {
        trees: &trees,
        mushrooms: &mut mushrooms,
        spire_id,
        mana: &mut mana,
        frame,
    };
    f(&mut story, &mut world)
}

#[derive(Debug, Clone)]
pub struct StoryManager {
    pub stage: i32,
    pub trees_take_no_damage: bool,
    pub tutorial_skipped: bool,
    pub story_wave_spawn_frame: Option<i32>,
}

impl StoryManager {
    pub fn new() -> Self {
        Self {
            stage: STAGE_INTRO,
            trees_take_no_damage: false,
            tutorial_skipped: false,
            story_wave_spawn_frame: None,
        }
    }

    pub fn handle_next_story_stage(&mut self, w: &mut StoryWorld<'_>) {
        let next_stage = self.stage + 1;
        if matches!(next_stage, STAGE_REPEL | STAGE_ARMY_APPROACHES) {
            self.set_stage(next_stage, w);
        }
    }

    pub fn skip_tutorial(&mut self, w: &mut StoryWorld<'_>) -> bool {
        if !flove_shared::skip_offered(self.stage) {
            return false;
        }
        self.tutorial_skipped = true;
        self.set_stage(STAGE_ARMY_APPROACHES, w);
        crate::gadgets::growth::behavior::spawn_trees(w.spire_id, 3);
        *w.mana = SKIPPED_TUTORIAL_MANA;
        w.mushrooms.mark_first_wave_spawned();
        true
    }

    pub fn update(&mut self, w: &mut StoryWorld<'_>) {
        if self.stage == STAGE_FREE_PLAY {
            return;
        }

        if self.stage != STAGE_UPGRADE && !self.tutorial_skipped {
            *w.mana = 0;
        }

        match self.stage {
            STAGE_REPEL => {
                if w.mushrooms.story_mushrooms.is_empty() {
                    self.set_stage(STAGE_HEAL, w);
                }
            }
            STAGE_HEAL => {
                if check_trees_healed(w.trees) {
                    self.set_stage(STAGE_FIRST_WAVE, w);
                }
            }
            STAGE_FIRST_WAVE => {
                if let Some(spawn_frame) = self.story_wave_spawn_frame
                    && w.frame - spawn_frame > 30
                    && w.mushrooms.count_living_mushrooms() <= 2
                {
                    self.set_stage(STAGE_UPGRADE, w);
                }
            }
            STAGE_UPGRADE if check_tree_upgraded(w.trees) => {
                self.set_stage(STAGE_FLOWER_FIELDS, w);
            }
            _ => {}
        }
    }

    pub fn set_stage(&mut self, stage: i32, w: &mut StoryWorld<'_>) {
        self.stage = stage;

        match stage {
            STAGE_REPEL => {
                self.trees_take_no_damage = true;
                w.mushrooms.spawn_story_tutorial_mushrooms();
            }
            STAGE_HEAL => {
                self.trees_take_no_damage = false;
                damage_trees(w.trees, 0.5);
            }
            STAGE_FIRST_WAVE => {
                w.mushrooms.force_spawn_wave_1();
                self.story_wave_spawn_frame = Some(w.frame);
            }
            STAGE_UPGRADE => {
                crate::gadgets::growth::behavior::spawn_trees(w.spire_id, 3);
                *w.mana = UPGRADE_STAGE_MANA;
            }
            STAGE_ARMY_APPROACHES => {
                self.stage = STAGE_FREE_PLAY;
                if w.mushrooms.start_spawn_frame.is_none() {
                    w.mushrooms.start_spawn_frame = Some(w.frame + 100);
                }
            }
            _ => {}
        }
    }
}

fn damage_trees(trees: &TreeManager, target_fraction: f32) {
    for tree_id in trees.get_tree_ids() {
        if let Ok(health) = spring::units_info::get_unit_health(tree_id) {
            let deviation = 0.2f32;
            let random_factor = (random::float() * 2.0 - 1.0) * deviation;
            let new_h = (health.health * target_fraction + health.max_health * random_factor)
                .clamp(0.05 * health.max_health, 0.95 * health.max_health);
            spring::set_unit_health(
                tree_id,
                spring::owned::unit_control::UnitHealthValue {
                    health: new_h,
                    capture: 0.0,
                    paralyze: 0.0,
                    build: 1.0,
                    use_amounts: false,
                },
            )
            .log_err_once("wounding a tree for the Heal stage");
        }
    }
}

fn check_trees_healed(trees: &TreeManager) -> bool {
    let tree_ids = trees.get_tree_ids();
    if tree_ids.is_empty() {
        return false;
    }
    for tree_id in tree_ids {
        if let Ok(health) = spring::units_info::get_unit_health(tree_id)
            && health.health < health.max_health * 0.8
        {
            return false;
        }
    }
    true
}

fn check_tree_upgraded(trees: &TreeManager) -> bool {
    trees.trees.iter().any(|t| t.level >= 2)
}
