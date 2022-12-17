// iunorm
//
//! This library facilitates mapping between floating-point numbers, normalized
//! to some range, and (un)signed integers.
//!
//! - `Inorm` types are a new-type over **signed** integers that *by default*
//! map to floating-point range of `-1..1`.
//! - `Unorm` types are a new-type over **unsigned** integers that *by default*
//! map to floating-point range of `0..1`.
//!
//! The *by default* mapping is used by the `From`/`Into` traits and the
//! `from_f32`/`to_f32` methods, and the equivalent `f64` versions.
//!
//! Additionally both `Inorm` & `Unorm` types can map from/to any custom
//! floating-point range by using the `from_f32_minmax`/`to_f32_minmax` methods,
//! and the equivalent `f64` versions.
//!
//! # Examples
//!
//! ```
//! # use iunorm::*;
//! # use approx::assert_relative_eq;
//! // default unsigned mapping `[0..1]`
//! assert_eq![Unorm32(u32::MIN), Unorm32::from_f32(0.0)];
//! assert_eq![Unorm32(u32::MAX / 2 + 1), Unorm32::from(0.5)];
//! assert_eq![Unorm32(u32::MAX), Unorm32::from(1.0)];
//! // and back
//! assert_relative_eq![0.0, Unorm32(u32::MIN).to_f32()];
//! assert_relative_eq![0.5, Unorm32(u32::MAX / 2).into(), max_relative = 1e-9];
//! assert_relative_eq![1.0, Unorm32(u32::MAX).into()];
//!
//! // default signed mapping `[-1..1]`
//! assert_eq![Inorm16(i16::MIN), Inorm16::from_f32(-1.0)];
//! assert_eq![Inorm16(0), Inorm16::from(0.0)];
//! assert_eq![Inorm16(i16::MAX), Inorm16::from(1.0)];
//! // and back
//! assert_relative_eq![-1.0, Inorm16(i16::MIN).to_f32()];
//! assert_relative_eq![0.0, Inorm16(0).into(), max_relative = 1e-9];
//! assert_relative_eq![1.0, Inorm16(i16::MAX).into()];
//! ```
//!
//! ## Using custom ranges
//!
//! ```
//! # use iunorm::*;
//! # use approx::assert_relative_eq;
//! // to unsigned from a `180..180`
//! assert_eq![Unorm8(u8::MIN), Unorm8::from_f32_minmax(-180.0, -180.0, 180.0)];
//! assert_eq![Unorm8(u8::MAX / 2), Unorm8::from_f32_minmax(0.0, -180.0, 180.0)];
//! assert_eq![Unorm8(u8::MAX), Unorm8::from_f32_minmax(180.0, -180.0, 180.0)];
//! // and back
//! assert_eq![-180.0, Unorm16(u16::MIN).to_f32_minmax(-180.0, 180.0)];
//! assert_eq![-0.002746582, Unorm16(u16::MAX / 2).to_f32_minmax(-180.0, 180.0)];
//! assert_eq![180.0, Unorm16(u16::MAX).to_f32_minmax(-180.0, 180.0)];
//!
//! // to signed from a `0..360`
//! assert_eq![Inorm32(i32::MIN), Inorm32::from_f32_minmax(0.0, 0.0, 360.0)];
//! assert_eq![Inorm32(0), Inorm32::from_f32_minmax(180.0, 0.0, 360.0)];
//! assert_eq![Inorm32(i32::MAX), Inorm32::from_f32_minmax(360.0, 0.0, 360.0)];
//! // and back
//! assert_eq![0.0, Unorm64(u64::MIN).to_f32_minmax(0.0, 360.0)];
//! assert_eq![180.0, Unorm64(u64::MAX / 2).to_f32_minmax(0.0, 360.0)];
//! assert_eq![360.0, Unorm64(u64::MAX).to_f32_minmax(0.0, 360.0)];
//! ```

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
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod test;

mod generate;
pub use generate::*;

mod fns;
pub use fns::*;
