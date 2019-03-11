//! # Rusty Xkcd
//!
//! [![Build Status](https://travis-ci.org/Kixiron/rusty_xkcd.svg?branch=master)](https://travis-ci.org/Kixiron/rusty_xkcd)
//! [![Docs.rs](https://docs.rs/rusty_xkcd/badge.svg)](https://docs.rs/rusty_xkcd)
//! [![Crates.io](https://img.shields.io/crates/v/rusty_xkcd.svg)](https://crates.io/crates/rusty_xkcd)
//! [![License](https://img.shields.io/github/license/kixiron/rusty_xkcd.svg)](https://github.com/Kixiron/rusty_xkcd/blob/master/LICENSE)
//!
//! Rusty Xkcd is an API wrapper for the [Xkcd API]().
//! It aims to give full access to the Xkcd API with maximum preformance, control, and reliability.
//!
//! ## Xkcd Comics
//!
//! Using rusty_xkcd to interact with the xkcd api is easy!
//!
//! To start, add this line to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! rusty_xkcd = "0.1.1"
//! ```
//!
//! Then import the crate to wherever you want to use it
//!
//! ```rust
//! extern crate rusty_xkcd;
//! use rusty_xkcd::Comic;
//! ```
//!
//! Now you're ready to start using the `Comic` API!  
//! To get the latest xkcd comic, use `get_latest_comic()`
//!
//! ```rust
//! # extern crate rusty_xkcd;
//! # use rusty_xkcd::Comic;
//! let comic = Comic::get_latest_comic().unwrap();
//! ```
//!
//! To get a comic by number, use `get_comic()`
//!
//! ```rust
//! # extern crate rusty_xkcd;
//! # use rusty_xkcd::Comic;
//! let comic = Comic::get_comic(100).unwrap();
//! ```
//!
//! And finally, to get a random xkcd comic, use `get_random_comic()`
//!
//! ```rust
//! # extern crate rusty_xkcd;
//! # use rusty_xkcd::Comic;
//! let comic = Comic::get_random_comic().unwrap();
//! ```
//!
//! ### The Comic Struct
//!
//! The `Comic` struct contains both the static methods for getting an xkcd comic
//! (`get_comic()`, etc.),
//! as well as the comic data.
//! 
//! # Coming Soon
//! [Explain Xkcd](https://www.explainxkcd.com)
//! [What If](https://what-if.xkcd.com)
//!

#[macro_use]
extern crate quick_error;

/// ## Errors
/// The module containing all errors for rusty_xkcd
mod errors;
pub use errors::Error;

/// ## Comics
/// The module containing all comic logic and interfaces for rusty_xkcd
mod comics;
pub use comics::Comic;
