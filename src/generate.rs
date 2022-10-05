// iunorm::generate
//
//!
//

use crate::{scale32, scale64};
#[cfg(feature = "std")]
use std::fmt;

macro_rules! norm_sizes {
    (sizes: $( [$size:literal, $utype:ty, $itype:ty] ),+ ) => {
        $( norm_sizes!{@one: $size, $utype, $itype} )+
    };
    (@one: $size:literal, $utype:ty, $itype:ty) => {
        paste::paste!{
            /* Unsigned */

            #[doc = "A normalized number mapped to the range of a [`" $utype "`]."]
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct [<Unorm $size>](pub $utype);

            impl [<Unorm $size>] {
                #[doc = "Returns a `" [<Unorm $size>] "` from an `f32`" ]
                /// normalized to `0..1`.
                #[inline]
                #[must_use]
                pub fn from_f32(f: f32) -> Self {
                    Self((f * (($utype::MAX) as f32 + 1.)) as _)
                    // Self(scale32(f, 0., 1., $utype::MIN as f32, $utype::MAX as f32 + 1.) as _)
                }
                /// Converts the current value to an `f32` normalized to `0..1`.
                #[inline]
                #[must_use]
                pub fn to_f32(&self) -> f32 {
                    self.0 as f32 / $utype::MAX as f32
                }

                #[doc = "Returns a `" [<Unorm $size>] "` from an `f32`" ]
                /// normalized to `min..max`.
                #[inline]
                #[must_use]
                pub fn from_f32_minmax(f: f32, min: f32, max: f32) -> Self {
                    debug_assert![min < max];
                    Self(scale32(f, min, max, $utype::MIN as f32, $utype::MAX as f32) as _)
                }
                /// Converts the current value to an `f32` normalized to `min..max`.
                #[inline]
                #[must_use]
                pub fn to_f32_minmax(&self, min: f32, max: f32) -> f32 {
                    debug_assert![min < max];
                    scale32(self.0 as f32, $utype::MIN as f32, $utype::MAX as f32, min, max)
                }

                #[doc = "Returns a `" [<Unorm $size>] "` from an `f64`" ]
                /// normalized to `0..1`.
                #[inline]
                #[must_use]
                pub fn from_f64(f: f64) -> Self {
                    Self((f * (($utype::MAX) as f64 + 1.)) as _)
                    // Self(scale64(f, 0., 1., $utype::MIN as f64, $utype::MAX as f64 + 1.) as _)
                }
                /// Converts the current value to an `f64` normalized to `0..1`.
                #[inline]
                #[must_use]
                pub fn to_f64(&self) -> f64 {
                    self.0 as f64 / $utype::MAX as f64
                }

                #[doc = "Returns a `" [<Unorm $size>] "` from an `f64`" ]
                /// normalized to `min..max`.
                #[inline]
                #[must_use]
                pub fn from_f64_minmax(f: f64, min: f64, max: f64) -> Self {
                    debug_assert![min < max];
                    Self(scale64(f, min, max, $utype::MIN as f64, $utype::MAX as f64) as _)
                }
                /// Converts the current value to an `f64` normalized to `min..max`.
                #[inline]
                #[must_use]
                pub fn to_f64_minmax(&self, min: f64, max: f64) -> f64 {
                    debug_assert![min < max];
                    scale64(self.0 as f64, $utype::MIN as f64, $utype::MAX as f64, min, max)
                }
            }

            #[cfg(feature="std")]
            impl fmt::Display for [<Unorm $size>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.fmt(f)
                }
            }

            // from/into floats (Unorm normalizes by default to 0..1)
            impl From<f32> for [<Unorm $size>] {
                #[inline]
                fn from(f: f32) -> Self {
                    Self::from_f32(f)
                }
            }
            impl From<f64> for [<Unorm $size>] {
                #[inline]
                fn from(f: f64) -> Self {
                    Self::from_f64(f)
                }
            }
            impl From<[<Unorm $size>]> for f32 {
                #[inline]
                fn from(f: [<Unorm $size>]) -> f32 {
                    f.to_f32()
                }
            }
            impl From<[<Unorm $size>]> for f64 {
                #[inline]
                fn from(f: [<Unorm $size>]) -> f64 {
                    f.to_f64()
                }
            }

            // from/into inner type
            impl From<$utype> for [<Unorm $size>] {
                #[inline]
                fn from(f: $utype) -> Self {
                    Self(f)
                }
            }
            impl From<[<Unorm $size>]> for $utype {
                #[inline]
                fn from(unorm: [<Unorm $size>]) -> Self {
                    unorm.0
                }
            }

            /* Signed */

            #[doc = "A normalized number mapped to the range of an [`" $itype "`]."]
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct [<Inorm $size>](pub $itype);

            impl [<Inorm $size>] {
                #[doc = "Returns an `" [<Inorm $size>] "` from an `f32`" ]
                /// normalized to `-1..1`.
                #[inline]
                #[must_use]
                pub fn from_f32(f: f32) -> Self {
                    Self((f * (($itype::MAX) as f32 + 1.)) as _)
                    // Self(scale32(f, 0., 1., $itype::MIN as f32, $itype::MAX as f32 + 1.) as _)
                }
                /// Converts the current value to an `f32` normalized to `-1..1`.
                #[inline]
                #[must_use]
                pub fn to_f32(&self) -> f32 {
                    match self.0 {
                        $itype::MIN => -1.,
                        _ => self.0 as f32 / $itype::MAX as f32,
                    }
                }

                #[doc = "Returns an `" [<Inorm $size>] "` from an `f32`" ]
                /// normalized to `min..max`.
                #[inline]
                #[must_use]
                pub fn from_f32_minmax(f: f32, min: f32, max: f32) -> Self {
                    debug_assert![min < max];
                    Self(scale32(f, min, max, $itype::MIN as f32, $itype::MAX as f32) as _)
                }
                #[doc = "Converts the current value to an `f32` normalized to `min..max`." ]
                #[inline]
                #[must_use]
                pub fn to_f32_minmax(&self, min: f32, max: f32) -> f32 {
                    debug_assert![min < max];
                    scale32(self.0 as f32, $itype::MIN as f32, $itype::MAX as f32, min, max)
                }

                #[doc = "Returns an `" [<Inorm $size>] "` from an `f64`" ]
                /// normalized to `-1..1`.
                #[inline]
                #[must_use]
                pub fn from_f64(f: f64) -> Self {
                    Self((f * (($itype::MAX) as f64 + 1.)) as _)
                }
                #[doc = "Converts the current value to an `f64` normalized to `-1..1`." ]
                #[inline]
                #[must_use]
                pub fn to_f64(&self) -> f64 {
                    match self.0 {
                        $itype::MIN => -1.,
                        _ => self.0 as f64 / $itype::MAX as f64,
                    }
                }

                #[doc = "Returns an `" [<Inorm $size>] "` from an `f64`" ]
                /// normalized to `min..max`.
                #[inline]
                #[must_use]
                pub fn from_f64_minmax(f: f64, min: f64, max: f64) -> Self {
                    debug_assert![min < max];
                    Self(scale64(f, min, max, $itype::MIN as f64, $itype::MAX as f64) as _)
                    // Self(scale64(f, 0., 1., $itype::MIN as f64, $itype::MAX as f64 + 1.) as _)
                }
                /// Converts the current value to an `f64` normalized to `min..max`.
                #[inline]
                #[must_use]
                pub fn to_f64_minmax(&self, min: f64, max: f64) -> f64 {
                    debug_assert![min < max];
                    scale64(self.0 as f64, $itype::MIN as f64, $itype::MAX as f64, min, max)
                }
            }

            #[cfg(feature="std")]
            impl fmt::Display for [<Inorm $size>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.fmt(f)
                }
            }

            // from/into floats (Inorm normalizes by default to -1..1)
            impl From<f32> for [<Inorm $size>] {
                #[inline]
                fn from(f: f32) -> Self {
                    Self::from_f32(f)
                }
            }
            impl From<f64> for [<Inorm $size>] {
                #[inline]
                fn from(f: f64) -> Self {
                    Self::from_f64(f)
                }
            }
            impl From<[<Inorm $size>]> for f32 {
                #[inline]
                fn from(f: [<Inorm $size>]) -> f32 {
                    f.to_f32()
                }
            }
            impl From<[<Inorm $size>]> for f64 {
                #[inline]
                fn from(f: [<Inorm $size>]) -> f64 {
                    f.to_f64()
                }
            }

            // from/into inner type
            impl From<$itype> for [<Inorm $size>] {
                #[inline]
                fn from(f: $itype) -> Self {
                    Self(f)
                }
            }
            impl From<[<Inorm $size>]> for $itype {
                #[inline]
                fn from(inorm: [<Inorm $size>]) -> Self {
                    inorm.0
                }
            }
        }
    }
}

norm_sizes! {
    sizes: [8, u8, i8], [16, u16, i16], [32, u32, i32], [64, u64, i64], [128, u128, i128]
}
