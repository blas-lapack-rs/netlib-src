//! Source of [BLAS] and [LAPACK] via [Netlib].
//!
//! ## [Architecture]
//!
//! ## Configuration
//!
//! The following Cargo features are supported:
//!
//! * `cblas` to build CBLAS (enabled by default),
//! * `lapacke` to build LAPACKE (enabled by default),
//! * `static` to link to Netlib statically,
//! * `system` to skip building the bundled Netlib, and
//! * `tmg` to build TMG (enabled by default).
//!
//! [architecture]: https://blas-lapack-rs.github.io/architecture
//! [blas]: https://en.wikipedia.org/wiki/BLAS
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//! [netlib]: http://www.netlib.org/

#![no_std]
