// iunorm::tests
//
//!
//

use crate::*;
use approx::assert_relative_eq;

#[test]
fn unsigned() {
    // basic 8-bit
    assert_eq![0u8, Unorm8::from_f32(0.0).into()];
    assert_eq![255u8, Unorm8::from_f32(1.0).into()];

    assert_eq![0., Unorm8::from_f32(0.0).into()];
    assert_eq![0., Unorm8(u8::MIN).into()];
    assert_eq![1., Unorm8::from_f32(1.0).into()];
    assert_eq![1., Unorm8(u8::MAX).into()];

    // no <over|under>flow
    assert_eq![255u8, Unorm8::from_f32(999.0).into()];
    assert_eq![0u8, Unorm8::from_f32(-999.0).into()];

    assert_eq![0., Unorm8::from_f32(-999.0).into()];
    assert_eq![1., Unorm8::from_f32(999.0).into()];

    // all sizes
    assert_eq![u16::MAX, Unorm16::from_f32(1.0).into()];
    assert_eq![u32::MAX, Unorm32::from_f32(1.0).into()];
    assert_eq![u64::MAX, Unorm64::from_f32(1.0).into()];
    assert_eq![u128::MAX, Unorm128::from_f32(1.0).into()];
    assert_eq![0u128, Unorm128::from_f32(0.0).into()];

    assert_eq![1., Unorm16::from_f32(1.0).into()];
    assert_eq![1., Unorm32::from_f32(1.0).into()];
    assert_eq![1., Unorm64::from_f32(1.0).into()];
    assert_eq![1., Unorm128::from_f32(1.0).into()];
    assert_eq![0., Unorm128::from_f32(0.0).into()];
}

#[test]
fn signed() {
    // basic 8-bit
    assert_eq![i8::MIN, Inorm8::from_f32(-1.0).into()];
    assert_eq![0_i8, Inorm8::from_f32(0.0).into()];
    assert_eq![i8::MAX, Inorm8::from_f32(1.0).into()];

    assert_eq![0., Inorm8::from_f32(0.0).into()];
    assert_eq![0., Inorm8(0).into()];
    assert_eq![1., Inorm8::from_f32(1.0).into()];
    assert_eq![1., Inorm8(i8::MAX).into()];
    assert_eq![-1., Inorm8::from_f32(-1.0).into()];
    assert_eq![-1., Inorm8(i8::MIN).into()];

    // no <over|under>flow
    assert_eq![i8::MIN, Inorm8::from_f32(-999.0).into()];
    assert_eq![i8::MAX, Inorm8::from_f32(999.0).into()];

    assert_eq![1_f32, Inorm8::from_f32(999.0).into()];
    assert_eq![-1_f32, Inorm8::from_f32(-999.0).into()];

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

    assert_eq![1., Inorm16::from_f32(1.0).into()];
    assert_eq![1., Inorm32::from_f32(1.0).into()];
    assert_eq![1., Inorm64::from_f32(1.0).into()];
    assert_eq![1., Inorm128::from_f32(1.0).into()];
    assert_eq![0., Inorm128::from_f32(0.0).into()];
    assert_eq![-1., Inorm16::from_f32(-1.0).into()];
    assert_eq![-1., Inorm32::from_f32(-1.0).into()];
    assert_eq![-1., Inorm64::from_f32(-1.0).into()];
    assert_eq![-1., Inorm128::from_f32(-1.0).into()];
}

#[test]
fn unsigned_minmax() {
    // ranges that include negative numbers are also supported for Unorms:

    assert_eq![u8::MIN, Unorm8::from_f32_minmax(-20.0, -20.0, 20.0).into()];
    assert_eq![
        u8::MAX / 2,
        Unorm8::from_f32_minmax(0.0, -20.0, 20.0).into()
    ];
    assert_eq![u8::MAX, Unorm8::from_f32_minmax(20.0, -20.0, 20.0).into()];

    assert_eq![u8::MIN, Unorm8::from_f64_minmax(-20.0, -20.0, 20.0).into()];
    assert_eq![
        u8::MAX / 2,
        Unorm8::from_f64_minmax(0.0, -20.0, 20.0).into()
    ];
    assert_eq![u8::MAX, Unorm8::from_f64_minmax(20.0, -20.0, 20.0).into()];

    // and back
    assert_relative_eq![-20., Unorm8(u8::MIN).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![
        -0.078432,
        Unorm8(u8::MAX / 2).to_f32_minmax(-20.0, 20.0),
        max_relative = 1e-5
    ];
    assert_relative_eq![20., Unorm8(u8::MAX).to_f32_minmax(-20.0, 20.0)];
    // (notice the increment of precision for the midpoint)
    assert_relative_eq![-20., Unorm16(u16::MIN).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![-0.0003051, Unorm16(u16::MAX / 2).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![20., Unorm16(u16::MAX).to_f32_minmax(-20.0, 20.0)];
    //
    assert_relative_eq![-20., Unorm32(u32::MIN).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![0.0, Unorm32(u32::MAX / 2).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![20., Unorm32(u32::MAX).to_f32_minmax(-20.0, 20.0)];

    // f64
    assert_relative_eq![-20., Unorm8(u8::MIN).to_f64_minmax(-20.0, 20.0)];
    assert_relative_eq![
        -0.078431,
        Unorm8(u8::MAX / 2).to_f64_minmax(-20.0, 20.0),
        max_relative = 1e-5
    ];
    assert_relative_eq![20., Unorm8(u8::MAX).to_f64_minmax(-20.0, 20.0)];
}

#[test]
fn signed_minmax() {
    assert_eq![i8::MIN, Inorm8::from_f32_minmax(-20.0, -20.0, 20.0).into()];
    assert_eq![0_i8, Inorm8::from_f32_minmax(0.0, -20.0, 20.0).into()];
    assert_eq![i8::MAX, Inorm8::from_f32_minmax(20.0, -20.0, 20.0).into()];

    assert_eq![i8::MIN, Inorm8::from_f64_minmax(-20.0, -20.0, 20.0).into()];
    assert_eq![0_i8, Inorm8::from_f64_minmax(0.0, -20.0, 20.0).into()];
    assert_eq![i8::MAX, Inorm8::from_f64_minmax(20.0, -20.0, 20.0).into()];

    // and back
    assert_relative_eq![-20., Inorm8(i8::MIN).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![
        9.96,
        Inorm8(i8::MAX / 2).to_f32_minmax(-20.0, 20.0),
        max_relative = 1e-4
    ];
    assert_relative_eq![20., Inorm8(i8::MAX).to_f32_minmax(-20.0, 20.0)];
    // (notice the increment of precision for the midpoint)
    assert_relative_eq![-20., Inorm16(i16::MIN).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![
        9.999,
        Inorm16(i16::MAX / 2).to_f32_minmax(-20.0, 20.0),
        max_relative = 1e-4
    ];
    assert_relative_eq![20., Inorm16(i16::MAX).to_f32_minmax(-20.0, 20.0)];
    //
    assert_relative_eq![-20., Inorm32(i32::MIN).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![10.0, Inorm32(i32::MAX / 2).to_f32_minmax(-20.0, 20.0)];
    assert_relative_eq![20., Inorm32(i32::MAX).to_f32_minmax(-20.0, 20.0)];

    // f64
    assert_relative_eq![-20., Inorm8(i8::MIN).to_f64_minmax(-20.0, 20.0)];
    assert_relative_eq![
        9.96,
        Inorm8(i8::MAX / 2).to_f64_minmax(-20.0, 20.0),
        max_relative = 1e-4
    ];
    assert_relative_eq![20., Inorm8(i8::MAX).to_f64_minmax(-20.0, 20.0)];
}
