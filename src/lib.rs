#[macro_use]
extern crate quick_error;

/// # Rusty xkcd
/// [![Build Status](https://travis-ci.org/Kixiron/rusty_xkcd.svg?branch=master)](https://travis-ci.org/Kixiron/rusty_xkcd)
/// Rusty xkcd is an API wrapper for the [xkcd API](). It aims to give full access to the xkcd API with maximum preformance, control, and reliability.

/// ## Errors
/// The module containing all errors for rusty_xkcd
mod errors;
pub use errors::Error;

/// ## Comics
/// The module containing all comic logic and interfaces for rusty_xkcd
mod comics;
pub use comics::Comic;
