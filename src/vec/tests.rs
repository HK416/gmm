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
    
    let v_diff = vector_sub(v_res, v_c);
    let length = vector4_length_sq(v_diff);

    assert!(length <= f32::EPSILON, "invalid add operation!");
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

    let v_diff = vector_sub(v_res, v_c);
    let length = vector4_length_sq(v_diff);

    assert!(length <= f32::EPSILON, "invalid sub operation!");
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

    let v_diff = vector_sub(v_res, v_c);
    let length = vector4_length_sq(v_diff);

    assert!(length <= f32::EPSILON, "invalid mul operation!");
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

    let v_diff = vector_sub(v_res, v_c);
    let length = vector4_length_sq(v_diff);

    assert!(length <= f32::EPSILON, "invalid div operation!");
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

    assert!((res - c).abs() <= f32::EPSILON, "invalid vector2 dot operation!");
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

    assert!((res - c).abs() <= f32::EPSILON, "invalid vector3 dot operation!");
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
