use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cell::RefCell;
use spring::gfx::GfxUnitDrawOptions;
use spring::gl;
use std::sync::OnceLock;

const OUTLINE_THICKNESS: f32 = 1.0;

#[derive(Clone, Copy)]
enum OutlineKind {
    Tree(u8),
    Spire,
    Hidden,
    Other,
}

const OUTLINE_KINDS: [(&str, OutlineKind); 9] = [
    ("treelevel1", OutlineKind::Tree(1)),
    ("treelevel2", OutlineKind::Tree(2)),
    ("treelevel3", OutlineKind::Tree(3)),
    ("spire", OutlineKind::Spire),
    ("base", OutlineKind::Hidden),
    ("expansion1", OutlineKind::Hidden),
    ("expansion2", OutlineKind::Hidden),
    ("expansion3", OutlineKind::Hidden),
    ("mushroomspawn", OutlineKind::Hidden),
];

fn outline_kind(unit_def_id: spring::DefId) -> OutlineKind {
    static KINDS: OnceLock<Vec<(i32, OutlineKind)>> = OnceLock::new();
    KINDS
        .get_or_init(|| {
            OUTLINE_KINDS
                .iter()
                .filter_map(|&(name, kind)| {
                    spring::get_unit_def_id_by_name(name)
                        .ok()
                        .map(|id| (id, kind))
                })
                .collect()
        })
        .iter()
        .find_map(|&(id, kind)| (id == unit_def_id.0).then_some(kind))
        .unwrap_or(OutlineKind::Other)
}

fn hovered_unit() -> Option<i32> {
    let mouse = spring::input::get_mouse_state(0).ok()?;
    if mouse.offscreen {
        return None;
    }
    if let Ok(context) = spring::rml_ui::get_context("flove-wasm")
        && context.exists
        && spring::rml_ui::context_is_mouse_interacting(context.context_handle).unwrap_or(false)
    {
        return None;
    }
    spring::pick::unit_at(mouse.x, mouse.y)
}

#[derive(Default)]
pub struct CreatedFrames(BTreeMap<i32, f32>);

impl CreatedFrames {
    fn get(&mut self, unit_id: i32) -> f32 {
        if let Some(&frame) = self.0.get(&unit_id) {
            return frame;
        }
        match spring::rules_params::get_unit_rules_param_f32(unit_id, "createdFrame") {
            Ok(frame) => *self.0.entry(unit_id).or_insert(frame),
            Err(_) => 0.0,
        }
    }

    pub fn forget(&mut self, unit_id: i32) {
        self.0.remove(&unit_id);
    }
}

#[derive(Default)]
pub(super) struct Camera {
    pub projection: [f32; 16],
    pub modelview: [f32; 16],
}

pub(super) fn draw_visible_units(
    camera: &Camera,
    deformation_location: i32,
    created_frames: &RefCell<CreatedFrames>,
) {
    let _ = spring::gfx::push_attrib(gl::ALL_ATTRIB_BITS);
    let _ = spring::gfx::alpha_test(false, gl::ALWAYS, 0.0);
    let _ = spring::gfx::blending(false);
    let _ = spring::gfx::depth_mask(true);
    let _ = spring::gfx::depth_test(spring::gfx::GfxDepthTestOptions {
        enable: true,
        set_func: true,
        func: gl::LEQUAL,
    });
    let _ = spring::gfx::clear(gl::COLOR_BUFFER_BIT, &[0.0, 0.0, 0.0, 0.0], 4);
    let _ = spring::gfx::clear(gl::DEPTH_BUFFER_BIT, &[1.0], 1);
    let _ = spring::gfx::matrix_mode(gl::PROJECTION);
    let _ = spring::gfx::load_matrix(&camera.projection);
    let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
    let _ = spring::gfx::load_matrix(&camera.modelview);

    let _ = spring::gfx::color(0.0, 0.0, 0.0, OUTLINE_THICKNESS);

    let unit_ids: Vec<i32> =
        spring::unit_rendering::get_visible_units(-1, 0.0, false).unwrap_or_default();
    let current_frame = spring::game::frame().unwrap_or(0);
    let hovered = hovered_unit();

    for &unit_id in &unit_ids {
        let unit_def_id = match spring::get_unit_def_id(unit_id) {
            Ok(id) if id.0 > 0 => id,
            _ => continue,
        };

        if let Ok(health) = spring::get_unit_health(unit_id)
            && health.build_progress < 1.0
        {
            continue;
        }

        let kind = outline_kind(unit_def_id);
        if matches!(kind, OutlineKind::Hidden) {
            continue;
        }
        let deformation = tree_deformation(unit_id, kind, current_frame, created_frames);
        let _ = spring::gfx::uniform(deformation_location, &deformation, 3);
        let highlight = hovered == Some(unit_id)
            && matches!(kind, OutlineKind::Tree(1 | 2))
            && !spring::units_info::get_unit_is_dead(unit_id).unwrap_or(true);
        if highlight {
            let _ = spring::gfx::color(0.1, 1.0, 0.65, OUTLINE_THICKNESS);
        } else {
            let _ = spring::gfx::color(0.0, 0.0, 0.0, OUTLINE_THICKNESS);
        }

        let _ = spring::gfx::unit(
            unit_id,
            GfxUnitDrawOptions {
                apply_transform: true,
                do_raw_draw: true,
                no_lua_call: true,
                full_model: true,
                ..Default::default()
            },
        );
    }

    let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
    let _ = spring::gfx::pop_attrib(0);
}

fn tree_deformation(
    unit_id: i32,
    kind: OutlineKind,
    current_frame: u32,
    created_frames: &RefCell<CreatedFrames>,
) -> [f32; 3] {
    const GROW_FRAMES: f32 = 33.0 * 0.5;
    let sway = match kind {
        OutlineKind::Tree(_) => 0.002,
        OutlineKind::Spire => 0.0001,
        _ => return [0.0, 1.0, 0.0],
    };
    let phase = (unit_id % 10) as f32 + current_frame as f32 / ((unit_id % 7) + 6) as f32;
    let mut deformation = [sway * libm::sinf(phase), 1.0, 0.0];
    if matches!(kind, OutlineKind::Tree(_)) {
        let created_frame = created_frames.borrow_mut().get(unit_id);
        deformation[1] = ((current_frame as f32 - created_frame) / GROW_FRAMES).min(1.0);
        deformation[2] = spring::units_info::get_unit_height(unit_id).unwrap_or(40.0);
    }
    deformation
}

pub(super) fn blur_pass(shader: u32, vsx: i32, vsy: i32) {
    if shader == 0 || vsx <= 1 || vsy <= 1 {
        return;
    }
    let _ = spring::gfx::clear(gl::COLOR_BUFFER_BIT, &[0.0, 0.0, 0.0, 0.0], 4);
    let _ = spring::gfx::matrix_mode(gl::TEXTURE);
    let _ = spring::gfx::load_identity(0);
    let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
    let _ = spring::gfx::use_shader(shader);
    full_screen_rect(vsx, vsy);
}

pub(super) fn full_screen_rect(vsx: i32, vsy: i32) {
    let x_off = 0.5 / (vsx as f32);
    let y_off = 0.5 / (vsy as f32);
    let _ = spring::gfx::tex_rect(
        -1.0 - x_off,
        1.0 + y_off,
        1.0 + x_off,
        -1.0 - y_off,
        0.0,
        1.0,
        1.0,
        0.0,
    );
}
