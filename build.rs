extern crate riscv_target;

use riscv_target::Target;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    if target.starts_with("riscv") {
        let mut target = Target::from_target_str(&target);
        target.retain_extensions("if");

        let target = target.to_string();

        fs::copy(
            format!("bin/trap_{}.a", target),
            out_dir.join(format!("lib{}.a", name)),
        )
        .unwrap();
        // link our trap handler
        println!("cargo:rustc-link-lib=static={}", name);

        println!("cargo:rustc-link-search={}", out_dir.display());

        // Put the linker script somewhere the linker can find it
        fs::File::create(out_dir.join("hal_defaults.x"))
            .unwrap()
            .write_all(include_bytes!("hal_defaults.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out_dir.display());
        fs::File::create(out_dir.join("memory.x"))
            .unwrap()
            .write_all(include_bytes!("hal_defaults.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out_dir.display());
    }

    // Either link the run from RAM linker script or the standard run from flash one
    #[cfg(not(feature = "ramexec"))]
    println!("cargo:rustc-link-arg-examples=-Tmemory.x");
    #[cfg(feature = "ramexec")]
    println!("cargo:rustc-link-arg-examples=-Trun_from_ram.x");

    println!("cargo:rustc-link-arg-examples=-Tlink.x");
    println!("cargo:rustc-link-arg-examples=-Thal_defaults.x");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hal_defaults.x");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=run_from_ram.x");
}
