// Copyright 2022 Oxide Computer Company

//! Support for generated clients.

#![deny(missing_docs)]

// Enforce mutual exclusivity of backend features
#[cfg(all(feature = "reqwest-client", feature = "gloo-client"))]
compile_error!("Cannot enable both reqwest-client and gloo-client features");

#[cfg(not(any(feature = "reqwest-client", feature = "gloo-client")))]
compile_error!("Must enable either reqwest-client or gloo-client feature");

// Shared code used by both backends
mod shared;
pub use shared::*;

// reqwest backend
#[cfg(feature = "reqwest-client")]
mod reqwest_impl;
#[cfg(feature = "reqwest-client")]
pub use reqwest_impl::*;

// gloo-net backend
#[cfg(feature = "gloo-client")]
mod gloo_impl;
#[cfg(feature = "gloo-client")]
pub use gloo_impl::*;

// For stand-alone crates, rather than adding a dependency on
// progenitor-client, we simply dump the code right in. This means we don't
// need to determine the provenance of progenitor (crates.io, github, etc.)
// when generating the stand-alone crate.
#[doc(hidden)]
#[cfg(feature = "reqwest-client")]
pub fn code() -> &'static str {
    include_str!("reqwest_impl.rs")
}

#[doc(hidden)]
#[cfg(feature = "gloo-client")]
pub fn code() -> &'static str {
    include_str!("gloo_impl.rs")
}
