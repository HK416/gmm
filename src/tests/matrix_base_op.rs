use rand::Rng;
use crate::{Float3x3, Float4x4, Matrix};
use super::{NUM_TEST, EPSILON};



#[test]
fn matrix3x3_add() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 9] = rng.gen();
        let origin_b: [f32; 9] = rng.gen();

        // Float3x3
        let float_a: Float3x3 = origin_a.into();
        let float_b: Float3x3 = origin_b.into();
        let float_c = float_a + float_b;

        // Matrix
        let matrix_a: Matrix = float_a.into();
        let matrix_b: Matrix = float_b.into();
        let matrix_c = matrix_a + matrix_b;

        // Control group
        let glam_a = glam::Mat3::from_cols_array(&origin_a);
        let glam_b = glam::Mat3::from_cols_array(&origin_b);
        let glam_c = glam_a + glam_b;

        // Compare `Float3x3` and `Control group`
        let a: [f32; 9] = float_c.into();
        let b: [f32; 9] = glam_c.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Add operation on `Float3x3` is invalid! (Float3x3:{:?}, Control group:{:?})", test, a, b);

        // Compare `Matrix` and `Control group`
        let a: [f32; 9] = { let t: Float3x3 = matrix_c.into(); t.into() };
        let b: [f32; 9] = glam_c.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Add operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix4x4_add() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 16] = rng.gen();
        let origin_b: [f32; 16] = rng.gen();

        // Float4x4
        let float_a: Float4x4 = origin_a.into();
        let float_b: Float4x4 = origin_b.into();
        let float_c = float_a + float_b;

        // Matrix
        let matrix_a: Matrix = float_a.into();
        let matrix_b: Matrix = float_b.into();
        let matrix_c = matrix_a + matrix_b;

        // Control group
        let glam_a = glam::Mat4::from_cols_array(&origin_a);
        let glam_b = glam::Mat4::from_cols_array(&origin_b);
        let glam_c = glam_a + glam_b;

        // Compare `Float4x4` and `Control group`
        let a: [f32; 16] = float_c.into();
        let b: [f32; 16] = glam_c.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Add operation on `Float4x4` is invalid! (Float4x4:{:?}, Control group:{:?})", test, a, b);

        // Compare `Matrix` and `Control group`
        let a: [f32; 16] = { let t: Float4x4 = matrix_c.into(); t.into() };
        let b: [f32; 16] = glam_c.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Add operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix3x3_sub() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 9] = rng.gen();
        let origin_b: [f32; 9] = rng.gen();

        // Float3x3
        let float_a: Float3x3 = origin_a.into();
        let float_b: Float3x3 = origin_b.into();
        let float_c = float_a - float_b;

        // Matrix
        let matrix_a: Matrix = float_a.into();
        let matrix_b: Matrix = float_b.into();
        let matrix_c = matrix_a - matrix_b;

        // Control group
        let glam_a = glam::Mat3::from_cols_array(&origin_a);
        let glam_b = glam::Mat3::from_cols_array(&origin_b);
        let glam_c = glam_a - glam_b;

        // Compare `Float3x3` and `Control group`
        let a: [f32; 9] = float_c.into();
        let b: [f32; 9] = glam_c.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Float3x3` is invalid! (Float3x3:{:?}, Control group:{:?})", test, a, b);

        // Compare `Matrix` and `Control group`
        let a: [f32; 9] = { let t: Float3x3 = matrix_c.into(); t.into() };
        let b: [f32; 9] = glam_c.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix4x4_sub() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 16] = rng.gen();
        let origin_b: [f32; 16] = rng.gen();

        // Float4x4
        let float_a: Float4x4 = origin_a.into();
        let float_b: Float4x4 = origin_b.into();
        let float_c = float_a - float_b;

        // Matrix
        let matrix_a: Matrix = float_a.into();
        let matrix_b: Matrix = float_b.into();
        let matrix_c = matrix_a - matrix_b;

        // Control group
        let glam_a = glam::Mat4::from_cols_array(&origin_a);
        let glam_b = glam::Mat4::from_cols_array(&origin_b);
        let glam_c = glam_a - glam_b;

        // Compare `Float4x4` and `Control group`
        let a: [f32; 16] = float_c.into();
        let b: [f32; 16] = glam_c.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Float4x4` is invalid! (Float4x4:{:?}, Control group:{:?})", test, a, b);

        // Compare `Matrix` and `Control group`
        let a: [f32; 16] = { let t: Float4x4 = matrix_c.into(); t.into() };
        let b: [f32; 16] = glam_c.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix3x3_neg() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin: [f32; 9] = rng.gen();

        // Float3x3
        let float_a: Float3x3 = origin.into();
        let float_b = -float_a;

        // Matrix 
        let matrix_a: Matrix = float_a.into();
        let matrix_b = -matrix_a;

        // Control group
        let glam_a = glam::Mat3::from_cols_array(&origin);
        let glam_b = -glam_a;

        // Compare `Float3x3` and `Control group`
        let a: [f32; 9] = float_b.into();
        let b: [f32; 9] = glam_b.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Float3x3` is invalid! (Float3x3:{:?}, Control group:{:?})", test, a, b);

        // Compare `Matrix` and `Control group`
        let a: [f32; 9] = { let t: Float3x3 = matrix_b.into(); t.into() };
        let b: [f32; 9] = glam_b.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix4x4_neg() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin: [f32; 16] = rng.gen();

        // Float4x4
        let float_a: Float4x4 = origin.into();
        let float_b = -float_a;

        // Matrix 
        let matrix_a: Matrix = float_a.into();
        let matrix_b = -matrix_a;

        // Control group
        let glam_a = glam::Mat4::from_cols_array(&origin);
        let glam_b = -glam_a;

        // Compare `Float4x4` and `Control group`
        let a: [f32; 16] = float_b.into();
        let b: [f32; 16] = glam_b.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Float4x4` is invalid! (Float4x4:{:?}, Control group:{:?})", test, a, b);

        // Compare `Matrix` and `Control group`
        let a: [f32; 16] = { let t: Float4x4 = matrix_b.into(); t.into() };
        let b: [f32; 16] = glam_b.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test] 
fn matrix3x3_mul_scalar() {
    // `Matrix` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 9] = rng.gen();
        let scalar: f32 = rng.gen();

        // Float3x3 
        let float_a: Float3x3 = origin_a.into();
        let float_b = float_a * scalar;

        // Control group
        let glam_a = glam::Mat3::from_cols_array(&origin_a);
        let glam_b = glam_a * scalar;

        // Compare `Float3x3` and `Control group`
        let a: [f32; 9] = float_b.into();
        let b: [f32; 9]=  glam_b.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Multiply scalar operation on `Float3x3` is invalid! (Float3x3:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test] 
fn matrix4x4_mul_scalar() {
    // `Matrix` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 16] = rng.gen();
        let scalar: f32 = rng.gen();

        // Float4x4
        let float_a: Float4x4 = origin_a.into();
        let float_b = float_a * scalar;

        // Control group
        let glam_a = glam::Mat4::from_cols_array(&origin_a);
        let glam_b = glam_a * scalar;

        // Compare `Float4x4` and `Control group`
        let a: [f32; 16] = float_b.into();
        let b: [f32; 16]=  glam_b.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Multiply scalar operation on `Float4x4` is invalid! (Float4x4:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix3x3_mul() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 9] = rng.gen();
        let origin_b: [f32; 9] = rng.gen();

        // Float3x3
        let float_a: Float3x3 = origin_a.into();
        let float_b: Float3x3 = origin_b.into();
        let float_c = float_a * float_b;

        // Matrix
        let matrix_a: Matrix = float_a.into();
        let matrix_b: Matrix = float_b.into();
        let matrix_c = matrix_a * matrix_b;

        // Control group
        let glam_a = glam::Mat3::from_cols_array(&origin_a);
        let glam_b = glam::Mat3::from_cols_array(&origin_b);
        let glam_c = glam_a * glam_b;

        // Compare `Float3x3` and `Control group`
        let a: [f32; 9] = float_c.into();
        let b: [f32; 9] = glam_c.to_cols_array();
        let mut invalidate = false;
        for idx in 0..9 {
            invalidate |= (a[idx] - b[idx]).abs() > EPSILON;
        }
        assert!(!invalidate, "Test:{} >> Multiply operation on `Float3x3` is invalid! (Float3x3:{:?}, Control group:{:?})", test, a, b);

        // Compare `Matrix` and `Control group`
        let a: [f32; 9] = { let t: Float3x3 = matrix_c.into(); t.into() };
        let b: [f32; 9] = glam_c.to_cols_array();
        let mut invalidate = false;
        for idx in 0..9 {
            invalidate |= (a[idx] - b[idx]).abs() > EPSILON;
        }
        assert!(!invalidate, "Test:{} >> Multiply operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix4x4_mul() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 16] = rng.gen();
        let origin_b: [f32; 16] = rng.gen();

        // Float4x4
        let float_a: Float4x4 = origin_a.into();
        let float_b: Float4x4 = origin_b.into();
        let float_c = float_a * float_b;

        // Matrix
        let matrix_a: Matrix = float_a.into();
        let matrix_b: Matrix = float_b.into();
        let matrix_c = matrix_a * matrix_b;

        // Control group
        let glam_a = glam::Mat4::from_cols_array(&origin_a);
        let glam_b = glam::Mat4::from_cols_array(&origin_b);
        let glam_c = glam_a * glam_b;

        // Compare `Float3x3` and `Control group`
        let a: [f32; 16] = float_c.into();
        let b: [f32; 16] = glam_c.to_cols_array();
        let mut invalidate = false;
        for idx in 0..16 {
            invalidate |= (a[idx] - b[idx]).abs() > EPSILON;
        }
        assert!(!invalidate, "Test:{} >> Multiply operation on `Float4x4` is invalid! (Float4x4:{:?}, Control group:{:?})", test, a, b);

        // Compare `Matrix` and `Control group`
        let a: [f32; 16] = { let t: Float4x4 = matrix_c.into(); t.into() };
        let b: [f32; 16] = glam_c.to_cols_array();
        let mut invalidate = false;
        for idx in 0..16 {
            invalidate |= (a[idx] - b[idx]).abs() > EPSILON;
        }
        assert!(!invalidate, "Test:{} >> Multiply operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}
