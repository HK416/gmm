use rand::Rng;
use crate::{Float2, Float3, Float4, Vector};
use super::{NUM_TEST, EPSILON};



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
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: [f32; 2] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float2 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float2 = origin_b.into(); t.into() };
        let vector_c = vector_a.vec2_dot(vector_b);

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b: glam::Vec2 = origin_b.into();
        let glam_c = glam_a.dot(glam_b);

        // Compare `Vector` and `Control group`
        let a: f32 = { let t: Float2 = vector_c.into(); t.x };
        let b: f32 = glam_c;
        assert_eq!(a, b, "Test:{} >> Dot2 operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_dot() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: [f32; 3] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float3 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float3 = origin_b.into(); t.into() };
        let vector_c = vector_a.vec3_dot(vector_b);

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b: glam::Vec3 = origin_b.into();
        let glam_c = glam_a.dot(glam_b);

        // Compare `Vector` and `Control group`
        let a: f32 = { let t: Float3 = vector_c.into(); t.x };
        let b: f32 = glam_c;
        assert_eq!(a, b, "Test:{} >> Dot3 operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_dot() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float4 = origin_b.into(); t.into() };
        let vector_c = vector_a.vec4_dot(vector_b);

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a.dot(glam_b);

        // Compare `Vector` and `Control group`
        let a: f32 = { let t: Float4 = vector_c.into(); t.x };
        let b: f32 = glam_c;
        assert!((a - b).abs() <= EPSILON, "Test:{} >> Dot4 operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_cross() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: [f32; 3] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float3 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float3 = origin_b.into(); t.into() };
        let vector_c = vector_a.vec3_cross(vector_b);

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b: glam::Vec3 = origin_b.into();
        let glam_c = glam_a.cross(glam_b);

        // Compare `Vector` and `Control group`
        let a: [f32; 3] = { let t: Float3 = vector_c.into(); t.into() };
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Cross operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}
