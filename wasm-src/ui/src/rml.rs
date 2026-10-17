use crate::input;
use crate::state::Global;

const CONTEXT_NAME: &str = "flove-wasm";
const DOCUMENT_PATH: &str = "LuaUI/wasm/flove.rml";

#[derive(Debug, Clone, Copy)]
pub struct HudBindings {
    pub power: u64,
    pub mana: u64,
}

#[derive(Default)]
pub struct Document {
    context: u64,
    document: u64,
    hud: Option<HudBindings>,
    loads: u32,
}

impl Document {
    pub fn is_loaded(&self) -> bool {
        self.context != 0
    }

    pub fn loads(&self) -> u32 {
        self.loads
    }

    pub fn context(&self) -> u64 {
        self.context
    }
}

pub fn is_live(spring: &spring::Spring<'_, spring::Ui>, global: &Global) -> Option<bool> {
    let (context, document) = {
        let loaded = global.document.access();
        (loaded.context, loaded.document)
    };
    let found = spring.rml_ui().get_context(CONTEXT_NAME).ok()?;
    if !found.exists || found.context_handle != context || document == 0 {
        return Some(false);
    }
    match spring.rml_ui().document_get_context(document) {
        Ok(owner) => Some(owner.exists && owner.context_handle == context),
        Err(error) if error.code == spring::ErrorCode::InvalidArgument as i32 => Some(false),
        Err(_) => None,
    }
}

pub fn forget(spring: &spring::Spring<'_, spring::Ui>, global: &Global) {
    let context = {
        let mut loaded = global.document.access_mut();
        let context = loaded.context;
        *loaded = Document {
            loads: loaded.loads,
            ..Document::default()
        };
        context
    };
    if context != 0 {
        let _ = spring.rml_ui().remove_context(context);
    }
}

pub fn load(spring: &spring::Spring<'_, spring::Ui>, global: &Global) -> bool {
    match spring.rml_ui().is_ready() {
        Ok(true) => {}
        Ok(false) => return false,
        Err(error) => {
            spring::log_warning!(
                spring,
                crate::diagnostics::SECTION,
                "RmlUi readiness query failed (error code {})",
                error.code
            );
            return false;
        }
    }

    let context = match spring.rml_ui().create_context(CONTEXT_NAME) {
        Ok(result) if result.success && result.context_handle != 0 => result.context_handle,
        Ok(_) => {
            spring::log_warning!(
                spring,
                crate::diagnostics::SECTION,
                "RmlUi context creation returned no context"
            );
            return false;
        }
        Err(error) => {
            spring::log_warning!(
                spring,
                crate::diagnostics::SECTION,
                "RmlUi context creation failed (error code {})",
                error.code
            );
            return false;
        }
    };
    let Some((hud, document)) = load_document(spring, context) else {
        let _ = spring.rml_ui().remove_context(context);
        return false;
    };
    let mut loaded = global.document.access_mut();
    *loaded = Document {
        context,
        document,
        hud: Some(hud),
        loads: loaded.loads + 1,
    };
    true
}

fn load_document(
    spring: &spring::Spring<'_, spring::Ui>,
    context: u64,
) -> Option<(HudBindings, u64)> {
    let _ = spring.rml_ui().context_unload_all_documents(context);
    let _ = spring.rml_ui().context_remove_data_model(context, "flove");
    let _ = spring.rml_ui().context_enable_mouse_cursor(context, true);

    let Some(hud) = create_hud_model(spring, context) else {
        spring::log_warning!(
            spring,
            crate::diagnostics::SECTION,
            "RmlUi Flove data-model creation failed"
        );
        return None;
    };

    let document = match spring
        .rml_ui()
        .context_load_document(context, DOCUMENT_PATH)
    {
        Ok(result) if result.success && result.document_handle != 0 => result.document_handle,
        Ok(_) => {
            spring::log_warning!(
                spring,
                crate::diagnostics::SECTION,
                "RmlUi Flove document load returned no document"
            );
            return None;
        }
        Err(error) => {
            spring::log_warning!(
                spring,
                crate::diagnostics::SECTION,
                "RmlUi Flove document load failed (error code {})",
                error.code
            );
            return None;
        }
    };
    match spring.rml_ui().document_show(document, Default::default()) {
        Ok(true) => {}
        Ok(false) => {
            spring::log_warning!(
                spring,
                crate::diagnostics::SECTION,
                "RmlUi Flove document show returned false"
            );
            return None;
        }
        Err(error) => {
            spring::log_warning!(
                spring,
                crate::diagnostics::SECTION,
                "RmlUi Flove document show failed (error code {})",
                error.code
            );
            return None;
        }
    }

    Some((hud, document))
}

fn bind_event(
    spring: &spring::Spring<'_, spring::Ui>,
    model: u64,
    name: &str,
    handler: impl Fn(&spring::Spring<'_, spring::Ui>, &Global) + spring::RetainedCapture + 'static,
) -> Option<()> {
    let bound = spring
        .rml_ui()
        .data_model_bind_event(model, name, &[], move |spring, _| {
            crate::with_global(|global| handler(spring, global));
            Ok(())
        })
        .is_ok_and(|registration| registration.success && registration.event_handle != 0);
    if !bound {
        spring::log_warning!(
            spring,
            crate::diagnostics::SECTION,
            "RmlUi event {name} could not be bound"
        );
        return None;
    }
    Some(())
}

fn create_hud_model(spring: &spring::Spring<'_, spring::Ui>, context: u64) -> Option<HudBindings> {
    let model = spring
        .rml_ui()
        .context_create_data_model(context, "flove")
        .ok()?;
    if !model.success || model.data_model_handle == 0 {
        return None;
    }
    let model = model.data_model_handle;
    bind_event(spring, model, "start_normal", |spring, global| {
        input::action_start_difficulty(spring, global, 1)
    })?;
    bind_event(spring, model, "start_hard", |spring, global| {
        input::action_start_difficulty(spring, global, 2)
    })?;
    bind_event(spring, model, "start_extreme", |spring, global| {
        input::action_start_difficulty(spring, global, 3)
    })?;
    bind_event(spring, model, "leave_game", |spring, _| {
        springcabal::menu::quit_to_menu(spring)
    })?;
    bind_event(spring, model, "story_next", |spring, global| {
        input::action_story_next(spring, global);
    })?;
    bind_event(spring, model, "story_skip", |spring, global| {
        input::action_story_skip(spring, global);
    })?;
    bind_event(spring, model, "endgame_restart", input::action_restart)?;
    bind_event(spring, model, "endgame_quit", |spring, _| {
        springcabal::menu::quit_to_menu(spring)
    })?;

    let bind_int = |name: &str, initial: i32| {
        let binding = spring
            .rml_ui()
            .data_model_bind_int(model, name, initial)
            .ok()?;
        (binding.success && binding.variable_handle != 0).then_some(binding.variable_handle)
    };
    Some(HudBindings {
        power: bind_int("power", 0)?,
        mana: bind_int("mana", 0)?,
    })
}

pub fn set_hud_int(
    spring: &spring::Spring<'_, spring::Ui>,
    global: &Global,
    variable: impl FnOnce(&HudBindings) -> u64,
    value: i32,
) {
    let handle = global.document.access().hud.as_ref().map(variable);
    if let Some(handle) = handle.filter(|h| *h != 0) {
        let _ = spring.rml_ui().data_model_set_int(handle, value);
    }
}

fn element(spring: &spring::Spring<'_, spring::Ui>, global: &Global, id: &str) -> Option<u64> {
    let document = global.document.access().document;
    if document == 0 {
        return None;
    }
    let element = spring
        .rml_ui()
        .element_get_element_by_id(document, id)
        .ok()?;
    element.exists.then_some(element.element_handle)
}

pub fn set_text(spring: &spring::Spring<'_, spring::Ui>, global: &Global, id: &str, value: &str) {
    if let Some(handle) = element(spring, global, id) {
        let _ = spring.rml_ui().element_set_inner_rml(handle, value);
    }
}

pub fn set_display(
    spring: &spring::Spring<'_, spring::Ui>,
    global: &Global,
    id: &str,
    visible: bool,
) {
    if let Some(handle) = element(spring, global, id) {
        let _ = spring
            .rml_ui()
            .element_set_class(handle, "hidden", !visible);
    }
}
