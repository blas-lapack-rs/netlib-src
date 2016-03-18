extern crate cmake;

use cmake::Config;
use std::env::var;
use std::fs::metadata;
use std::path::PathBuf;
use std::process::Command;

macro_rules! switch(($condition:expr) => (if $condition { "ON" } else { "OFF" }));

fn main() {
    let kind = if var("CARGO_FEATURE_STATIC").is_ok() { "static" } else { "dylib" };
    let cblas = var("CARGO_FEATURE_CBLAS").is_ok();
    let lapacke = var("CARGO_FEATURE_LAPACKE").is_ok();

    if !var("CARGO_FEATURE_SYSTEM").is_ok() {
        let source = PathBuf::from(&var("CARGO_MANIFEST_DIR").unwrap()).join("source");

        if metadata(&source.join("CBLAS/CMAKE")).is_err() {
            let _ = Command::new("ln")
                            .args(&["-s", "cmake", "CMAKE"])
                            .current_dir(&source.join("CBLAS"))
                            .status();
        }

        let output = Config::new(&source)
                            .define("BUILD_TESTING", "OFF")
                            .define("BUILD_SHARED_LIBS", switch!(kind == "dylib"))
                            .define("CBLAS", switch!(cblas))
                            .define("LAPACKE", switch!(lapacke))
                            .define("CMAKE_INSTALL_LIBDIR", "lib")
                            .build();

        println!("cargo:rustc-link-search={}", output.join("lib").display());
    }

    println!("cargo:rustc-link-lib={}=blas", kind);
    println!("cargo:rustc-link-lib={}=lapack", kind);
    println!("cargo:rustc-link-lib=dylib=gfortran");
    if cblas {
        println!("cargo:rustc-link-lib={}=cblas", kind);
    }
    if lapacke {
        println!("cargo:rustc-link-lib={}=lapacke", kind);
    }
}
