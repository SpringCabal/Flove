use core::cell::{Cell, RefCell};

use spring::ResultExt;
use spring::gl;

const VERTEX: &str =
    include_str!("../../../../../../ModelMaterials/Shaders/outline_instanced.vert");
const FRAGMENT: &str =
    include_str!("../../../../../../ModelMaterials/Shaders/outline_instanced.frag");

struct Resources {
    shader: u32,
    viewport: i32,
    vertex: u32,
    index: u32,
    instances: u32,
    vao: u32,
    capacity: usize,
}

impl Resources {
    fn create(spring: &spring::Spring<'_, spring::Ui>) -> spring::Result<Self> {
        let shader = spring
            .gfx()
            .create_shader("", VERTEX, "", "", "", FRAGMENT, "", Default::default())?
            .shader_id;
        let mut resources = Self {
            shader,
            viewport: -1,
            vertex: 0,
            index: 0,
            instances: 0,
            vao: 0,
            capacity: 0,
        };
        let configure = (|| {
            spring.gfx().use_shader(shader)?;
            resources.viewport = spring.gfx().get_uniform_location(shader, "screenXY")?;
            let texture = spring.gfx().get_uniform_location(shader, "tex0")?;
            spring.gfx().uniform_int(texture, &[0], 1)?;
            spring.gfx().use_shader(0)?;
            resources.vertex = spring.gfx().get_vbo(gl::ARRAY_BUFFER, false)?.vbo_id;
            resources.index = spring
                .gfx()
                .get_vbo(gl::ELEMENT_ARRAY_BUFFER, false)?
                .vbo_id;
            spring.gfx().models_vbo(resources.vertex)?;
            spring.gfx().models_vbo(resources.index)?;
            Ok(())
        })();
        if let Err(error) = configure {
            resources.delete(spring);
            return Err(error);
        }
        Ok(resources)
    }

    fn grow(
        &mut self,
        spring: &spring::Spring<'_, spring::Ui>,
        count: usize,
    ) -> spring::Result<()> {
        let capacity = count.max(256).next_power_of_two();
        let instances = spring.gfx().get_vbo(gl::ARRAY_BUFFER, true)?.vbo_id;
        let vao = match spring.gfx().get_vao() {
            Ok(vao) => vao.vao_id,
            Err(error) => {
                let _ = spring.gfx().delete_vbo(instances);
                return Err(error);
            }
        };
        let attributes = [
            spring::gfx::GfxVBOAttributeOptions {
                id: 6,
                size: 4,
                type_: gl::UNSIGNED_INT,
                normalized: false,
            },
            spring::gfx::GfxVBOAttributeOptions {
                id: 7,
                size: 4,
                type_: gl::FLOAT,
                normalized: false,
            },
        ];
        let configure = spring
            .gfx()
            .define_vbo(instances, capacity as i32, false, 0, false, 0, &attributes)
            .and_then(|()| spring.gfx().attach_vertex_buffer_vao(vao, self.vertex))
            .and_then(|()| spring.gfx().attach_index_buffer_vao(vao, self.index))
            .and_then(|()| spring.gfx().attach_instance_buffer_vao(vao, instances));
        if let Err(error) = configure {
            let _ = spring.gfx().delete_vao(vao);
            let _ = spring.gfx().delete_vbo(instances);
            return Err(error);
        }
        if self.vao != 0 {
            let _ = spring.gfx().delete_vao(self.vao);
        }
        if self.instances != 0 {
            let _ = spring.gfx().delete_vbo(self.instances);
        }
        self.vao = vao;
        self.instances = instances;
        self.capacity = capacity;
        Ok(())
    }

    fn draw(
        &mut self,
        spring: &spring::Spring<'_, spring::Ui>,
        ids: &[u32],
        effects: &[f32],
        size: (i32, i32),
    ) -> spring::Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        assert_eq!(effects.len(), 4 * ids.len());
        if ids.len() > self.capacity {
            self.grow(spring, ids.len())?;
        }
        spring
            .gfx()
            .instance_data_from_units_vbo(self.instances, ids, 6, -1, 0)?;
        spring
            .gfx()
            .upload_vbo(self.instances, effects, 7, 0, 1, 0)?;
        spring.gfx().clear_submission_vao(self.vao)?;
        spring.gfx().add_units_to_submission_vao(self.vao, ids)?;
        spring.gfx().use_shader(self.shader)?;
        spring
            .gfx()
            .uniform(self.viewport, &[size.0 as f32, size.1 as f32], 2)?;
        spring.gfx().submit_vao(self.vao)
    }

    fn delete(self, spring: &spring::Spring<'_, spring::Ui>) {
        if self.vao != 0 {
            let _ = spring.gfx().delete_vao(self.vao);
        }
        for vbo in [self.instances, self.vertex, self.index] {
            if vbo != 0 {
                let _ = spring.gfx().delete_vbo(vbo);
            }
        }
        if self.shader != 0 {
            let _ = spring.gfx().delete_shader(self.shader);
        }
    }
}

#[derive(Default)]
pub(super) struct Batch {
    resources: RefCell<Option<Resources>>,
    failed: Cell<bool>,
}

impl Batch {
    pub fn available(&self, spring: &spring::Spring<'_, spring::Ui>) -> bool {
        if self.resources.borrow().is_none() && !self.failed.get() {
            let resources =
                Resources::create(spring).log_err_once(spring, "creating batched outline models");
            self.failed.set(resources.is_none());
            *self.resources.borrow_mut() = resources;
        }
        self.resources.borrow().is_some()
    }

    pub fn draw(
        &self,
        spring: &spring::Spring<'_, spring::Ui>,
        ids: &[u32],
        effects: &[f32],
        size: (i32, i32),
    ) -> bool {
        let mut resources = self.resources.borrow_mut();
        let ok = resources.as_mut().is_some_and(|resources| {
            resources
                .draw(spring, ids, effects, size)
                .log_err_once(spring, "submitting batched outline models")
                .is_some()
        });
        if !ok {
            self.failed.set(true);
            if let Some(resources) = resources.take() {
                resources.delete(spring);
            }
        }
        ok
    }

    pub fn shutdown(&self, spring: &spring::Spring<'_, spring::Ui>) {
        if let Some(resources) = self.resources.borrow_mut().take() {
            resources.delete(spring);
        }
    }
}
