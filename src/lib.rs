//! Source of [BLAS] and [LAPACK] via [Netlib].
//!
//! The usage of the package is explained [here][usage].
//!
//! The following Cargo features are supported:
//!
//! * `cblas` to build CBLAS (enabled by default),
//! * `lapacke` to build LAPACKE (enabled by default),
//! * `static` to link to Netlib statically,
//! * `system` to skip building the bundled Netlib, and
//! * `tmg` to build TMG (enabled by default).
//!
//! [blas]: https://en.wikipedia.org/wiki/BLAS
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//! [netlib]: http://www.netlib.org/
//! [usage]: https://blas-lapack-rs.github.io/usage

#![no_std]
