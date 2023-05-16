fn main() {
    let mut opts = built::Options::default();
    opts.set_dependencies(true);

    let src = project_root::get_project_root().unwrap();
    let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap())
        .join("built.rs");
    built::write_built_file_with_opts(&opts, src.as_ref(), &dst)
        .expect("Failed to acquire build-time information");
}
