use std::env::var;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
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
    if cblas {
        println!("cargo:rustc-link-lib={}=cblas", kind);
    }
    println!("cargo:rustc-link-lib={}=lapack", kind);
    if lapacke {
        println!("cargo:rustc-link-lib={}=lapacke", kind);
    }
}

#[allow(unused_must_use)]
fn use_bundled() {
    let kind = if var("CARGO_FEATURE_STATIC_NETLIB").is_ok() { "static" } else { "dylib" };
    let cblas = var("CARGO_FEATURE_EXCLUDE_CBLAS").is_err();
    let lapacke = var("CARGO_FEATURE_EXCLUDE_LAPACKE").is_err();

    let source = PathBuf::from(&var("CARGO_MANIFEST_DIR").unwrap()).join("source");
    let output = PathBuf::from(&var("OUT_DIR").unwrap());

    if fs::metadata(&source.join("CBLAS/CMAKE")).is_err() {
        Command::new("ln")
                .args(&["-s", "cmake", "CMAKE"])
                .current_dir(&source.join("CBLAS"))
                .status();
    }

    run(Command::new("cmake")
                .arg(&source)
                .arg("-DBUILD_TESTING=off")
                .arg(&format!("-DBUILD_SHARED_LIBS={}", if kind == "dylib" { "on" } else { "off" }))
                .arg(&format!("-DCBLAS={}", if cblas { "on" } else { "off" }))
                .arg(&format!("-DLAPACKE={}", if lapacke { "on" } else { "off" }))
                .current_dir(&output));

    run(Command::new("make")
                .arg(&format!("-j{}", var("NUM_JOBS").unwrap()))
                .current_dir(&output));

    println!("cargo:rustc-link-search={}", output.join("lib").display());
    println!("cargo:rustc-link-lib={}=blas", kind);
    if cblas {
        println!("cargo:rustc-link-lib={}=cblas", kind);
    }
    println!("cargo:rustc-link-lib={}=lapack", kind);
    if lapacke {
        println!("cargo:rustc-link-lib={}=lapacke", kind);
    }

    match read("CMAKE_Fortran_COMPILER", &output.join("CMakeCache.txt")) {
        Ok(ref name) => {
            if name.contains("gfortran") {
                println!("cargo:rustc-link-lib=dylib=gfortran");
            }
        },
        Err(error) => panic!("failed to detect Fortran: {}", error),
    }
}

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => if !status.success() {
            panic!("`{:?}` failed: {}", command, status);
        },
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        },
    }
}

fn read(prefix: &str, path: &Path) -> Result<String> {
    let mut file = try!(File::open(path));
    let reader = BufReader::new(&mut file);
    for line in reader.lines() {
        let line = try!(line);
        if line.starts_with(&prefix) {
            return Ok(line)
        }
    }
    Err(Error::new(ErrorKind::Other, format!("failed to find `{}` in {}", prefix, path.display())))
}
