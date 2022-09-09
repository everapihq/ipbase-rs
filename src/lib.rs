//! # ipbase-rs
//!
//! The [Ipbase][ipbase] crate provides an easy to use wrapper over the
//! [ipbase api][ipbase_api].
//!
//! * Easy to use
//! * Async api calls with [reqwest][reqwest]
//! * Ready deserialized structs of the ipbase responses
//! * Manages authentication for you, just pass your api token once
//!
//! ## Requirements
//! * Your own [ipbase api key][ipbase_api]
//! * Async runtime configured e.g. [tokio][tokio]
//!
//!
//!
//! ## Examples
//!
//!
//! ## Optional Features
//!
//!
//! ## Troubleshooting
//! If you get a ResponseParsingError during usage of the crate this is very likely
//! due to an invalid input where the ipbase api will throw an error or
//! due to some unexpected values that were returned by the api. E.g. sometimes the api
//! will return `false` instead of a number for certain fields or other fields were missing.
//!
//! In this case please check if your input is valid and if so create a bug report on the
//! crate [repository][ipbase_rs_repo] and provide some information about your input.
//!
//! [ipbase]: ./api/struct.Ipbase.html
//! [ipbase_rs_repo]: https://github.com/everapihq/ipbase-rs
//! [ipbase_api]: https://api.ipbase.com/
//! [reqwest]: https://crates.io/crates/reqwest
//! [tokio]: https://crates.io/crates/tokio

#![warn(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate thiserror;

pub mod api;
mod error;
pub mod models;
mod utils;

pub use api::Ipbase;
pub use error::IpbaseError as Error;
