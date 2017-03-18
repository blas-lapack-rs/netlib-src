extern crate cmake;

use cmake::Config;
use std::{env, fs, os};
use std::path::{Path, PathBuf};

macro_rules! feature(($name:expr) => (env::var(concat!("CARGO_FEATURE_", $name)).is_ok()));
macro_rules! switch(($condition:expr) => (if $condition { "ON" } else { "OFF" }));

fn main() {
    let mut suffix = "";
    let kind = if feature!("STATIC") { "static" } else { "dylib" };
    let cblas = feature!("CBLAS");
    let lapacke = feature!("LAPACKE");
    if !feature!("SYSTEM") {
        suffix = "-netlib";
        let source = PathBuf::from("source");
        if !source.join("CBLAS").join("CMAKE").exists() {
            os::unix::fs::symlink(source.join("CBLAS").join("cmake"),
                                  source.join("CBLAS").join("CMAKE")).unwrap();
        }
        let output = Config::new(&source)
                            .define("BUILD_TESTING", "OFF")
                            .define("BUILD_SHARED_LIBS", switch!(kind == "dylib"))
                            .define("CBLAS", switch!(cblas))
                            .define("LAPACKE_WITH_TMG", switch!(lapacke))
                            .define("CMAKE_INSTALL_LIBDIR", "lib")
                            .build();
        let output = output.join("lib");
        rename(&output, "blas", suffix);
        rename(&output, "lapack", suffix);
        println!("cargo:rustc-link-search={}", output.display());
    }
    println!("cargo:rustc-link-lib={}=blas{}", kind, suffix);
    println!("cargo:rustc-link-lib={}=lapack{}", kind, suffix);
    println!("cargo:rustc-link-lib=dylib=gfortran");
    if cblas {
        println!("cargo:rustc-link-lib={}=cblas", kind);
    }
    if lapacke {
        println!("cargo:rustc-link-lib={}=lapacke", kind);
        println!("cargo:rustc-link-lib={}=tmglib", kind);
    }
}

fn rename(directory: &Path, name: &str, suffix: &str) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        if !stem.starts_with("lib") {
            continue;
        }
        if &stem[3..] != name {
            continue;
        }
        let mut new_path = path.clone();
        new_path.set_file_name(format!("lib{}{}", name, suffix));
        new_path.set_extension(path.extension().unwrap());
        fs::rename(&path, &new_path).unwrap();
        return;
    }
}
