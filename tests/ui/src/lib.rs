#![cfg(test)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate bitflags_derive;

mod debug;
mod display;
mod from_str;

#[cfg(feature = "serde")]
mod serde;
