#![no_std]
#![allow(
  unused_macros,
  non_camel_case_types
)]
#![feature(
  lang_items
)]

#[macro_use]
mod macros;

mod arch;
pub use arch::*;

mod basic;
pub use basic::*;

mod api;
pub use api::*;

// C library
mod errno;
mod locale;
mod string;
mod wchar;
