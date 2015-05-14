use std::env;
use std::path::*;
use std::process::*;

fn main() {
    let kind = "static";

    if !env::var("CARGO_FEATURE_SYSTEM_NETLIB").is_ok() {
        let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("lapack");
        let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());

        run(Command::new("cmake")
                    .arg(&src)
                    .arg("-DCMAKE_Fortran_FLAGS='-O2 -frecursive -fPIC'")
                    .arg("-DLAPACKE=on")
                    .current_dir(&dst), "cmake");

        run(Command::new("cmake")
                    .arg("--build").arg(".")
                    .arg("--")
                    .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
                    .current_dir(&dst), "cmake");

        println!("cargo:rustc-link-search={}", dst.join("lib").display());
    }

    println!("cargo:rustc-link-lib=dylib=gfortran");
    if !env::var("CARGO_FEATURE_BLAS_ONLY").is_ok() {
        println!("cargo:rustc-link-lib={}=lapack", kind);
        println!("cargo:rustc-link-lib={}=lapacke", kind);
        println!("cargo:rustc-link-lib={}=blas", kind);
    } else {
        println!("cargo:rustc-link-lib={}=blas", kind);
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
