use core::f32;
use crate::prelude::*;



#[test]
fn vector_load_store_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    let a = Float4::new(X, Y, Z, W);
    let v = load_float4(a);
    let b = store_float4(v);

    assert!((a.x - b.x).abs() <= f32::EPSILON, "invalid load/store operation!");
    assert!((a.y - b.y).abs() <= f32::EPSILON, "invalid load/store operation!");
    assert!((a.z - b.z).abs() <= f32::EPSILON, "invalid load/store operation!");
    assert!((a.w - b.w).abs() <= f32::EPSILON, "invalid load/store operation!");
}

#[test]
fn vectoru32_load_store_test() {
    const X: u32 = 10;
    const Y: u32 = 2000;
    const Z: u32 = 300000;
    const W: u32 = 40000000;

    let a = UInteger4::new(X, Y, Z, W);
    let v = load_uinteger4(a);
    let b = store_uinteger4(v);
    let c = store_boolean4(v);

    assert!(!c.any(), "invalid load/store operation!");
    assert_eq!(a.x, b.x, "invalid load/store operation!");
    assert_eq!(a.y, b.y, "invalid load/store operation!");
    assert_eq!(a.z, b.z, "invalid load/store operation!");
    assert_eq!(a.w, b.w, "invalid load/store operation!");
}

#[test]
fn vector_add_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    const VAL: f32 = 3.141592;

    let a = Float4::new(X, Y, Z, W);
    let b = Float4::fill(VAL);
    let c = Float4::new(X + VAL, Y + VAL, Z + VAL, W + VAL);

    let v_a = load_float4(a);
    let v_b = load_float4(b);
    let v_c = load_float4(c);
    let v_res = vector_add(v_a, v_b);
    
    assert!(vector4_eq(v_res, v_c), "invalid add operation!");
}

#[test]
fn vector_sub_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    const VAL: f32 = 3.141592;

    let a = Float4::new(X, Y, Z, W);
    let b = Float4::fill(VAL);
    let c = Float4::new(X - VAL, Y - VAL, Z - VAL, W - VAL);

    let v_a = load_float4(a);
    let v_b = load_float4(b);
    let v_c = load_float4(c);
    let v_res = vector_sub(v_a, v_b);

    assert!(vector4_eq(v_res, v_c), "invalid sub operation!");
}

#[test]
fn vector_mul_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    const VAL: f32 = 3.141592;
    
    let a = Float4::new(X, Y, Z, W);
    let b = Float4::fill(VAL);
    let c = Float4::new(X * VAL, Y * VAL, Z * VAL, W * VAL);

    let v_a = load_float4(a);
    let v_b = load_float4(b);
    let v_c = load_float4(c);
    let v_res = vector_mul(v_a, v_b);

    assert!(vector4_eq(v_res, v_c), "invalid mul operation!");
}

#[test]
fn vector_div_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    const VAL: f32 = 3.141592;
    
    let a = Float4::new(X, Y, Z, W);
    let b = Float4::fill(VAL);
    let c = Float4::new(X / VAL, Y / VAL, Z / VAL, W / VAL);

    let v_a = load_float4(a);
    let v_b = load_float4(b);
    let v_c = load_float4(c);
    let v_res = vector_div(v_a, v_b);

    assert!(vector4_eq(v_res, v_c), "invalid div operation!");
}

#[test]
fn vector_neg_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    let a = Float4::new(X, Y, Z, W);
    let b = Float4::new(-X, -Y, -Z, -W);

    let v_a = load_float4(a);
    let v_b = load_float4(b);
    let v_res = vector_neg(v_a);

    assert!(vector4_eq(v_res, v_b), "invalid negative operation!");
}

#[test]
fn vector_min_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    const VAL: f32 = 3.141592;

    let a = Float4::new(X, Y, Z, W);
    let b = Float4::fill(VAL);
    let c = Float4::new(X, Y, VAL, VAL);

    let v_a = load_float4(a);
    let v_b = load_float4(b);
    let v_c = load_float4(c);
    let v_res = vector_min(v_a, v_b);

    assert!(vector4_eq(v_res, v_c), "invalid minimum operation!");
}

#[test]
fn vector_max_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    const VAL: f32 = 3.141592;

    let a = Float4::new(X, Y, Z, W);
    let b = Float4::fill(VAL);
    let c = Float4::new(VAL, VAL, Z, W);

    let v_a = load_float4(a);
    let v_b = load_float4(b);
    let v_c = load_float4(c);
    let v_res = vector_max(v_a, v_b);

    assert!(vector4_eq(v_res, v_c), "invalid maximum operation!");
}

#[test]
fn vector2_dot_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    let a = Float4::new(X, Y, Z, W);
    let c = X * X + Y * Y;
    
    let v_a = load_float4(a);
    let res = vector2_dot(v_a, v_a);

    assert!((res - c).abs() <= f32::EPSILON, "invalid vector2 dot operation! (res:{}, c:{})", res, c);
}

#[test]
fn vector3_dot_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    let a = Float4::new(X, Y, Z, W);
    let c = X * X + Y * Y + Z * Z;
    
    let v_a = load_float4(a);
    let res = vector3_dot(v_a, v_a);

    assert!((res - c).abs() <= f32::EPSILON, "invalid vector3 dot operation! (res:{}, c:{})", res, c);
}

#[test]
fn vector4_dot_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    let a = Float4::new(X, Y, Z, W);
    let c = X * X + Y * Y + Z * Z + W * W;
    
    let v_a = load_float4(a);
    let res = vector4_dot(v_a, v_a);
    
    assert!((res - c).abs() <= f32::EPSILON, "invalid vector4 dot operation! (res:{}, c:{})", res, c);
}

#[test]
fn vector3_cross_test_0() {
    const AX: f32 = 1.0;
    const AY: f32 = 0.0;
    const AZ: f32 = 0.0;

    const BX: f32 = 0.0;
    const BY: f32 = 1.0;
    const BZ: f32 = 0.0;

    const CX: f32 = 0.0;
    const CY: f32 = 0.0;
    const CZ: f32 = 1.0;

    let a = Float3::new(AX, AY, AZ);
    let b = Float3::new(BX, BY, BZ);
    let c = Float3::new(CX, CY, CZ);

    let v_a = load_float3(a);
    let v_b = load_float3(b);
    let v_c = load_float3(c);
    let v_res = vector3_cross(v_a, v_b);

    assert!(vector3_eq(v_res, v_c), "invalid vector3 cross operation! (v_res:{:?}, v_c:{:?})", v_res, v_c);
}

#[test]
fn vector3_cross_test_1() {
    const AX: f32 = 1.0;
    const AY: f32 = 2.0;
    const AZ: f32 = -2.0;

    const BX: f32 = 3.0;
    const BY: f32 = 0.0;
    const BZ: f32 = 1.0;

    const CX: f32 = 2.0;
    const CY: f32 = -7.0;
    const CZ: f32 = -6.0;

    let a = Float3::new(AX, AY, AZ);
    let b = Float3::new(BX, BY, BZ);
    let c = Float3::new(CX, CY, CZ);

    let v_a = load_float3(a);
    let v_b = load_float3(b);
    let v_c = load_float3(c);
    let v_res = vector3_cross(v_a, v_b);

    assert!(vector3_eq(v_res, v_c), "invalid vector3 cross operation! (v_res:{:?}, v_c:{:?})", v_res, v_c);
}

#[test]
fn vector3_cross_test_2() {
    const AX: f32 = -1.0;
    const AY: f32 = 1.0;
    const AZ: f32 = 0.0;

    const BX: f32 = -1.0;
    const BY: f32 = 0.0;
    const BZ: f32 = 1.0;

    const CX: f32 = 1.0;
    const CY: f32 = 1.0;
    const CZ: f32 = 1.0;

    let a = Float3::new(AX, AY, AZ);
    let b = Float3::new(BX, BY, BZ);
    let c = Float3::new(CX, CY, CZ);

    let v_a = load_float3(a);
    let v_b = load_float3(b);
    let v_c = load_float3(c);
    let v_res = vector3_cross(v_a, v_b);

    assert!(vector3_eq(v_res, v_c), "invalid vector3 cross operation! (v_res:{:?}, v_c:{:?})", v_res, v_c);
}
