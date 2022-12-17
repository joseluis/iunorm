// iunorm::fns
//
//!
//

/// Scales an `f32` `v`alue in between `[min..max]` to a new range `[a..b]`.
///
/// $$
/// v' = (b - a) \frac{v - min}{max - min} + a
/// $$
///
/// # Examples
/// ```
/// # use iunorm::scale32;
/// assert_eq![0.25, scale32(90., 0., 360., 0., 1.)];
/// assert_eq![-0.5, scale32(90., 0., 360., -1., 1.)];
///
/// assert_eq![90., scale32(0.25, 0., 1., 0., 360.)];
/// assert_eq![90., scale32(-0.5, -1., 1., 0., 360.)];
/// ```
#[inline]
#[must_use]
pub fn scale32(v: f32, min: f32, max: f32, a: f32, b: f32) -> f32 {
    (b - a) * (v - min) / (max - min) + a
}

/// Scales an `f64` `v`alue in between `[min..max]` to a new range `[a..b]`.
///
/// $$
/// v' = (b - a) \frac{v - min}{max - min} + a
/// $$
///
/// # Examples
/// ```
/// # use iunorm::scale64;
/// assert_eq![0.125, scale64(45., 0., 360., 0., 1.)];
/// assert_eq![-0.75, scale64(45., 0., 360., -1., 1.)];
///
/// assert_eq![45., scale64(0.125, 0., 1., 0., 360.)];
/// assert_eq![45., scale64(-0.75, -1., 1., 0., 360.)];
/// ```
#[inline]
#[must_use]
pub fn scale64(v: f64, min: f64, max: f64, a: f64, b: f64) -> f64 {
    (b - a) * (v - min) / (max - min) + a
}
