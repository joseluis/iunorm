// iunorm::generate

macro_rules! norm_sizes {
    (sizes: $( [$size:literal, $utype:ty, $itype:ty] ),+ ) => {
        $( norm_sizes!{@one: $size, $utype, $itype} )+
    };
    (@one: $size:literal, $utype:ty, $itype:ty) => {
        paste::paste!{
            #[doc = "An unsigned normalized number, between 0 and 1"]
            #[doc = "mapped to the range of a [`" $utype "`]."]
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct [<Unorm $size>](pub $utype);

            impl [<Unorm $size>] {
                #[doc = "New `" [<Unorm $size>] "` from an `f32`" ]
                #[inline]
                #[must_use]
                pub fn from_f32(f: f32) -> Self {
                    const FORMULA: f32 = 1. / (($utype::MAX) as f32 + 1.);
                    Self((f / FORMULA) as _)
                }
                #[doc = "New `" [<Unorm $size>] "` from an `f64`" ]
                #[inline]
                #[must_use]
                pub fn from_f64(f: f64) -> Self {
                    const FORMULA: f64 = 1. / (($utype::MAX) as f64 + 1.);
                    Self((f / FORMULA) as _)
                }
            }

            // from floats
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

            // Signed

            #[doc = "A signed normalized number, between -1 and 1,"]
            #[doc = "mapped to the range of an [`" $itype "`]."]
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct [<Inorm $size>](pub $itype);

            impl [<Inorm $size>] {
                #[doc = "New `" [<Inorm $size>] "` from an `f32`" ]
                #[inline]
                #[must_use]
                pub fn from_f32(f: f32) -> Self {
                    const FORMULA: f32 = 1. / (($itype::MAX) as f32 + 1.);
                    Self((f / FORMULA) as _)
                }
                #[doc = "New `" [<Inorm $size>] "` from an `f64`" ]
                #[inline]
                #[must_use]
                pub fn from_f64(f: f64) -> Self {
                    const FORMULA: f64 = 1. / (($itype::MAX) as f64 + 1.);
                    Self((f / FORMULA) as _)
                }
            }

            // from floats
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
