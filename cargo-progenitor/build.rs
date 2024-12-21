// Copyright 2023 Oxide Computer Company

fn main() {
    let src = project_root::get_project_root().unwrap();
    let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
    built::write_built_file_with_opts(Some(&src), &dst)
        .expect("Failed to acquire build-time information");
}
