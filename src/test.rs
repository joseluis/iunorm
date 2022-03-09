// iunorm::tests

use crate::*;

#[test]
fn unsigned_conversions() {
    // basic 8-bit
    assert_eq![0u8, Unorm8::from_f32(0.0).into()];
    assert_eq![255u8, Unorm8::from_f32(1.0).into()];

    // no <over|under>flow
    assert_eq![255u8, Unorm8::from_f32(999.0).into()];
    assert_eq![0u8, Unorm8::from_f32(-999.0).into()];

    // all sizes
    assert_eq![u16::MAX, Unorm16::from_f32(1.0).into()];
    assert_eq![u32::MAX, Unorm32::from_f32(1.0).into()];
    assert_eq![u64::MAX, Unorm64::from_f32(1.0).into()];
    assert_eq![u128::MAX, Unorm128::from_f32(1.0).into()];
    assert_eq![0u128, Unorm128::from_f32(0.0).into()];
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
