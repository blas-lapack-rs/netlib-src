extern crate cmake;

use cmake::Config;
use std::env::var;
use std::fs::metadata;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if var("CARGO_FEATURE_SYSTEM_NETLIB").is_ok() {
        use_system();
    } else {
        use_bundled();
    }
}

fn use_system() {
    let kind = if var("CARGO_FEATURE_STATIC_NETLIB").is_ok() { "static" } else { "dylib" };
    let cblas = var("CARGO_FEATURE_EXCLUDE_CBLAS").is_err();
    let lapacke = var("CARGO_FEATURE_EXCLUDE_LAPACKE").is_err();

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

fn use_bundled() {
    let kind = if var("CARGO_FEATURE_STATIC_NETLIB").is_ok() { "static" } else { "dylib" };
    let cblas = var("CARGO_FEATURE_EXCLUDE_CBLAS").is_err();
    let lapacke = var("CARGO_FEATURE_EXCLUDE_LAPACKE").is_err();

    let source = PathBuf::from(&var("CARGO_MANIFEST_DIR").unwrap()).join("source");

    if metadata(&source.join("CBLAS/CMAKE")).is_err() {
        let _ = Command::new("ln")
                        .args(&["-s", "cmake", "CMAKE"])
                        .current_dir(&source.join("CBLAS"))
                        .status();
    }

    let output = Config::new(&source)
                        .define("BUILD_TESTING", "OFF")
                        .define("BUILD_SHARED_LIBS", if kind == "dylib" { "ON" } else { "OFF" })
                        .define("CBLAS", if cblas { "ON" } else { "OFF" })
                        .define("LAPACKE", if lapacke { "ON" } else { "OFF" })
                        .build();

    println!("cargo:rustc-link-search={}", output.join("lib").display());
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
