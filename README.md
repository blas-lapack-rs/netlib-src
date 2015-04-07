# netlib-provider

A crate for statically linking to netlib BLAS. By default, this crate will
build and use a bundled BLAS. Building this will require a Fortran and C
compiler available.  This provides BLAS and LAPACK.

Two Cargo features are supported:

- `system-netlib`: don't use the bundled BLAS/LAPACK.
- `blas-only`: don't link to LAPACK

# Where are all the FFI definitions?

This crate only provides a BLAS implementation. Bindings are available at https://github.com/stainless-steel/libblas-sys,
and a wrapper is available at https://github.com/stainless-steel/blas.
