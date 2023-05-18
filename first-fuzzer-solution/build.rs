use std::process::Command;

fn main() {
    let status = Command::new("cargo")
        .arg("rustc")
        .arg("-p")
        .arg("first-target-solution")
        .arg("--target-dir")
        .arg("target/fuzz-target/")
        .arg("--")
        .arg("-C")
        .arg("prefer-dynamic")
        .arg("-C")
        .arg("passes=sancov-module")
        .arg("-C")
        .arg("llvm-args=-sanitizer-coverage-level=3")
        .arg("-C")
        .arg("llvm-args=-sanitizer-coverage-inline-8bit-counters")
        .arg("-C")
        .arg("llvm-args=-sanitizer-coverage-prune-blocks=0")
        .arg("-C")
        .arg("link-dead-code")
        .arg("-C")
        .arg("lto=no")
        .arg("--emit=dep-info,link")
        .status()
        .expect("Failed to spawn Cargo");

    assert!(status.success(), "Target build command failed");

    println!("cargo:rustc-link-lib=first_target_solution");
    println!(
        "cargo:rustc-link-search={}/target/fuzz-target/debug/",
        env!("CARGO_MANIFEST_DIR")
    );
}
