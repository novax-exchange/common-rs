#[cfg(feature = "build_script")]
use std::path::PathBuf;

#[cfg(feature = "build_script")]
fn main() {
    let out_dir = PathBuf::from("src/svc");
    tonic_build::configure()
        .out_dir(out_dir)
        .compile(&["proto/sample.proto"], &["proto"])
        .unwrap();
}

#[cfg(not(feature = "build_script"))]
fn main() {}