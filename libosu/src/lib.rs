//! # libosu
//!
//! `libosu` is an attempt to make a convenient library for writing osu!-related programs. It
//! includes data structures and parsers for beatmaps, replays, and more.
//!
//! Please note that until this crate hits `1.0`, none of the APIs in this crate will be stable, so
//! take care when using this crate.

#![warn(missing_docs)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod beatmap;
mod enums;
mod hitobject;
mod hitsounds;
mod point;
mod serde;
mod timing;

pub use beatmap::*;
pub use enums::*;
pub use hitobject::*;
pub use hitsounds::*;
pub use point::*;
pub use serde::*;
pub use timing::*;

/// Says "hello there"
#[deprecated]
pub fn say_hello_there() {
    println!("hello there");
}
