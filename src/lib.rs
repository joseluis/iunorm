// iunorm
//
//! Mapping normalized floating point numbers to (un)signed integers.
//!
//! This library defines multiple types of sizes ranging between 8 and 128-bit,
//! that can represent a normalized value (between 0 and 1 if unsigned, or -1
//! and 1 if signed), mapping the range to its `::MIN` & `::MAX` boundaries.
//!
//! # Example
//!
//! ```
//! # use iunorm::*;
//! assert_eq![u64::MAX, Unorm64::from_f64( 1.0).into()];
//! assert_eq![0.5, Unorm64(u64::MAX / 2).into()];
//!
//! assert_eq![i16::MIN, Inorm16::from_f32(-1.0).into()];
//! assert_eq![-1.0, Inorm16(i16::MIN).into()];
//! ```
//!
//! Values less
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

#[cfg(test)]
mod test;

mod generate;
pub use generate::*;
