# netlib-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a source of [BLAS] and [LAPACK] via [Netlib].

## [Architecture]

## Configuration

The following Cargo features are supported:

* `cblas` to build CBLAS (enabled by default),
* `lapacke` to build LAPACKE (enabled by default),
* `static` to link to Netlib statically,
* `system` to skip building the bundled Netlib, and
* `tmg` to build TMG (enabled by default).

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[architecture]: https://blas-lapack-rs.github.io/architecture
[blas]: https://en.wikipedia.org/wiki/BLAS
[lapack]: https://en.wikipedia.org/wiki/LAPACK
[netlib]: http://www.netlib.org/

[build-img]: https://github.com/blas-lapack-rs/netlib-src/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/blas-lapack-rs/netlib-src/actions/workflows/build.yml
[documentation-img]: https://docs.rs/netlib-src/badge.svg
[documentation-url]: https://docs.rs/netlib-src
[package-img]: https://img.shields.io/crates/v/netlib-src.svg
[package-url]: https://crates.io/crates/netlib-src
