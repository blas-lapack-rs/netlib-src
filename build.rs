use std::env;
use std::path::*;
use std::process::*;

fn main() {
    let kind = "static";

    let lapack = !env::var("CARGO_FEATURE_EXCLUDE_LAPACK").is_ok();

    if !env::var("CARGO_FEATURE_SYSTEM_NETLIB").is_ok() {
        let cblas = !env::var("CARGO_FEATURE_EXCLUDE_CBLAS").is_ok();

        let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("lapack");
        let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());

        run(Command::new("cmake")
                    .arg(&src)
                    .arg("-DCMAKE_Fortran_FLAGS='-O2 -frecursive -fPIC'")
                    .arg(&format!("-DCBLAS={}", if cblas { "on" } else { "off" }))
                    .arg(&format!("-DLAPACKE={}", if lapack { "on" } else { "off" }))
                    .current_dir(&dst), "cmake");

        run(Command::new("cmake")
                    .arg("--build").arg(".")
                    .arg("--")
                    .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
                    .current_dir(&dst), "cmake");

        println!("cargo:rustc-link-search={}", dst.join("lib").display());
    }

    println!("cargo:rustc-link-lib=dylib=gfortran");
    println!("cargo:rustc-link-lib={}=blas", kind);
    if lapack {
        println!("cargo:rustc-link-lib={}=lapack", kind);
        println!("cargo:rustc-link-lib={}=lapacke", kind);
    }
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(error) => fail(&format!("failed to execute `{}`: {}", program, error)),
    };
    if !status.success() {
        fail(&format!("`{}` failed: {}", program, status));
    }
}

fn fail(message: &str) -> ! {
    panic!("\n{}\n\nbuild script failed", message)
}
