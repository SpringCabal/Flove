use spring_addons::HideDefaultInterface;
use spring_addons::ui::WidgetHandler;
use springcabal::verification::WorldWidgets;

use super::HIDDEN_INTERFACE;
use crate::state::Global;

pub fn setup(handler: &mut WidgetHandler<Global>) {
    WorldWidgets::new(handler)
        .widget(HideDefaultInterface::new(HIDDEN_INTERFACE))
        .widget(super::gfx_grass::GfxGrassWidget::new())
        .widget(super::gfx_outline::GfxOutlineWidget::new())
        .widget(super::gfx_hide_triggers::GfxHideTriggersWidget::new())
        .announce(crate::diagnostics::SECTION);
}
