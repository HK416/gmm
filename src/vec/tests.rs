use core::f32;
use rand::Rng;

use crate::prelude::*;

/// Some SIMD instructions do not conform to IEEE-754. (for performance benefits)
/// 
/// So we compare using a separate Epsilon constant.
/// 
const EPSILON: f32 = 1.192092896e-6;

/// Verification number of tests
const NUM_TEST: usize = 1_000_000;


#[test]
fn vector_load_store_test() {
    const X: f32 = 1.1234567;
    const Y: f32 = 2.2345678;
    const Z: f32 = 3.3456789;
    const W: f32 = 4.4567890;

    let a = Float4::new(X, Y, Z, W);
    let v = load_float4(a);
    let b = store_float4(v);

    assert!((a.x - b.x).abs() <= EPSILON, "invalid load/store operation!");
    assert!((a.y - b.y).abs() <= EPSILON, "invalid load/store operation!");
    assert!((a.z - b.z).abs() <= EPSILON, "invalid load/store operation!");
    assert!((a.w - b.w).abs() <= EPSILON, "invalid load/store operation!");
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

    let a = Float4x4::from_column_array(&ELEMENTS);
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
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 4] = rng.gen();
        let b_arr: [f32; 4] = rng.gen();

        
        let a = Float4::from_array(&a_arr);
        let b = Float4::from_array(&b_arr);
        let v_a = load_float4(a);
        let v_b = load_float4(b);
        let v_res = vector_add(v_a, v_b);
        let res = store_float4(v_res);
    
        let g_a = glam::Vec4::from_array(a_arr);
        let g_b = glam::Vec4::from_array(b_arr);
        let g_res = g_a + g_b;
    
        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..4 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid add operation (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn vector_sub_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 4] = rng.gen();
        let b_arr: [f32; 4] = rng.gen();

        
        let a = Float4::from_array(&a_arr);
        let b = Float4::from_array(&b_arr);
        let v_a = load_float4(a);
        let v_b = load_float4(b);
        let v_res = vector_sub(v_a, v_b);
        let res = store_float4(v_res);
    
        let g_a = glam::Vec4::from_array(a_arr);
        let g_b = glam::Vec4::from_array(b_arr);
        let g_res = g_a - g_b;
    
        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..4 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid sub operation (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn vector_mul_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 4] = rng.gen();
        let b_arr: [f32; 4] = rng.gen();

        
        let a = Float4::from_array(&a_arr);
        let b = Float4::from_array(&b_arr);
        let v_a = load_float4(a);
        let v_b = load_float4(b);
        let v_res = vector_mul(v_a, v_b);
        let res = store_float4(v_res);
    
        let g_a = glam::Vec4::from_array(a_arr);
        let g_b = glam::Vec4::from_array(b_arr);
        let g_res = g_a * g_b;
    
        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..4 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid mul operation (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn vector_div_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 4] = rng.gen();
        let b_val: f32 = rng.gen_range(0.001..f32::MAX);

        
        let a = Float4::from_array(&a_arr);
        let b = Float4::fill(b_val);
        let v_a = load_float4(a);
        let v_b = load_float4(b);
        let v_res = vector_div(v_a, v_b);
        let res = store_float4(v_res);
    
        let g_a = glam::Vec4::from_array(a_arr);
        let g_b = glam::Vec4::splat(b_val);
        let g_res = g_a / g_b;
    
        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..4 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid add operation (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn vector_neg_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let arr: [f32; 4] = rng.gen();

        let a = Float4::from_array(&arr);
        let v_a = load_float4(a);
        let v_res = vector_neg(v_a);
        let res = store_float4(v_res);
    
        let g_a = glam::Vec4::from_array(arr);
        let g_res = -g_a;
    
        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..4 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid neg operation (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn vector_min_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 4] = rng.gen();
        let b_arr: [f32; 4] = rng.gen();
        
        let a = Float4::from_array(&a_arr);
        let b = Float4::from_array(&b_arr);
        let v_a = load_float4(a);
        let v_b = load_float4(b);
        let v_res = vector_min(v_a, v_b);
        let res = store_float4(v_res);
    
        let g_a = glam::Vec4::from_array(a_arr);
        let g_b = glam::Vec4::from_array(b_arr);
        let g_res = g_a.min(g_b);
    
        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..4 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid min operation (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn vector_max_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 4] = rng.gen();
        let b_arr: [f32; 4] = rng.gen();

        let a = Float4::from_array(&a_arr);
        let b = Float4::from_array(&b_arr);
        let v_a = load_float4(a);
        let v_b = load_float4(b);
        let v_res = vector_max(v_a, v_b);
        let res = store_float4(v_res);
    
        let g_a = glam::Vec4::from_array(a_arr);
        let g_b = glam::Vec4::from_array(b_arr);
        let g_res = g_a.max(g_b);
    
        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..4 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid max operation (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn vector_transform_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let matrix: [f32; 16] = rng.gen();
        let vector: [f32; 4] = rng.gen();

        let m = Float4x4::from_column_array(&matrix);
        let v = Float4::from_array(&vector);
        let m = load_float4x4(m);
        let v = load_float4(v);
        let res = vector_transform(m, v);
        let res = store_float4(res);

        let g_m = glam::Mat4::from_cols_array(&matrix);
        let g_v = glam::Vec4::from_array(vector);
        let g_res = g_m.mul_vec4(g_v);

        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..4 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid vector transform operation (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn vector2_dot_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 2] = rng.gen();
        let b_arr: [f32; 2] = rng.gen();
        
        let a = Float2::from_array(&a_arr);
        let b = Float2::from_array(&b_arr);
        let v_a = load_float2(a);
        let v_b = load_float2(b);
        let res = vector2_dot(v_a, v_b);
    
        let g_a = glam::Vec2::from_array(a_arr);
        let g_b = glam::Vec2::from_array(b_arr);
        let g_res = g_a.dot(g_b);
    
        let validate = (res - g_res).abs() <= EPSILON;
        assert!(validate, "invalid vector2 dot operation (test:{}, this:{}, glam:{})", test, res, g_res);
    }
}

#[test]
fn vector3_dot_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 3] = rng.gen();
        let b_arr: [f32; 3] = rng.gen();
        
        let a = Float3::from_array(&a_arr);
        let b = Float3::from_array(&b_arr);
        let v_a = load_float3(a);
        let v_b = load_float3(b);
        let res = vector3_dot(v_a, v_b);
    
        let g_a = glam::Vec3::from_array(a_arr);
        let g_b = glam::Vec3::from_array(b_arr);
        let g_res = g_a.dot(g_b);
    
        let validate = (res - g_res).abs() <= EPSILON;
        assert!(validate, "invalid vector3 dot operation (test:{}, this:{}, glam:{})", test, res, g_res);
    }
}

#[test]
fn vector4_dot_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 4] = rng.gen();
        let b_arr: [f32; 4] = rng.gen();
        
        let a = Float4::from_array(&a_arr);
        let b = Float4::from_array(&b_arr);
        let v_a = load_float4(a);
        let v_b = load_float4(b);
        let res = vector4_dot(v_a, v_b);
    
        let g_a = glam::Vec4::from_array(a_arr);
        let g_b = glam::Vec4::from_array(b_arr);
        let g_res = g_a.dot(g_b);
    
        let validate = (res - g_res).abs() <= EPSILON;
        assert!(validate, "invalid vector4 dot operation (test:{}, this:{}, glam:{})", test, res, g_res);
    }
}

#[test]
fn vector3_cross_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        loop {
            let a_arr: [f32; 3] = rng.gen();
            let b_arr: [f32; 3] = rng.gen();

            let a = Float3::from_array(&a_arr);
            let b = Float3::from_array(&b_arr);
            let v_a = load_float3(a);
            let v_a = vector3_normalize(v_a);
            let v_b = load_float3(b);
            let v_b = vector3_normalize(v_b);
            if v_a.is_none() | v_b.is_none() {
                continue;
            }

            let v_res = vector3_cross(v_a.unwrap(), v_b.unwrap());
            let res = store_float3(v_res);

            let g_a = glam::Vec3::from_array(a_arr).normalize();
            let g_b = glam::Vec3::from_array(b_arr).normalize();
            let g_res = g_a.cross(g_b);

            let raw_res = res.as_ref();
            let raw_g_res = g_res.as_ref();
            for idx in 0..3 {
                let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
                assert!(validate, "invalid vector3 cross operation (test:{}, this:{}, glam:{})", test, res, g_res);
            }
            break;
        }
    }
}

#[test]
fn quaternion_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        loop {
            let a_arr: [f32; 4] = rng.gen();
            let b_arr: [f32; 4] = rng.gen();
            
            let qa = Float4::from_array(&a_arr);
            let qb = Float4::from_array(&b_arr);
            let v_qa = load_float4(qa);
            let v_qb = load_float4(qb);
            let v_qa = vector4_normalize(v_qa);
            let v_qb = vector4_normalize(v_qb);
            if v_qa.is_none() | v_qb.is_none() {
                continue;
            }

            let v_qres = quaternion_mul(v_qa.unwrap(), v_qb.unwrap());
            let qres = store_float4(v_qres);

            let g_qa = glam::Quat::from_array(a_arr).normalize();
            let g_qb = glam::Quat::from_array(b_arr).normalize();
            let g_qres = g_qa.mul_quat(g_qb);

            let raw_qres = qres.as_ref();
            let raw_g_qres = g_qres.as_ref();
            for idx in 0..4 {
                let validate = (raw_qres[idx] - raw_g_qres[idx]).abs() <= EPSILON;
                assert!(validate, "invalid quaternion mul operation (test:{}, this:{}, glam:{})", test, qres, g_qres);
            }
            break;
        }
    }
}

#[test]
fn matrix_transpose_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let arr: [f32; 16] = rng.gen();
        let a = Float4x4::from_column_array(&arr);
        let m_a = load_float4x4(a);
        let m_ta = matrix_transpose(m_a);
        let res = store_float4x4(m_ta);

        let g_a = glam::Mat4::from_cols_array(&arr);
        let g_res = g_a.transpose();
        
        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..16 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid matrix transpose operation! (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn matrix_mul_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let a_arr: [f32; 16] = rng.gen();
        let b_arr: [f32; 16] = rng.gen();
        let a = Float4x4::from_column_array(&a_arr);
        let b = Float4x4::from_column_array(&b_arr);
        let m_a = load_float4x4(a);
        let m_b = load_float4x4(b);
        let m_res = matrix_mul(m_a, m_b);
        let res = store_float4x4(m_res);

        let g_a = glam::Mat4::from_cols_array(&a_arr);
        let g_b = glam::Mat4::from_cols_array(&b_arr);
        let g_res = g_a.mul_mat4(&g_b);

        let raw_res = res.as_ref();
        let raw_g_res = g_res.as_ref();
        for idx in 0..16 {
            let validate = (raw_res[idx] - raw_g_res[idx]).abs() <= EPSILON;
            assert!(validate, "invalid matrix mul operation! (test:{}, this:{}, glam:{})", test, res, g_res);
        }
    }
}

#[test]
fn matrix_determinant_test() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        let arr: [f32; 16] = rng.gen();
        let a = Float4x4::from_column_array(&arr);
        let m_a = load_float4x4(a);
        let det = matrix_determinant(m_a);

        let g_a = glam::Mat4::from_cols_array(&arr);
        let g_det = g_a.determinant();

        let validate = (det - g_det).abs() <= EPSILON;
        assert!(validate, "invalid matrix determinant operation! (test:{}, this:{}, glam:{})", test, det, g_det);
    }
}
