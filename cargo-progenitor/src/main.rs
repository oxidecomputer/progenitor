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

    let mut sdk = builder.generate_sdk();
    populate_dependencies(&mut sdk, args.include_client)?;
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
    tomlout.push_str(&sdk.to_toml_dependencies());
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

// Indirect dependencies may or may not be preserved in built's output so we
// manually encode the versions. We need to take care to update this
// particularly when generated code depends on particular dependency versions.
struct Dependencies {
    base64: &'static str,
    bytes: &'static str,
    chrono: &'static str,
    futures: &'static str,
    json_serde: &'static str,
    percent_encoding: &'static str,
    rand: &'static str,
    regress: &'static str,
    reqwest: &'static str,
    schemars: &'static str,
    serde: &'static str,
    serde_json: &'static str,
    serde_urlencoded: &'static str,
    uuid: &'static str,
}

static DEPENDENCIES: Dependencies = Dependencies {
    base64: "0.22",
    bytes: "1.9",
    chrono: "0.4",
    futures: "0.3",
    json_serde: "0.0.1-alpha.3",
    percent_encoding: "2.3",
    rand: "0.8",
    regress: "0.10",
    reqwest: "0.13",
    schemars: "0.8",
    serde: "1.0",
    serde_json: "1.0",
    serde_urlencoded: "0.7",
    uuid: "1.0",
};

/// Register on the SDK's codespace what every generated client needs and
/// the versions this generator is tested against for the crates the
/// code says it needs, so that the codespace writes the whole
/// `[dependencies]` section.
///
/// A crate this generator has no pin for, such as one named by an
/// `x-rust-type` extension or a crate path override, is declared as the
/// code asks. A pin that disagrees with what the code asks is an error.
pub fn populate_dependencies(sdk: &mut Codespace, include_client: bool) -> Result<()> {
    // What every generated client needs, whatever the document says.
    sdk.add_dependency(pinned("bytes", DEPENDENCIES.bytes))?;
    sdk.add_dependency(pinned("futures-core", DEPENDENCIES.futures))?;
    sdk.add_dependency(Dependency {
        default_features: Some(false),
        features: features(["json", "query", "stream"]),
        ..pinned("reqwest", DEPENDENCIES.reqwest)
    })?;
    sdk.add_dependency(Dependency {
        features: features(["derive"]),
        ..pinned("serde", DEPENDENCIES.serde)
    })?;
    sdk.add_dependency(pinned("serde_urlencoded", DEPENDENCIES.serde_urlencoded))?;

    if include_client {
        // code included from progenitor-client needs extra dependencies
        sdk.add_dependency(pinned("percent-encoding", DEPENDENCIES.percent_encoding))?;
        sdk.add_dependency(pinned("serde_json", DEPENDENCIES.serde_json))?;
    } else {
        let crate_version =
            if let (false, Some(value)) = (is_non_release(), option_env!("CARGO_PKG_VERSION")) {
                value
            } else {
                "*"
            };
        sdk.add_dependency(pinned("progenitor-client", crate_version))?;
    }

    // The pins for whatever else the code asked for.
    let pins = sdk
        .dependencies()
        .filter_map(|dep| pin_for(&dep.name))
        .collect::<Vec<_>>();
    for pin in pins {
        sdk.add_dependency(pin)?;
    }
    Ok(())
}

/// A dependency on `name` at the version this generator is tested against.
fn pinned(name: &str, version: &str) -> Dependency {
    Dependency {
        version: version.parse().expect("the pinned versions parse"),
        ..Dependency::new(name)
    }
}

fn features<const N: usize>(names: [&str; N]) -> Vec<String> {
    names.iter().map(|name| name.to_string()).collect()
}

/// The version and features this generator is tested against for a crate
/// the generated code may name; `None` for a crate it has no pin for,
/// such as one named by an `x-rust-type` extension or a crate path
/// override, which is declared as the code asks.
fn pin_for(name: &str) -> Option<Dependency> {
    let pin = match name {
        "serde_json" => pinned("serde_json", DEPENDENCIES.serde_json),
        "regress" => pinned("regress", DEPENDENCIES.regress),
        "uuid" => Dependency {
            features: features(["serde", "v4"]),
            ..pinned("uuid", DEPENDENCIES.uuid)
        },
        "chrono" => Dependency {
            default_features: Some(false),
            features: features(["serde"]),
            ..pinned("chrono", DEPENDENCIES.chrono)
        },
        "futures" => pinned("futures", DEPENDENCIES.futures),
        "base64" => pinned("base64", DEPENDENCIES.base64),
        "rand" => pinned("rand", DEPENDENCIES.rand),
        "schemars" => pinned("schemars", DEPENDENCIES.schemars),
        "json-serde" => pinned("json-serde", DEPENDENCIES.json_serde),
        _ => return None,
    };
    Some(pin)
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
