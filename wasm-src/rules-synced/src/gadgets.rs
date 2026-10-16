pub mod command_policy;
pub mod flowers;
pub mod growth;
pub mod input;
#[cfg(feature = "headless-integration")]
pub mod integration;
pub mod love;
pub mod mushrooms;
pub mod poison;
pub mod round;
pub mod spire;
pub mod status;
pub mod story;
pub mod trees;
pub mod upgrade_shots;

pub use command_policy::CommandPolicyGadget;
pub use flowers::FlowersGadget;
pub use growth::GrowthGadget;
pub use input::InputGadget;
#[cfg(feature = "headless-integration")]
pub use integration::IntegrationGadget;
pub use love::LoveGadget;
pub use mushrooms::MushroomsGadget;
pub use poison::PoisonGadget;
pub use round::RoundGadget;
pub use spire::SpireGadget;
pub use status::StatusGadget;
pub use story::StoryGadget;
pub use trees::TreesGadget;
pub use upgrade_shots::UpgradeShotsGadget;

use spring_addons::register_gadgets;
use spring_addons::rules::GadgetHandler;

use crate::state::Global;

pub fn setup(handler: &mut GadgetHandler<Global>) {
    #[cfg(feature = "e2e")]
    spring::log_info!(
        "flove",
        "FLOVE E2E BUILD: add_mana, upgrade, field_of_flowers and flower accepted"
    );
    {
        register_gadgets!(
            handler,
            [
                RoundGadget::new(),
                TreesGadget::new(),
                GrowthGadget::new(),
                MushroomsGadget::new(),
                StoryGadget::new(),
                FlowersGadget::new(),
                PoisonGadget::new(),
                SpireGadget::new(),
                StatusGadget::new(),
            ]
        );

        #[cfg(feature = "headless-integration")]
        register_gadgets!(handler, [IntegrationGadget::new()]);

        register_gadgets!(
            handler,
            [
                CommandPolicyGadget::new(),
                InputGadget::new(),
                LoveGadget::new(),
                UpgradeShotsGadget::new(),
            ]
        );
    }
}
