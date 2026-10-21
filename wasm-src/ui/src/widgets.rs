pub mod gallery;
pub mod game_start;
pub mod gfx_hide_triggers;
pub mod gfx_outline;
pub mod gui_camera;
pub mod gui_hud;
pub mod gui_input;
pub mod gui_rml;
pub mod gui_story;
#[cfg(feature = "headless-integration")]
pub mod integration;
pub mod snd_music;

use spring::default_interface::{
    CLOCK, CONSOLE, MINIMAP, PLAYER_INFO, RESOURCE_BAR, TOOLTIP, UNIT_ICONS,
};
use spring_addons::HideDefaultInterface;
use spring_addons::register_widgets;
use spring_addons::ui::WidgetHandler;

use crate::state::Global;

const HIDDEN_INTERFACE: u32 =
    CONSOLE | RESOURCE_BAR | TOOLTIP | CLOCK | PLAYER_INFO | MINIMAP | UNIT_ICONS;

pub fn setup(handler: &mut WidgetHandler<Global>, spring: &spring::Spring<'_, spring::Ui>) {
    register_widgets!(handler, [springcabal::controls::GameControls::new()]);
    if springcabal::verification::world_only(spring) {
        gallery::setup(spring, handler);
        return;
    }
    #[cfg(feature = "headless-integration")]
    register_widgets!(handler, [integration::IntegrationWidget::new()]);

    register_widgets!(
        handler,
        [
            HideDefaultInterface::new(HIDDEN_INTERFACE & !CONSOLE),
            game_start::StartForceWidget::new(),
            gfx_outline::GfxOutlineWidget::new(),
            gui_rml::GuiRmlWidget::new(),
            gui_hud::GuiHudWidget::new(),
            gui_story::GuiStoryWidget::new(),
            gui_input::GuiInputWidget::new(),
            gui_camera::GuiCameraWidget::new(),
            snd_music::SndMusicWidget::new(),
            gfx_hide_triggers::GfxHideTriggersWidget::new(),
        ]
    );
}
