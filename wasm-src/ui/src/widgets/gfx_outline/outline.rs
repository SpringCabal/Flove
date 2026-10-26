mod batch;
mod passes;
mod resources;

use alloc::rc::Rc;
use core::cell::RefCell;

use spring::ResultExt;
use spring::gl;

pub use passes::CreatedFrames;

use resources::{Shaders, Targets};

#[derive(Default)]
pub struct Outline {
    initialized: bool,
    batch: Rc<batch::Batch>,
    vsx: i32,
    vsy: i32,
    res_changed: bool,
    shaders: Shaders,
    targets: Option<Targets>,
}

struct Frame {
    batch: Rc<batch::Batch>,
    targets: Targets,
    shaders: Shaders,
    vsx: i32,
    vsy: i32,
}

impl Outline {
    fn recreate_targets(&mut self, spring: &spring::Spring<'_, spring::Ui>) {
        if let Some(targets) = self.targets.take() {
            targets.delete(spring);
        }
        if self.vsx <= 1 || self.vsy <= 1 {
            return;
        }
        self.targets = Targets::create(spring, self.vsx, self.vsy);
        self.res_changed = true;
    }

    fn ensure_initialized(&mut self, spring: &spring::Spring<'_, spring::Ui>) {
        if self.initialized {
            return;
        }
        if let Ok(geom) = spring.display().get_view_geometry() {
            self.vsx = geom.view_size_x;
            self.vsy = geom.view_size_y;
        }
        spring::log_info!(
            spring,
            "flove-outline",
            "ensure_initialized: vsx={} vsy={}",
            self.vsx,
            self.vsy
        );
        self.shaders = resources::compile_shaders(spring);
        self.recreate_targets(spring);
        self.initialized = true;
    }

    pub fn shutdown(&mut self, spring: &spring::Spring<'_, spring::Ui>) {
        self.batch.shutdown(spring);
        if let Some(targets) = self.targets.take() {
            targets.delete(spring);
        }
        for shader in [self.shaders.depth, self.shaders.blur_h, self.shaders.blur_v] {
            if shader != 0 {
                let _ = spring.gfx().delete_shader(shader);
            }
        }
    }

    pub fn on_view_resize(&mut self, spring: &spring::Spring<'_, spring::Ui>, vsx: i32, vsy: i32) {
        self.vsx = vsx;
        self.vsy = vsy;
        if self.initialized {
            self.recreate_targets(spring);
        }
    }

    fn prepare_frame(&mut self, spring: &spring::Spring<'_, spring::Ui>) -> Option<Frame> {
        self.ensure_initialized(spring);
        if let Ok(geom) = spring.display().get_view_geometry()
            && (geom.view_size_x != self.vsx || geom.view_size_y != self.vsy)
            && geom.view_size_x > 1
            && geom.view_size_y > 1
        {
            self.vsx = geom.view_size_x;
            self.vsy = geom.view_size_y;
            self.recreate_targets(spring);
        }
        let targets = self.targets.clone()?;
        if self.vsx <= 1 || self.vsy <= 1 {
            return None;
        }
        if self.res_changed {
            self.res_changed = false;
            resources::set_view_size_uniforms(spring, &self.shaders, self.vsx, self.vsy);
        }
        Some(Frame {
            batch: Rc::clone(&self.batch),
            targets,
            shaders: self.shaders,
            vsx: self.vsx,
            vsy: self.vsy,
        })
    }
}

pub fn draw_world_pre_unit(
    spring: &spring::Spring<'_, spring::Ui>,
    outline: &RefCell<Outline>,
    created_frames: &RefCell<CreatedFrames>,
) {
    let Some(frame) = outline.borrow_mut().prepare_frame(spring) else {
        return;
    };
    let Frame {
        batch,
        targets,
        shaders,
        vsx,
        vsy,
    } = frame;

    let _ = spring
        .gfx()
        .copy_to_texture(&targets.depth, 0, 0, 0, 0, vsx, vsy, gl::TEXTURE_2D, 0);
    let _ = spring.gfx().bind_texture(&targets.depth, 0, true);

    let mut camera = passes::Camera::default();
    if let Ok(projection) = spring.gfx().get_matrix_data(gl::PROJECTION)
        && let Ok(modelview) = spring.gfx().get_matrix_data(gl::MODELVIEW)
        && projection.len() == 16
        && modelview.len() == 16
    {
        camera.projection.copy_from_slice(&projection);
        camera.modelview.copy_from_slice(&modelview);
    }

    let _ = spring.gfx().use_shader(shaders.depth);
    spring
        .gfx()
        .render_to_texture(&targets.offscreen, |spring| {
            passes::draw_visible_units(
                spring,
                &camera,
                shaders.depth,
                shaders.tree_deformation,
                created_frames,
                &batch,
                (vsx, vsy),
            );
        })
        .log_err_once(spring, "drawing outline silhouettes");

    let _ = spring.gfx().bind_texture(&targets.offscreen, 0, true);
    spring
        .gfx()
        .render_to_texture(&targets.blur, |spring| {
            passes::blur_pass(spring, shaders.blur_v, vsx, vsy);
        })
        .log_err_once(spring, "vertical outline blur");

    let _ = spring.gfx().bind_texture(&targets.blur, 0, true);
    spring
        .gfx()
        .render_to_texture(&targets.offscreen, |spring| {
            passes::blur_pass(spring, shaders.blur_h, vsx, vsy);
        })
        .log_err_once(spring, "horizontal outline blur");

    composite(spring, &targets.offscreen, vsx, vsy);
}

fn composite(spring: &spring::Spring<'_, spring::Ui>, texture: &str, vsx: i32, vsy: i32) {
    let _ = spring.gfx().use_shader(0);
    let _ = spring.gfx().matrix_mode(gl::TEXTURE);
    let _ = spring.gfx().load_identity();
    let _ = spring.gfx().matrix_mode(gl::PROJECTION);
    let _ = spring.gfx().push_matrix();
    let _ = spring.gfx().load_identity();
    let _ = spring.gfx().matrix_mode(gl::MODELVIEW);
    let _ = spring.gfx().push_matrix();
    let _ = spring.gfx().load_identity();

    let _ = spring.gfx().push_attrib(gl::ALL_ATTRIB_BITS);
    let _ = spring.gfx().alpha_test(false, gl::ALWAYS, 0.0);
    let _ = spring.gfx().depth_test(spring::gfx::GfxDepthTestOptions {
        enable: false,
        set_func: false,
        func: 0,
    });
    let _ = spring.gfx().depth_mask(false);
    let _ = spring.gfx().culling(false);
    let _ = spring.gfx().blending(true);
    let _ = spring
        .gfx()
        .blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    let _ = spring.gfx().bind_texture(texture, 0, true);

    passes::full_screen_rect(spring, vsx, vsy);

    let _ = spring.gfx().bind_texture(texture, 0, false);
    let _ = spring.gfx().matrix_mode(gl::PROJECTION);
    let _ = spring.gfx().pop_matrix();
    let _ = spring.gfx().matrix_mode(gl::MODELVIEW);
    let _ = spring.gfx().pop_matrix();
    let _ = spring.gfx().use_shader(0);
    let _ = spring.gfx().pop_attrib();
}
