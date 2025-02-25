// Copyright 2025 Oxide Computer Company

use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use openapiv3::OpenAPI;
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

    let mut builder = Generator::new(
        GenerationSettings::default()
            .with_interface(args.interface.into())
            .with_tag(args.tags.into()),
    );

    match builder.generate_tokens(&api) {
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
                edition = \"2021\"\n\
                license = \"{}\"\n",
                name, version, &args.license_name,
            );
            if let Some(registry_name) = args.registry_name {
                tomlout.extend(format!("publish = [\"{}\"]\n", registry_name).chars());
            }
            tomlout.extend(
                format!(
                    "\n\
                [dependencies]\n\
                {}\n\
                \n",
                    dependencies(builder, args.include_client).join("\n"),
                )
                .chars(),
            );

            save(&toml, tomlout.as_str())?;

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
        }

        Err(e) => {
            println!("gen fail: {:?}", e);
            bail!("generation experienced errors");
        }
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
    percent_encoding: &'static str,
    rand: &'static str,
    regress: &'static str,
    reqwest: &'static str,
    #[cfg(feature = "middleware")]
    reqwest_middleware: &'static str,
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
    percent_encoding: "2.3",
    rand: "0.8",
    regress: "0.10",
    reqwest: "0.12",
    #[cfg(feature = "middleware")]
    reqwest_middleware: "0.4",
    serde: "1.0",
    serde_json: "1.0",
    serde_urlencoded: "0.7",
    uuid: "1.0",
};

pub fn dependencies(builder: Generator, include_client: bool) -> Vec<String> {
    let mut deps = vec![
        format!("bytes = \"{}\"", DEPENDENCIES.bytes),
        format!("futures-core = \"{}\"", DEPENDENCIES.futures),
        format!("reqwest = {{ version = \"{}\", default-features=false, features = [\"json\", \"stream\"] }}", DEPENDENCIES.reqwest),
        #[cfg(feature = "middleware")]
        format!("reqwest-middleware = {{ version = \"{}\", default-features=false, features = [\"json\"] }}", DEPENDENCIES.reqwest_middleware),
        format!("serde = {{ version = \"{}\", features = [\"derive\"] }}", DEPENDENCIES.serde),
        format!("serde_urlencoded = \"{}\"", DEPENDENCIES.serde_urlencoded),
    ];

    let type_space = builder.get_type_space();
    let mut needs_serde_json = false;

    if include_client {
        // code included from progenitor-client needs extra dependencies
        deps.push(format!(
            "percent-encoding = \"{}\"",
            DEPENDENCIES.percent_encoding
        ));
        needs_serde_json = true;
    } else {
        let crate_version =
            if let (false, Some(value)) = (is_non_release(), option_env!("CARGO_PKG_VERSION")) {
                value
            } else {
                "*"
            };
        let client_version_dep = format!("progenitor-client = \"{}\"", crate_version);
        deps.push(client_version_dep);
    }

    if type_space.uses_regress() {
        deps.push(format!("regress = \"{}\"", DEPENDENCIES.regress));
    }
    if type_space.uses_uuid() {
        deps.push(format!(
            "uuid = {{ version = \"{}\", features = [\"serde\", \"v4\"] }}",
            DEPENDENCIES.uuid
        ));
    }
    if type_space.uses_chrono() {
        deps.push(format!(
            "chrono = {{ version = \"{}\", default-features=false, features = [\"serde\"] }}",
            DEPENDENCIES.chrono
        ));
    }
    if builder.uses_futures() {
        deps.push(format!("futures = \"{}\"", DEPENDENCIES.futures));
    }
    if builder.uses_websockets() {
        deps.push(format!("base64 = \"{}\"", DEPENDENCIES.base64));
        deps.push(format!("rand = \"{}\"", DEPENDENCIES.rand));
    }
    if type_space.uses_serde_json() || needs_serde_json {
        deps.push(format!("serde_json = \"{}\"", DEPENDENCIES.serde_json));
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
