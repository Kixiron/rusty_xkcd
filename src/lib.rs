// Copyright 2019 Kixiron
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Rusty Xkcd
//!
//! [![Crates.io](https://img.shields.io/crates/v/rusty_xkcd.svg)](https://crates.io/crates/rusty_xkcd)
//! [![Docs.rs](https://docs.rs/rusty_xkcd/badge.svg)](https://docs.rs/rusty_xkcd)
//! [![GitLab](https://img.shields.io/badge/GitLab-Mirror-Orange.svg)](https://gitlab.com/Kixiron/rusty_xkcd)
//! [![Travis CI](https://img.shields.io/travis/Kixiron/rusty_xkcd.svg?branch=master&label=Travis%20build)](https://travis-ci.org/Kixiron/rusty_xkcd)
//! [![GitLab CI](https://img.shields.io/gitlab/pipeline/kixiron/rusty_xkcd.svg?branch=master&label=GitLab%20build)](https://gitlab.com/Kixiron/rusty_xkcd/pipelines)
//! [![License](https://img.shields.io/github/license/kixiron/rusty_xkcd.svg)](https://github.com/Kixiron/rusty_xkcd/blob/master/LICENSE)
//! [![Language](https://img.shields.io/github/languages/top/kixiron/rusty_xkcd.svg)](https://github.com/Kixiron/rusty_xkcd)
//! [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat)](https://github.com/Kixiron/rusty_xkcd/pull/new/master)
//!
//! Rusty Xkcd is an API wrapper for the [Xkcd API](https://xkcd.com/json.html).
//! It aims to give full access to the Xkcd API with maximum performance, control, and reliability.
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
//! Comic data is stored in the following format:  
//! `title` The title of the xkcd comic  
//! `url` The url of the xkcd comic  
//! `img_url` The image url of the xkcd comic  
//! `alt_text` The alternate/tooltip text of the xkcd comic  
//! `number` The number of the xkcd comic  
//! `date` The date that the xkcd comic was published  
//!
//! # Coming Soon
//!
//! [Explain Xkcd](https://www.explainxkcd.com)  
//! [What If](https://what-if.xkcd.com)

#[macro_use]
extern crate quick_error;

mod errors;
/// ## Errors
/// The module containing all errors for rusty_xkcd
pub use errors::Error;

/// ## Comics
/// The module containing all comic logic and interfaces for rusty_xkcd
mod comics;
pub use comics::Comic;
pub use comics::Comics;


mod explanation;
pub use explanation::Explanation;
