use progenitor::{GenerationSettings, Generator, InterfaceStyle};

use std::{
    env,
    fs::{self, File},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    let src = "../sample_openapi/keeper.json";
    println!("cargo:rerun-if-changed={}", src);

    let spec = serde_json::from_reader(File::open(src)?)?;

    let mut generator = Generator::new(
        GenerationSettings::new()
            .types_only()
            .with_interface(InterfaceStyle::Builder)
            .with_derive("PartialEq"),
    );

    let tokens = generator.generate_tokens(&spec)?;

    let out_file = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");

    fs::write(out_file, tokens.to_string())?;

    Ok(())
}
