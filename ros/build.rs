use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let obj = out_dir.join("boot.o");

    let status = Command::new("i686-elf-as")
        .arg("src/boot.s")
        .arg("-o")
        .arg(&obj)
        .status()
        .expect("failed to run i686-elf-as");

    assert!(status.success(), "assembler failed");

    // Tell cargo to link boot.o
    println!("cargo:rustc-link-arg={}", obj.display());
    println!("cargo:rerun-if-changed=src/boot.s");
}
