# Netlib Provider

The package provides [BLAS][1] and [LAPACK][2] using the [Netlib][3]
implementation. By default, the package will build and use a bundled Netlib,
which requires a Fortran and C compiler available.

The following two Cargo features are supported:

- `system-netlib` to skip building the bundled Netlib and
- `blas-only` to skip building only LAPACK.

## Where are all the FFI definitions?

This package only provides an implementation of BLAS and LAPACK. Bindings are
available in [libblas-sys][4] and [liblapack-sys][5], and wrappers are available
in [blas][6] and [lapack][7].

[1]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
[2]: https://en.wikipedia.org/wiki/LAPACK
[3]: http://www.netlib.org/lapack
[4]: https://github.com/stainless-steel/libblas-sys
[5]: https://github.com/stainless-steel/liblapack-sys
[6]: https://github.com/stainless-steel/blas
[7]: https://github.com/stainless-steel/lapack
