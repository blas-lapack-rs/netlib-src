extern crate cmake;

use cmake::Config;
use std::env::var;
use std::fs::metadata;
use std::path::PathBuf;
use std::process::Command;

macro_rules! feature(($name:expr) => (var(concat!("CARGO_FEATURE_", $name)).is_ok()));
macro_rules! switch(($condition:expr) => (if $condition { "ON" } else { "OFF" }));

fn main() {
    let kind = if feature!("STATIC") { "static" } else { "dylib" };
    let cblas = feature!("CBLAS");
    let lapacke = feature!("LAPACKE");
    let testing = feature!("TESTING");

    if !feature!("SYSTEM") {
        let source = PathBuf::from("source");

        if metadata(&source.join("CBLAS/CMAKE")).is_err() {
            let _ = Command::new("ln")
                            .args(&["-s", "cmake", "CMAKE"])
                            .current_dir(&source.join("CBLAS"))
                            .status();
        }

        let output = Config::new(&source)
                            .define("BUILD_TESTING", switch!(testing))
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
    if testing {
        println!("cargo:rustc-link-lib={}=tmglib", kind);
    }
}
