use std::env;
use std::io::*;
use std::path::*;
use std::process::*;

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

        let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("lapack");
        let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());

        if !target.contains("apple") {
            // same reason as below
            let _ = Command::new("ln")
                            .arg("-s")
                            .arg(&src.join("CBLAS/cmake"))
                            .arg(&src.join("CBLAS/CMAKE")).status();
        }

        // we ignore this result. why? because you can't run `cmake` more than twice with this
        // setup :(
        let _ = Command::new("cmake")
                        .arg(&src)
                        .arg("-DCMAKE_Fortran_FLAGS='-O2 -frecursive -fPIC'")
                        .arg("-DCBLAS=on")
                        .arg("-DLAPACKE=on")
                        .arg(&format!("-DCMAKE_C_FLAGS={}", cflags))
                        .current_dir(&dst).status();

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
        println!("cargo:rustc-link-lib={}=cblas", kind);
    } else {
        println!("cargo:rustc-link-lib={}=blas", kind);
        println!("cargo:rustc-link-lib={}=cblas", kind);
    }
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
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
