use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-arg=--initial-memory=67108864");
    println!("cargo:rustc-link-arg=--no-growable-memory");
    println!("cargo:rerun-if-changed=build.rs");
    generate_spawn_points();
}

fn generate_spawn_points() {
    let source = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("../../LuaRules/Configs/spawn.lua");
    println!("cargo:rerun-if-changed={}", source.display());
    let text = fs::read_to_string(source).expect("exported Flove scene");
    let mut points = Vec::new();
    for block in text.split("\n    [").skip(1) {
        let name = block
            .split("unitDefName = \"")
            .nth(1)
            .expect("marker name")
            .split('"')
            .next()
            .unwrap();
        let marker = match name {
            "spire" => "Spire",
            "base" => "Base",
            "expansion1" => "Expansion1",
            "expansion2" => "Expansion2",
            "expansion3" => "Expansion3",
            "mushroomspawn" => "MushroomSpawn",
            _ => panic!("unknown scenario marker {name}"),
        };
        let coordinate = |key: &str| {
            block
                .lines()
                .find_map(|line| line.trim().strip_prefix(key))
                .expect("marker coordinate")
                .trim_end_matches(',')
                .trim()
                .parse::<f32>()
                .expect("float32 marker coordinate")
                .to_string()
        };
        points.push(format!(
            "    at(Marker::{marker}, {}f32, {}f32, {}f32, {}f32),\n",
            coordinate("x = "),
            coordinate("y = "),
            coordinate("z = "),
            coordinate("angle = ")
        ));
    }
    assert_eq!(points.len(), 40, "unexpected Flove marker count");
    let output = format!(
        "pub const INITIAL_UNITS: [SpawnPoint; 40] = [\n{}];\n",
        points.concat()
    );
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("spawn_points.rs"),
        output,
    )
    .unwrap();
}
