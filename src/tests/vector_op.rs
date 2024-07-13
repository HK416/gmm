use rand::Rng;
use crate::{Float2, Float3, Float4, Vector};

/// Some SIMD instructions do not conform to IEEE-754. (for performance benefits)
/// 
/// So we compare using a separate Epsilon constant.
/// 
const EPSILON: f32 = 1.192092896e-6;

/// Number of tests.
const NUM_TEST: usize = 1_000_000;



#[test]
fn vector_abs() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin.into(); t.into() };
        let vector_b = vector_a.abs();

        // Control group
        let glam_a: glam::Vec4 = origin.into();
        let glam_b = glam_a.abs();

        // Compare `Vector` and `Control group`
        let a: [f32; 4] = { let t: Float4 = vector_b.into(); t.into() };
        let b: [f32; 4] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Absolute operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, vector_b, glam_b);
    }
}

#[test]
fn vector_sum() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin.into(); t.into() };
        let a = { let t: Float4 = vector_a.sum().into(); t.x };

        // Control group
        let b: f32 = origin.iter().sum();
        

        // Compare `Vector` and `Control group`
        let validate = (a - b).abs() <= EPSILON;
        assert!(validate, "Test:{} >> Sum operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_dot() {
    // `Float*` data type does not support this operation.
}
