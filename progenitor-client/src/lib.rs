// Copyright 2022 Oxide Computer Company

mod progenitor_client;

pub use crate::progenitor_client::*;

#[doc(hidden)]
pub fn code() -> &'static str {
    include_str!("progenitor_client.rs")
}
