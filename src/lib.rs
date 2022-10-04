// iunorm
//
//! Mapping normalized floating point numbers to (un)signed integers.
//!
//! # Examples
//!
//! ```
//! # use iunorm::*;
//! assert_eq![u64::MAX, Unorm64::from_f64( 1.0).into()];
//! assert_eq![0.5, Unorm64(u64::MAX / 2).into()];
//!
//! assert_eq![i16::MIN, Inorm16::from_f32(-1.0).into()];
//! assert_eq![-1.0, Inorm16(i16::MIN).into()];
//! ```
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

/// Scales an `f32` `x` value in between `[xmin..xmax]` to a new range `[a..b]`.
///
/// $$
/// x' = (b - a) \frac{x - xmin}{xmax - xmin} + a
/// $$
///
/// # Examples
/// ```
/// # use iunorm::scale32;
/// assert_eq![0.25, scale32(90., 0., 360., 0., 1.)];
/// assert_eq![-0.5, scale32(90., 0., 360., -1., 1.)];
/// ```
#[inline]
#[must_use]
pub fn scale32(x: f32, xmin: f32, xmax: f32, a: f32, b: f32) -> f32 {
    (b - a) * (x - xmin) / (xmax - xmin) + a
}

/// Scales an `f64` `x` value in between `[xmin..xmax]` to a new range `[a..b]`.
///
/// $$
/// x' = (b - a) \frac{x - xmin}{xmax - xmin} + a
/// $$
///
/// # Examples
/// ```
/// # use iunorm::scale64;
/// assert_eq![0.25, scale64(90., 0., 360., 0., 1.)];
/// assert_eq![-0.5, scale64(90., 0., 360., -1., 1.)];
/// ```
#[inline]
#[must_use]
pub fn scale64(x: f64, xmin: f64, xmax: f64, a: f64, b: f64) -> f64 {
    (b - a) * (x - xmin) / (xmax - xmin) + a
}
