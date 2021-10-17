// Copyright 2021 Oxide Computer Company

use std::{
    env,
    fs::{self, File},
    path::Path,
};

use progenitor::Generator;

fn main() {
    let file = File::open("../sample_openapi/keeper.json").unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = Generator::new();

    let content = generator.generate_text(&spec).unwrap();

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");

    fs::write(out_file, content).unwrap();
}
