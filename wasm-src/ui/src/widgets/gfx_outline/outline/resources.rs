use alloc::rc::Rc;
use spring::gfx::{ShaderCreateParams, TextureCreateParams};
use spring::gl;

const DEPTH_FRAG_SRC: &str = include_str!("shaders/outline_depth.frag");
const DEPTH_VERT_SRC: &str = concat!(
    include_str!("../../../../../../ModelMaterials/Shaders/tree_deformation.glsl"),
    include_str!("shaders/outline_depth.vert"),
);
const BLUR_H_FRAG_SRC: &str = include_str!("shaders/outline_blur_h.frag");
const BLUR_V_FRAG_SRC: &str = include_str!("shaders/outline_blur_v.frag");

const LOG_SECTION: &str = "flove-outline";

#[derive(Clone, Copy, Default)]
pub(super) struct Shaders {
    pub depth: u32,
    pub blur_h: u32,
    pub blur_v: u32,
    pub depth_screen_xy: i32,
    pub tree_deformation: i32,
    pub blur_h_screen_x: i32,
    pub blur_v_screen_y: i32,
}

fn compile(name: &str, params: ShaderCreateParams<'_>) -> Option<u32> {
    match spring::gfx::create_shader(params) {
        Ok(shader) => {
            spring::log_info!(LOG_SECTION, "{name}={}", shader.shader_id);
            Some(shader.shader_id)
        }
        Err(error) => {
            spring::log_warning!(LOG_SECTION, "{name} failed: {error:?}");
            None
        }
    }
}

fn uniform_location(shader: u32, name: &str) -> i32 {
    spring::gfx::get_uniform_location(shader, name).unwrap_or(-1)
}

fn set_int_uniforms(shader: u32, values: &[(&str, i32)]) {
    let _ = spring::gfx::use_shader(shader);
    for &(name, value) in [("tex0", 0)].iter().chain(values) {
        if let Ok(location) = spring::gfx::get_uniform_location(shader, name) {
            let _ = spring::gfx::uniform_int(location, &[value], 1);
        }
    }
}

pub(super) fn compile_shaders() -> Shaders {
    let mut shaders = Shaders {
        depth_screen_xy: -1,
        tree_deformation: -1,
        blur_h_screen_x: -1,
        blur_v_screen_y: -1,
        ..Shaders::default()
    };
    if let Some(id) = compile(
        "depth_shader",
        ShaderCreateParams {
            vertex: DEPTH_VERT_SRC,
            fragment: DEPTH_FRAG_SRC,
            ..Default::default()
        },
    ) {
        shaders.depth = id;
        shaders.depth_screen_xy = uniform_location(id, "screenXY");
        shaders.tree_deformation = uniform_location(id, "treeDeformation");
        set_int_uniforms(id, &[("useEqualityTest", 1)]);
    }
    if let Some(id) = compile(
        "blur_h_shader",
        ShaderCreateParams {
            fragment: BLUR_H_FRAG_SRC,
            ..Default::default()
        },
    ) {
        shaders.blur_h = id;
        shaders.blur_h_screen_x = uniform_location(id, "screenX");
        set_int_uniforms(id, &[]);
    }
    if let Some(id) = compile(
        "blur_v_shader",
        ShaderCreateParams {
            fragment: BLUR_V_FRAG_SRC,
            ..Default::default()
        },
    ) {
        shaders.blur_v = id;
        shaders.blur_v_screen_y = uniform_location(id, "screenY");
        set_int_uniforms(id, &[]);
    }
    shaders
}

pub(super) fn set_view_size_uniforms(shaders: &Shaders, vsx: i32, vsy: i32) {
    if shaders.depth != 0 && shaders.depth_screen_xy >= 0 {
        let _ = spring::gfx::use_shader(shaders.depth);
        let _ = spring::gfx::uniform(shaders.depth_screen_xy, &[vsx as f32, vsy as f32], 2);
    }
    if shaders.blur_h != 0 && shaders.blur_h_screen_x >= 0 {
        let _ = spring::gfx::use_shader(shaders.blur_h);
        let _ = spring::gfx::uniform_int(shaders.blur_h_screen_x, &[vsx], 1);
    }
    if shaders.blur_v != 0 && shaders.blur_v_screen_y >= 0 {
        let _ = spring::gfx::use_shader(shaders.blur_v);
        let _ = spring::gfx::uniform_int(shaders.blur_v_screen_y, &[vsy], 1);
    }
}

#[derive(Clone)]
pub(super) struct Targets {
    pub depth: Rc<str>,
    pub offscreen: Rc<str>,
    pub blur: Rc<str>,
}

impl Targets {
    pub fn create(vsx: i32, vsy: i32) -> Option<Self> {
        let depth = create_texture(
            "depth_tex",
            vsx,
            vsy,
            TextureCreateParams {
                target: gl::TEXTURE_2D,
                format: gl::DEPTH_COMPONENT24,
                border: 0,
                min_filter: gl::NEAREST,
                mag_filter: gl::NEAREST,
                ..Default::default()
            },
        );
        let offscreen = create_texture("offscreen_tex", vsx, vsy, fbo_params(true));
        let blur = create_texture("blur_tex", vsx, vsy, fbo_params(false));
        match (depth, offscreen, blur) {
            (Some(depth), Some(offscreen), Some(blur)) => Some(Self {
                depth,
                offscreen,
                blur,
            }),
            (depth, offscreen, blur) => {
                if let Some(depth) = depth {
                    let _ = spring::gfx::delete_texture(&depth);
                }
                for fbo in [offscreen, blur].into_iter().flatten() {
                    let _ = spring::gfx::delete_texture_fbo(&fbo);
                }
                None
            }
        }
    }

    pub fn delete(&self) {
        let _ = spring::gfx::delete_texture(&self.depth);
        let _ = spring::gfx::delete_texture_fbo(&self.offscreen);
        let _ = spring::gfx::delete_texture_fbo(&self.blur);
    }
}

fn fbo_params(fbo_depth: bool) -> TextureCreateParams {
    TextureCreateParams {
        target: gl::TEXTURE_2D,
        format: 0,
        border: 0,
        min_filter: gl::LINEAR,
        mag_filter: gl::LINEAR,
        wrap_s: gl::CLAMP,
        wrap_t: gl::CLAMP,
        fbo: true,
        fbo_depth,
        ..Default::default()
    }
}

fn create_texture(name: &str, vsx: i32, vsy: i32, params: TextureCreateParams) -> Option<Rc<str>> {
    match spring::gfx::create_texture(vsx, vsy, 0, params) {
        Ok(texture) => {
            spring::log_info!(LOG_SECTION, "created {name}={texture}");
            Some(Rc::from(texture.as_str()))
        }
        Err(error) => {
            spring::log_warning!(LOG_SECTION, "failed {name}: {error:?}");
            None
        }
    }
}
