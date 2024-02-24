#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod confusables;
mod constants;
pub mod sentence;
pub mod word;
mod word_group;
mod word_part;
