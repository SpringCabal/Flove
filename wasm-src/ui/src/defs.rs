use std::sync::OnceLock;

pub struct Defs {
    #[cfg(feature = "headless-integration")]
    pub spire: i32,
    pub triggers: [i32; 5],
}

pub fn defs(spring: &spring::Spring<'_, spring::Ui>) -> &'static Defs {
    static DEFS: OnceLock<Defs> = OnceLock::new();
    DEFS.get_or_init(|| {
        let id = |name| {
            spring
                .unit_defs()
                .get_unit_def_id_by_name(name)
                .unwrap_or(-1)
        };
        Defs {
            #[cfg(feature = "headless-integration")]
            spire: id("spire"),
            triggers: [
                "base",
                "expansion1",
                "expansion2",
                "expansion3",
                "mushroomspawn",
            ]
            .map(id),
        }
    })
}
