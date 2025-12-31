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
use progenitor_impl::{space_out_items, HttpBackend};

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
    /// HTTP backend to use
    #[clap(value_enum, long, default_value_t = BackendArg::Reqwest)]
    backend: BackendArg,
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

#[derive(Copy, Clone, ValueEnum)]
enum BackendArg {
    Reqwest,
    Gloo,
}

impl From<BackendArg> for HttpBackend {
    fn from(arg: BackendArg) -> Self {
        match arg {
            BackendArg::Reqwest => HttpBackend::Reqwest,
            BackendArg::Gloo => HttpBackend::Gloo,
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

    // Validate backend and include_client compatibility
    if args.include_client && matches!(args.backend, BackendArg::Gloo) {
        bail!(
            "The --include-client flag is not supported with --backend gloo. \
            The gloo backend requires specific WASM dependencies that should be managed via Cargo.toml. \
            Use --include-client false and add 'progenitor-client = {{ version = \"...\", features = [\"gloo-client\"] }}' to your dependencies."
        );
    }

    let mut builder = Generator::new(
        GenerationSettings::default()
            .with_interface(args.interface.into())
            .with_tag(args.tags.into())
            .with_backend(args.backend.into()),
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
                    dependencies(builder, args.backend.into(), args.include_client).join("\n"),
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
    getrandom: &'static str,
    gloo_net: &'static str,
    js_sys: &'static str,
    percent_encoding: &'static str,
    rand: &'static str,
    regress: &'static str,
    reqwest: &'static str,
    serde: &'static str,
    serde_json: &'static str,
    serde_urlencoded: &'static str,
    uuid: &'static str,
    wasm_bindgen: &'static str,
    wasm_streams: &'static str,
    web_sys: &'static str,
}

static DEPENDENCIES: Dependencies = Dependencies {
    base64: "0.22",
    bytes: "1.9",
    chrono: "0.4",
    futures: "0.3",
    getrandom: "0.2",
    gloo_net: "0.6",
    js_sys: "0.3",
    percent_encoding: "2.3",
    rand: "0.8",
    regress: "0.10",
    reqwest: "0.12",
    serde: "1.0",
    serde_json: "1.0",
    serde_urlencoded: "0.7",
    uuid: "1.0",
    wasm_bindgen: "0.2",
    wasm_streams: "0.4",
    web_sys: "0.3",
};

pub fn dependencies(builder: Generator, backend: HttpBackend, include_client: bool) -> Vec<String> {
    let mut deps = vec![
        format!("serde = {{ version = \"{}\", features = [\"derive\"] }}", DEPENDENCIES.serde),
        format!("serde_urlencoded = \"{}\"", DEPENDENCIES.serde_urlencoded),
    ];

    // Add backend-specific dependencies
    match backend {
        HttpBackend::Reqwest => {
            deps.push(format!("bytes = \"{}\"", DEPENDENCIES.bytes));
            deps.push(format!("futures-core = \"{}\"", DEPENDENCIES.futures));
            deps.push(format!("reqwest = {{ version = \"{}\", default-features=false, features = [\"json\", \"stream\"] }}", DEPENDENCIES.reqwest));
        }
        HttpBackend::Gloo => {
            deps.push(format!("futures-core = \"{}\"", DEPENDENCIES.futures));
            deps.push(format!("gloo-net = \"{}\"", DEPENDENCIES.gloo_net));
            deps.push(format!("js-sys = \"{}\"", DEPENDENCIES.js_sys));
            deps.push(format!("wasm-bindgen = \"{}\"", DEPENDENCIES.wasm_bindgen));
            deps.push(format!("wasm-streams = \"{}\"", DEPENDENCIES.wasm_streams));
            deps.push(format!(
                "web-sys = {{ version = \"{}\", features = [\"Blob\", \"File\", \"FormData\", \"Headers\", \"ReadableStream\", \"Request\", \"RequestCredentials\", \"RequestInit\", \"RequestMode\", \"Response\", \"WebSocket\"] }}",
                DEPENDENCIES.web_sys
            ));
        }
    }

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
        let backend_feature = match backend {
            HttpBackend::Reqwest => "reqwest-client",
            HttpBackend::Gloo => "gloo-client",
        };
        let client_version_dep = format!("progenitor-client = {{ version = \"{}\", features = [\"{}\"] }}", crate_version, backend_feature);
        deps.push(client_version_dep);
    }

    if type_space.uses_regress() {
        deps.push(format!("regress = \"{}\"", DEPENDENCIES.regress));
    }
    if type_space.uses_uuid() {
        let uuid_features = match backend {
            HttpBackend::Reqwest => "\"serde\", \"v4\"",
            HttpBackend::Gloo => "\"serde\", \"v4\", \"js\"",
        };
        deps.push(format!(
            "uuid = {{ version = \"{}\", features = [{}] }}",
            DEPENDENCIES.uuid, uuid_features
        ));

        // Add getrandom with js feature, which is needed by uuid on WASM.
        if matches!(backend, HttpBackend::Gloo) {
            deps.push(format!(
                "getrandom = {{ version = \"{}\", features = [\"js\"] }}",
                DEPENDENCIES.getrandom
            ));
        }
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

        // Add getrandom with js feature, which is needed by rand on WASM.
        if matches!(backend, HttpBackend::Gloo) && !type_space.uses_uuid() {
            deps.push(format!(
                "getrandom = {{ version = \"{}\", features = [\"js\"] }}",
                DEPENDENCIES.getrandom
            ));
        }
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
