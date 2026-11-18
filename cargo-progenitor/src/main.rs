// Copyright 2025 Oxide Computer Company

use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Parser, ValueEnum};
use openapiv3::OpenAPI;
use progenitor::codespace::{Codespace, Dependency};
use progenitor::{GenerationSettings, Generator, InterfaceStyle, TagStyle};
use progenitor_impl::space_out_items;

fn is_non_release() -> bool {
    cfg!(debug_assertions)
}

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Progenitor(Args),
}

/// Generate a stand-alone crate from an OpenAPI document
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
    /// Target Rust crate registry
    #[clap(long)]
    registry_name: Option<String>,
    /// Target crate license
    #[clap(long, default_value = "SPECIFY A LICENSE BEFORE PUBLISHING")]
    license_name: String,

    /// SDK interface style
    #[clap(value_enum, long, default_value_t = InterfaceArg::Positional)]
    interface: InterfaceArg,
    /// SDK tag style
    #[clap(value_enum, long, default_value_t = TagArg::Merged)]
    tags: TagArg,
    /// Include client code rather than depending on progenitor-client
    #[clap(default_value = match is_non_release() { true => "true", false => "false" }, long, action = clap::ArgAction::Set)]
    include_client: bool,
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

fn reformat_code(input: String) -> String {
    let config = rustfmt_wrapper::config::Config {
        normalize_doc_attributes: Some(true),
        wrap_comments: Some(true),
        ..Default::default()
    };
    space_out_items(rustfmt_wrapper::rustfmt_config(config, input).unwrap()).unwrap()
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
    env_logger::init();

    let CargoCli::Progenitor(args) = CargoCli::parse();
    let api = load_api(&args.input)?;

    let builder = Generator::build(
        GenerationSettings::default()
            .with_interface(args.interface.into())
            .with_tag(args.tags.into()),
        &api,
    )?;

    let sdk = builder.generate_sdk();
    let deps = dependencies(&sdk, args.include_client)?;
    let typespace = builder.typespace();

    println!("-----------------------------------------------------");
    println!(" TYPE SPACE");
    println!("-----------------------------------------------------");
    for (idx, typ) in typespace.iter_types().enumerate() {
        println!("{:>4}  {}", idx, typ.name());
    }
    println!("-----------------------------------------------------");
    println!();

    let name = &args.name;
    let version = &args.version;

    // Create the top-level crate directory:
    let root = PathBuf::from(&args.output);
    std::fs::create_dir_all(&root)?;

    // Write the Cargo.toml file:
    let mut toml = root.clone();
    toml.push("Cargo.toml");

    let mut tomlout = format!(
        "[package]\n\
            name = \"{}\"\n\
            version = \"{}\"\n\
            edition = \"2024\"\n\
            license = \"{}\"\n",
        name, version, &args.license_name,
    );
    if let Some(registry_name) = args.registry_name {
        tomlout.extend(format!("publish = [\"{}\"]\n", registry_name).chars());
    }
    tomlout.push('\n');
    tomlout.push_str(&deps.to_toml_dependencies());
    tomlout.push('\n');

    save(&toml, tomlout.as_str())?;

    let api_code = sdk.into_stream();

    // Create the src/ directory:
    let mut src = root;
    src.push("src");
    std::fs::create_dir_all(&src)?;

    // Create the Rust source file containing the generated client:
    let lib_code = if args.include_client {
        format!("mod progenitor_client;\n\n{}", api_code)
    } else {
        api_code.to_string()
    };
    let lib_code = reformat_code(lib_code);

    let mut librs = src.clone();
    librs.push("lib.rs");
    save(librs, lib_code.as_str())?;

    // Create the Rust source file containing the support code:
    if args.include_client {
        let progenitor_client_code = progenitor_client::code();
        let mut clientrs = src;
        clientrs.push("progenitor_client.rs");
        save(clientrs, progenitor_client_code)?;
    }

    Ok(())
}

/// The generated crate's dependencies, except that with `--include-client` the
/// progenitor-client code is a module of the crate, so the crate takes on that
/// code's dependencies in place of progenitor-client.
fn dependencies(sdk: &Codespace, include_client: bool) -> Result<Codespace> {
    let mut deps = Codespace::default();
    for dep in sdk
        .dependencies()
        .filter(|dep| !include_client || dep.name != "progenitor-client")
    {
        deps.add_dependency(dep.clone())?;
    }

    // Indirect dependencies may or may not be preserved in built's output so we
    // manually encode the versions. We need to take care to update this
    // particularly when generated code depends on particular dependency
    // versions.
    if include_client {
        // code included from progenitor-client needs extra dependencies
        deps.add_dependency(pinned("bytes", "1.9"))?;
        deps.add_dependency(pinned("futures-core", "0.3"))?;
        deps.add_dependency(pinned("percent-encoding", "2.3"))?;
        deps.add_dependency(Dependency {
            default_features: Some(false),
            features: ["json", "query", "stream"]
                .iter()
                .map(|feature| feature.to_string())
                .collect(),
            ..pinned("reqwest", "0.13")
        })?;
        deps.add_dependency(pinned("serde", "1.0"))?;
        deps.add_dependency(pinned("serde_json", "1.0"))?;
        deps.add_dependency(pinned("serde_urlencoded", "0.7"))?;
    }

    Ok(deps)
}

/// A dependency on `name` at the version this generator is tested against.
fn pinned(name: &str, version: &str) -> Dependency {
    Dependency {
        version: version.parse().expect("the pinned versions parse"),
        ..Dependency::new(name)
    }
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
