// iunorm
//
//! Mapping normalized floating point numbers to (un)signed integers.
//!
//! This library defines multiple types of sizes ranging between 8 and 128-bit,
//! that can represent a normalized value (between 0 and 1 if unsigned, or -1
//! and 1 if signed), mapping the range to its `::MIN` & `::MAX` boundaries.
//

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]
#![forbid(unsafe_code)]
#![no_std]

mod generate;
pub use generate::*;
