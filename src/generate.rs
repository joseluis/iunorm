// iunorm::generate

macro_rules! norm_sizes {
    (sizes: $( [$size:literal, $utype:ty, $itype:ty] ),+ ) => {
        $( norm_sizes!{@one: $size, $utype, $itype} )+
    };
    (@one: $size:literal, $utype:ty, $itype:ty) => {
        paste::paste!{
            #[doc = "An unsigned normalized number, between 0 and 1"]
            #[doc = "mapped to the range of a " $utype "."]
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct [<Unorm $size>]($utype);

            impl [<Unorm $size>] {
                #[doc = "New " [<Unorm $size>] "from an `f32`" ]
                #[inline]
                fn from_f32(f: f32) -> Self {
                    const FORMULA: f32 = 1. / (($utype::MAX) as f32 + 1.);
                    Self((f / FORMULA) as _)
                }
                #[doc = "New " [<Unorm $size>] "from an `f64`" ]
                #[inline]
                fn from_f64(f: f64) -> Self {
                    const FORMULA: f64 = 1. / (($utype::MAX) as f64 + 1.);
                    Self((f / FORMULA) as _)
                }
            }

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
            #[doc = "mapped to the range of an " $itype "."]
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct [<Inorm $size>]($itype);

            impl [<Inorm $size>] {
                #[doc = "New " [<Inorm $size>] "from an `f32`" ]
                #[inline]
                fn from_f32(f: f32) -> Self {
                    const FORMULA: f32 = 1. / (($itype::MAX) as f32 + 1.);
                    Self((f / FORMULA) as _)
                }
                #[doc = "New " [<inorm $size>] "from an `f64`" ]
                #[inline]
                fn from_f64(f: f64) -> Self {
                    const FORMULA: f64 = 1. / (($itype::MAX) as f64 + 1.);
                    Self((f / FORMULA) as _)
                }
            }


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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_conversions() {
        // basic 8-bit
        assert_eq![0_u8, Unorm8::from_f32(0.0).into()];
        assert_eq![255_u8, Unorm8::from_f32(1.0).into()];

        // no <over|under>flow
        assert_eq![255_u8, Unorm8::from_f32(999.0).into()];
        assert_eq![0_u8, Unorm8::from_f32(-999.0).into()];

        // all sizes
        assert_eq![u16::MAX, Unorm16::from_f32(1.0).into()];
        assert_eq![u32::MAX, Unorm32::from_f32(1.0).into()];
        assert_eq![u64::MAX, Unorm64::from_f32(1.0).into()];
        assert_eq![u128::MAX, Unorm128::from_f32(1.0).into()];
        assert_eq![0_u128, Unorm128::from_f32(0.0).into()];
    }

    #[test]
    fn signed_conversions() {
        // basic 8-bit
        assert_eq![0_i8, Inorm8::from_f32(0.0).into()];
        assert_eq![127_i8, Inorm8::from_f32(1.0).into()];
        assert_eq![-128_i8, Inorm8::from_f32(-1.0).into()];

        // no <over|under>flow
        assert_eq![127_i8, Inorm8::from_f32(999.0).into()];
        assert_eq![-128_i8, Inorm8::from_f32(-999.0).into()];

        // all sizes
        assert_eq![i16::MAX, Inorm16::from_f32(1.0).into()];
        assert_eq![i32::MAX, Inorm32::from_f32(1.0).into()];
        assert_eq![i64::MAX, Inorm64::from_f32(1.0).into()];
        assert_eq![i128::MAX, Inorm128::from_f32(1.0).into()];
        assert_eq![0_i128, Inorm128::from_f32(0.0).into()];
        assert_eq![i16::MIN, Inorm16::from_f32(-1.0).into()];
        assert_eq![i32::MIN, Inorm32::from_f32(-1.0).into()];
        assert_eq![i64::MIN, Inorm64::from_f32(-1.0).into()];
        assert_eq![i128::MIN, Inorm128::from_f32(-1.0).into()];
    }
}
