// Copyright 2026 Oxide Computer Company

use std::{env, fs, path::Path};

fn main() {
    // Example build script showing integration with OUT_DIR.
    //
    // This build script copies a sample OpenAPI document to OUT_DIR. A more
    // complex build script might, for example, extract the OpenAPI document
    // from a non-file source, writing it out to OUT_DIR.
    let src = "../sample_openapi/keeper.json";
    println!("cargo:rerun-if-changed={}", src);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("keeper.json");
    fs::copy(src, dest).unwrap();
}
