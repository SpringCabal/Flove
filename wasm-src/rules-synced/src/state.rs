use core::cell::Cell;

use spring_addons::Resource;

use crate::flowers::FlowerFieldManager;
use crate::mushrooms::MushroomManager;
use crate::spire::SpireState;
use crate::story::StoryManager;
use crate::trees::TreeManager;
use crate::victory::VictoryManager;

pub struct Global {
    pub published_status:
        [springcabal::message::SendOnChange<f32>; crate::status::Status::PARAMS.len()],
    pub round: Cell<u32>,
    pub mana: Resource<i32>,
    pub spire: Resource<SpireState>,
    pub trees: Resource<TreeManager>,
    pub mushrooms: Resource<MushroomManager>,
    pub flowers: Resource<FlowerFieldManager>,
    pub story: Resource<StoryManager>,
    pub victory: Resource<VictoryManager>,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            published_status: Default::default(),
            round: Cell::new(0),
            mana: Resource::new(0),
            spire: Resource::new(SpireState::new()),
            trees: Resource::new(TreeManager::new()),
            mushrooms: Resource::new(MushroomManager::new()),
            flowers: Resource::new(FlowerFieldManager::new()),
            story: Resource::new(StoryManager::new()),
            victory: Resource::default(),
        }
    }
}
