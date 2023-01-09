// Copyright 2022 Oxide Computer Company

use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use openapiv3::OpenAPI;
use progenitor::{GenerationSettings, Generator, InterfaceStyle, TagStyle};

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser)]
struct Args {
    /// OpenAPI definition document (JSON or YAML)
    #[clap(short = 'i', long)]
    input: String,
    /// Output directory for Rust crate
    #[clap(short = 'o', long)]
    output: String,
    /// Target Rust crate name
    #[clap(short = 'n', long)]
    name: String,
    /// Target Rust crate version
    #[clap(short = 'v', long)]
    version: String,

    /// SDK interface style
    #[clap(value_enum, long, default_value_t = InterfaceArg::Positional)]
    interface: InterfaceArg,
    /// SDK tag style
    #[clap(value_enum, long, default_value_t = TagArg::Merged)]
    tags: TagArg,
}

#[derive(Copy, Clone, ValueEnum)]
enum InterfaceArg {
    Positional,
    Builder,
}

impl From<InterfaceArg> for InterfaceStyle {
    fn from(arg: InterfaceArg) -> Self {
        match arg {
            InterfaceArg::Positional => InterfaceStyle::Positional,
            InterfaceArg::Builder => InterfaceStyle::Builder,
        }
    }
}

#[derive(Copy, Clone, ValueEnum)]
enum TagArg {
    Merged,
    Separate,
}

impl From<TagArg> for TagStyle {
    fn from(arg: TagArg) -> Self {
        match arg {
            TagArg::Merged => TagStyle::Merged,
            TagArg::Separate => TagStyle::Separate,
        }
    }
}

fn save<P>(p: P, data: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let p = p.as_ref();
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(p)?;
    f.write_all(data.as_bytes())?;
    f.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let api = load_api(&args.input)?;

    let mut builder = Generator::new(
        GenerationSettings::default()
            .with_interface(args.interface.into())
            .with_tag(args.tags.into()),
    );

    match builder.generate_text(&api) {
        Ok(api_code) => {
            let type_space = builder.get_type_space();

            println!("-----------------------------------------------------");
            println!(" TYPE SPACE");
            println!("-----------------------------------------------------");
            for (idx, type_entry) in type_space.iter_types().enumerate() {
                let n = type_entry.describe();
                println!("{:>4}  {}", idx, n);
            }
            println!("-----------------------------------------------------");
            println!();

            let name = &args.name;
            let version = &args.version;

            /*
             * Create the top-level crate directory:
             */
            let root = PathBuf::from(&args.output);
            std::fs::create_dir_all(&root)?;

            /*
             * Write the Cargo.toml file:
             */
            let mut toml = root.clone();
            toml.push("Cargo.toml");

            let tomlout = format!(
                "[package]\n\
                name = \"{}\"\n\
                version = \"{}\"\n\
                edition = \"2021\"\n\
                \n\
                [dependencies]\n\
                {}\n\
                \n",
                name,
                version,
                dependencies(builder).join("\n"),
            );

            save(&toml, tomlout.as_str())?;

            /*
             * Create the src/ directory:
             */
            let mut src = root;
            src.push("src");
            std::fs::create_dir_all(&src)?;

            /*
             * Create the Rust source file containing the generated client:
             */
            let lib_code = format!("mod progenitor_client;\n\n{}", api_code);
            let mut librs = src.clone();
            librs.push("lib.rs");
            save(librs, lib_code.as_str())?;

            /*
             * Create the Rust source file containing the support code:
             */
            let progenitor_client_code = progenitor_client::code();
            let mut clientrs = src;
            clientrs.push("progenitor_client.rs");
            save(clientrs, progenitor_client_code)?;
        }

        Err(e) => {
            println!("gen fail: {:?}", e);
            bail!("generation experienced errors");
        }
    }

    Ok(())
}

pub fn dependencies(builder: Generator) -> Vec<String> {
    let dependency_versions: HashMap<String, String> = built_info::DEPENDENCIES
        .iter()
        .map(|(name, version)| (name.to_string(), version.to_string()))
        .collect();

    let mut deps = vec![
        format!("bytes = \"{}\"", dependency_versions.get("bytes").unwrap()),
        format!("futures-core = \"{}\"", dependency_versions.get("futures-core").unwrap()),
        format!("percent-encoding = \"{}\"", dependency_versions.get("percent-encoding").unwrap()),
        format!("reqwest = {{ version = \"{}\", default-features=false, features = [\"json\", \"stream\"] }}", dependency_versions.get("reqwest").unwrap()),
        format!("serde = {{ version = \"{}\", features = [\"derive\"] }}", dependency_versions.get("serde").unwrap()),
        format!("serde_urlencoded = \"{}\"", dependency_versions.get("serde_urlencoded").unwrap()),
    ];

    let type_space = builder.get_type_space();

    if type_space.uses_regress() {
        deps.push(format!(
            "regress = \"{}\"",
            dependency_versions.get("regress").unwrap()
        ));
    }
    if type_space.uses_uuid() {
        deps.push(format!(
            "uuid = {{ version = \"{}\", features = [\"serde\", \"v4\"] }}",
            dependency_versions.get("uuid").unwrap()
        ));
    }
    if type_space.uses_chrono() {
        deps.push(format!("chrono = {{ version = \"{}\", default-features=false, features = [\"serde\"] }}", dependency_versions.get("chrono").unwrap()))
    }
    if builder.uses_futures() {
        deps.push(format!(
            "futures = \"{}\"",
            dependency_versions.get("futures").unwrap()
        ));
    }
    if builder.uses_websockets() {
        deps.push(format!(
            "base64 = \"{}\"",
            dependency_versions.get("base64").unwrap()
        ));
        deps.push(format!(
            "rand = \"{}\"",
            dependency_versions.get("rand").unwrap()
        ));
    }
    if type_space.uses_serde_json() {
        deps.push(format!(
            "serde_json = \"{}\"",
            dependency_versions.get("serde_json").unwrap()
        ));
    }
    deps.sort_unstable();
    deps
}

fn load_api<P>(p: P) -> Result<OpenAPI>
where
    P: AsRef<Path> + std::clone::Clone + std::fmt::Debug,
{
    let mut f = File::open(p.clone())?;
    let api = match serde_json::from_reader(f) {
        Ok(json_value) => json_value,
        _ => {
            f = File::open(p)?;
            serde_yaml::from_reader(f)?
        }
    };
    Ok(api)
}
