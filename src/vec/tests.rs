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
fn matrix_load_store_test() {
    const ELEMENTS: [f32; 16] = [
        103.75782485548567, 
        73.89488159306876, 
        15.930446704662302, 
        -12.434618265668846, 
        -77.32049114915367, 
        46.031410186407896, 
        69.89353173722401, 
        -39.16914668039783, 
        68.14968983215715, 
        -19.71285306631725, 
        -120.46870508215576, 
        -34.357817359740366, 
        -94.05212950646396, 
        4.609474069712377, 
        -75.30518158135038, 
        99.87223603758153
    ];

    let a = Float4x4::from_array(&ELEMENTS);
    let m = load_float4x4(a);
    let b = store_float4x4(m);

    let a_arr = a.as_ref();
    let b_arr = b.as_ref();
    for idx in 0..16 {
        assert!((a_arr[idx] - b_arr[idx]).abs() <= f32::EPSILON, "invalid load/store operation!");
    }
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

#[test]
fn quaternion_test_0() {
    const AX: f32 = 0.08;
    const AY: f32 = 0.17;
    const AZ: f32 = -0.01;
    const AW: f32 = 0.98;

    const BX: f32 = 0.70;
    const BY: f32 = 0.00;
    const BZ: f32 = 0.00;
    const BW: f32 = 0.70;

    const CX: f32 = 0.742;
    const CY: f32 = 0.112;
    const CZ: f32 = -0.126;
    const CW: f32 = 0.63;

    let qa = Float4::new(AX, AY, AZ, AW);
    let qb = Float4::new(BX, BY, BZ, BW);
    let qc = Float4::new(CX, CY, CZ, CW);

    let v_qa = load_float4(qa);
    let v_qb = load_float4(qb);
    let v_qc = load_float4(qc);
    let v_res = quaternion_mul(v_qa, v_qb);

    assert!(vector4_eq(v_res, v_qc), "invalid quaternion mul operation! (v_res:{:?}, v_qc:{:?})", v_res, v_qc);
}

#[test]
fn matrix_transpose_test() {
    const ARR: [f32; 16] = [
        -66.46781402277918, 36.161587743841665, -87.87818063183681, 1.051366701884291, 
        47.15419759697318, 12.549041556657329, 44.88190040303925, -24.39321559332491, 
        112.4745861432626, 83.7932943750977, 119.9817159737807, -56.35303058139162, 
        -8.26872013734797, -20.440493810186638, -118.0236429363606, -104.48928792510765
    ];

    let a = Float4x4::from_array(&ARR);
    let m_a = load_float4x4(a);
    let m_ta = matrix_transpose(m_a);
    let res = store_float4x4(m_ta);

    let arr_a = a.as_ref();
    let arr_res = res.as_ref();
    for row in 0..4 {
        for col in 0..4 {
            assert!((arr_a[row * 4 + col] - arr_res[col * 4 + row]).abs() <= f32::EPSILON, "invalid matrix transpose operation!");
        }
    }
}

