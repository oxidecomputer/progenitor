// Copyright 2021 Oxide Computer Company

use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use openapiv3::OpenAPI;
use progenitor::{GenerationSettings, Generator, InterfaceStyle, TagStyle};
use serde::Deserialize;

#[derive(Parser)]
struct Args {
    /// OpenAPI definition document (JSON)
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
                edition = \"2018\"\n\
                \n\
                [dependencies]\n\
                {}\n\
                \n",
                name,
                version,
                builder.dependencies().join("\n"),
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

fn load<P, T>(p: P) -> Result<T>
where
    P: AsRef<Path>,
    for<'de> T: Deserialize<'de>,
{
    let p = p.as_ref();
    let f = File::open(p)?;
    Ok(serde_json::from_reader(f)?)
}

// TODO some or all of this validation should be in progenitor-impl
pub fn load_api<P>(p: P) -> Result<OpenAPI>
where
    P: AsRef<Path>,
{
    let api: OpenAPI = load(p)?;

    if api.openapi != "3.0.3" {
        /*
         * XXX During development we are being very strict, but this should
         * probably be relaxed.
         */
        bail!("unexpected version {}", api.openapi);
    }

    if !api.servers.is_empty() {
        bail!("servers not presently supported");
    }

    if api.security.is_some() {
        bail!("security not presently supported");
    }

    let mut opids = HashSet::new();
    for p in api.paths.paths.iter() {
        match p.1 {
            openapiv3::ReferenceOr::Reference { reference: _ } => {
                bail!("path {} uses reference, unsupported", p.0);
            }
            openapiv3::ReferenceOr::Item(item) => {
                /*
                 * Make sure every operation has an operation ID, and that each
                 * operation ID is only used once in the document.
                 */
                item.iter().try_for_each(|(_, o)| {
                    if let Some(oid) = o.operation_id.as_ref() {
                        if !opids.insert(oid.to_string()) {
                            bail!("duplicate operation ID: {}", oid);
                        }

                        if !o.servers.is_empty() {
                            bail!("op {}: servers, unsupported", oid);
                        }

                        if o.security.is_some() {
                            bail!("op {}: security, unsupported", oid);
                        }
                    } else {
                        bail!("path {} is missing operation ID", p.0);
                    }
                    Ok(())
                })?;

                if !item.servers.is_empty() {
                    bail!("path {} has servers; unsupported", p.0);
                }
            }
        }
    }

    Ok(api)
}
