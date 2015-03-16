use std::path::*;
use std::process::*;
use std::env;
use std::io::*;

fn main() {
    let kind = "static";

    if !env::var("CARGO_FEATURE_SYSTEM_NETLIB").is_ok() {
        let mut cflags = env::var("CFLAGS").unwrap_or(String::new());
        let target = env::var("TARGET").unwrap();

        if target.contains("i686") {
            cflags.push_str(" -m32");
        } else if target.contains("x86_64") {
            cflags.push_str(" -m64");
        }

        if !target.contains("i686") {
            cflags.push_str(" -fPIC");
        }

        let src = PathBuf::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("lapack");
        let dst = PathBuf::new(&env::var("OUT_DIR").unwrap());


        // how annoying! it seems to assume case-insensitive, and look in *both* places...
        run(Command::new("cp").arg("-r").arg(&src.join("CBLAS/cmake")).arg(&src.join("CBLAS/CMAKE")), "cp");

        run(Command::new("cmake").current_dir(&dst)
             .arg(&src)
             .arg("-DCMAKE_Fortran_FLAGS='-O2 -frecursive -fPIC'")
             .arg("-DCBLAS=on")
             .arg("-DLAPACKE=on"), "cmake");

        run(Command::new("cmake").current_dir(&dst)
             .arg("--build").arg(".")
             .arg("--target").arg("cblas")
             .arg("--target").arg("lapacke")
             .arg("--")
             .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap_or(String::from_str("1")))), "cmake");

        println!("cargo:rustc-flags=-L {}", dst.join("lib").display());
    }

    println!("cargo:rustc-flags=-l {}=blas", kind);
    if !env::var("CARGO_FEATURE_BLAS_ONLY").is_ok() {
        println!("cargo:rustc-flags=-l {}=lapack", kind);
        println!("cargo:rustc-flags=-l {}=lapacke", kind);
    }
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::FileNotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
