use std::env::var;
use std::fs;
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
    let lapacke = var("CARGO_FEATURE_EXCLUDE_LAPACKE").is_err();

    println!("cargo:rustc-link-lib=static=blas");
    println!("cargo:rustc-link-lib=static=lapack");
    if lapacke {
        println!("cargo:rustc-link-lib=static=lapacke");
    }
}

fn use_bundled() {
    macro_rules! make(
        ($directory:expr, $target: expr) => (
            run(Command::new("make")
                        .arg($target)
                        .arg(&format!("-j{}", var("NUM_JOBS").unwrap()))
                        .arg("OPTS=-O2 -frecursive -fPIC")
                        .arg("NOOPT=-O2 -frecursive -fPIC")
                        .arg("CFLAGS=-O3 -fPIC")
                        .current_dir($directory));
        );
    );

    macro_rules! ok(
        ($result:expr) => ( match $result {
            Ok(ok) => ok,
            Err(error) => panic!("`{}` failed with `{}`", stringify!($result), error),
        });
    );

    macro_rules! cp(
        ($source:expr, $destination:expr) => (ok!(fs::copy($source, $destination)));
    );

    let cblas = var("CARGO_FEATURE_EXCLUDE_CBLAS").is_err();
    let lapacke = var("CARGO_FEATURE_EXCLUDE_LAPACKE").is_err();

    let source = PathBuf::from(&var("CARGO_MANIFEST_DIR").unwrap()).join("lapack");
    let output = PathBuf::from(&var("OUT_DIR").unwrap());

    cp!(source.join("make.inc.example"), source.join("make.inc"));

    println!("cargo:rustc-link-lib=dylib=gfortran");
    println!("cargo:rustc-link-search={}", output.display());

    make!(&source, "blaslib");
    cp!(source.join("librefblas.a"), output.join("libblas.a"));
    println!("cargo:rustc-link-lib=static=blas");

    make!(&source, "lapacklib");
    cp!(source.join("liblapack.a"), output.join("liblapack.a"));
    println!("cargo:rustc-link-lib=static=lapack");

    if cblas {
        make!(&source.join("CBLAS"), "all");
        cp!(source.join("libcblas.a"), output.join("libcblas.a"));
        println!("cargo:rustc-link-lib=static=cblas");
    }

    if lapacke {
        make!(&source.join("LAPACKE"), "all");
        cp!(source.join("liblapacke.a"), output.join("liblapacke.a"));
        println!("cargo:rustc-link-lib=static=lapacke");
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
