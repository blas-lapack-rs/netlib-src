# Netlib Provider [![Version][version-img]][version-url]

The package provides [BLAS][1] and [LAPACK][2] using the [Netlib][3]
implementation. By default, the package will build and use a bundled Netlib,
which requires a Fortran and C compiler.

The following Cargo features are supported:

* `static-netlib` to link to Netlib statically,
* `system-netlib` to skip building the bundled Netlib,
* `exclude-cblas` to skip building CBLAS, and
* `exclude-lapacke` to skip building LAPACKE.

## Where are all the FFI definitions?

This package provides only an implementation of BLAS and LAPACK. Bindings are
available in [blas-sys][4] and [lapack-sys][5], and wrappers are available in
[blas][6] and [lapack][7].

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
[2]: https://en.wikipedia.org/wiki/LAPACK
[3]: http://www.netlib.org/lapack

[4]: https://github.com/stainless-steel/blas-sys
[5]: https://github.com/stainless-steel/lapack-sys
[6]: https://github.com/stainless-steel/blas
[7]: https://github.com/stainless-steel/lapack

[version-img]: https://img.shields.io/crates/v/netlib-provider.svg
[version-url]: https://crates.io/crates/netlib-provider
