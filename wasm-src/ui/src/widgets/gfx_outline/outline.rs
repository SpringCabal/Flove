mod passes;
mod resources;

use core::cell::RefCell;

use spring::gl;

pub use passes::CreatedFrames;

use resources::{Shaders, Targets};

#[derive(Default)]
pub struct Outline {
    initialized: bool,
    vsx: i32,
    vsy: i32,
    res_changed: bool,
    shaders: Shaders,
    targets: Option<Targets>,
}

struct Frame {
    targets: Targets,
    shaders: Shaders,
    vsx: i32,
    vsy: i32,
}

impl Outline {
    fn recreate_targets(&mut self) {
        if let Some(targets) = self.targets.take() {
            targets.delete();
        }
        if self.vsx <= 1 || self.vsy <= 1 {
            return;
        }
        self.targets = Targets::create(self.vsx, self.vsy);
        self.res_changed = true;
    }

    fn ensure_initialized(&mut self) {
        if self.initialized {
            return;
        }
        if let Ok(geom) = spring::display::get_view_geometry(0) {
            self.vsx = geom.view_size_x;
            self.vsy = geom.view_size_y;
        }
        spring::log_info!(
            "flove-outline",
            "ensure_initialized: vsx={} vsy={}",
            self.vsx,
            self.vsy
        );
        self.shaders = resources::compile_shaders();
        self.recreate_targets();
        self.initialized = true;
    }

    pub fn on_view_resize(&mut self, vsx: i32, vsy: i32) {
        self.vsx = vsx;
        self.vsy = vsy;
        if self.initialized {
            self.recreate_targets();
        }
    }

    fn prepare_frame(&mut self) -> Option<Frame> {
        self.ensure_initialized();
        if let Ok(geom) = spring::display::get_view_geometry(0)
            && (geom.view_size_x != self.vsx || geom.view_size_y != self.vsy)
            && geom.view_size_x > 1
            && geom.view_size_y > 1
        {
            self.vsx = geom.view_size_x;
            self.vsy = geom.view_size_y;
            self.recreate_targets();
        }
        let targets = self.targets.clone()?;
        if self.vsx <= 1 || self.vsy <= 1 {
            return None;
        }
        if self.res_changed {
            self.res_changed = false;
            resources::set_view_size_uniforms(&self.shaders, self.vsx, self.vsy);
        }
        Some(Frame {
            targets,
            shaders: self.shaders,
            vsx: self.vsx,
            vsy: self.vsy,
        })
    }
}

pub fn draw_world_pre_unit(outline: &RefCell<Outline>, created_frames: &RefCell<CreatedFrames>) {
    let Some(frame) = outline.borrow_mut().prepare_frame() else {
        return;
    };
    let Frame {
        targets,
        shaders,
        vsx,
        vsy,
    } = frame;

    let _ = spring::gfx::copy_to_texture(&targets.depth, 0, 0, 0, 0, vsx, vsy, gl::TEXTURE_2D, 0);
    let _ = spring::gfx::bind_texture(&targets.depth, 0, true);

    let mut camera = passes::Camera::default();
    if let Ok(projection) = spring::gfx::get_matrix_data(gl::PROJECTION)
        && let Ok(modelview) = spring::gfx::get_matrix_data(gl::MODELVIEW)
        && projection.len() == 16
        && modelview.len() == 16
    {
        camera.projection.copy_from_slice(&projection);
        camera.modelview.copy_from_slice(&modelview);
    }

    let _ = spring::gfx::use_shader(shaders.depth);
    let _ = spring::gfx::render_to_texture(&targets.offscreen, || {
        passes::draw_visible_units(&camera, shaders.tree_deformation, created_frames)
    });

    let _ = spring::gfx::bind_texture(&targets.offscreen, 0, true);
    let _ = spring::gfx::render_to_texture(&targets.blur, || {
        passes::blur_pass(shaders.blur_v, vsx, vsy)
    });

    let _ = spring::gfx::bind_texture(&targets.blur, 0, true);
    let _ = spring::gfx::render_to_texture(&targets.offscreen, || {
        passes::blur_pass(shaders.blur_h, vsx, vsy)
    });

    composite(&targets.offscreen, vsx, vsy);
}

fn composite(texture: &str, vsx: i32, vsy: i32) {
    let _ = spring::gfx::use_shader(0);
    let _ = spring::gfx::matrix_mode(gl::TEXTURE);
    let _ = spring::gfx::load_identity(0);
    let _ = spring::gfx::matrix_mode(gl::PROJECTION);
    let _ = spring::gfx::push_matrix(0);
    let _ = spring::gfx::load_identity(0);
    let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
    let _ = spring::gfx::push_matrix(0);
    let _ = spring::gfx::load_identity(0);

    let _ = spring::gfx::push_attrib(gl::ALL_ATTRIB_BITS);
    let _ = spring::gfx::alpha_test(false, gl::ALWAYS, 0.0);
    let _ = spring::gfx::depth_test(spring::gfx::GfxDepthTestOptions {
        enable: false,
        set_func: false,
        func: 0,
    });
    let _ = spring::gfx::depth_mask(false);
    let _ = spring::gfx::culling(false);
    let _ = spring::gfx::blending(true);
    let _ = spring::gfx::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    let _ = spring::gfx::bind_texture(texture, 0, true);

    passes::full_screen_rect(vsx, vsy);

    let _ = spring::gfx::bind_texture(texture, 0, false);
    let _ = spring::gfx::matrix_mode(gl::PROJECTION);
    let _ = spring::gfx::pop_matrix(0);
    let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
    let _ = spring::gfx::pop_matrix(0);
    let _ = spring::gfx::use_shader(0);
    let _ = spring::gfx::pop_attrib(0);
}
