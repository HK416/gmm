use rand::Rng;
use crate::{Float4, Float3x3, Float4x4, Matrix};
use super::{NUM_TEST, EPSILON};



#[test]
fn matrix3x3_transpose() {
    // `Float3x3` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin: [f32; 9] = rng.gen();
        
        // Matrix
        let matrix_a: Matrix = { let t: Float3x3 = origin.into(); t.into() };
        let matrix_t = matrix_a.transpose();

        // Control group
        let glam_a = glam::Mat3::from_cols_array(&origin);
        let glam_t = glam_a.transpose();

        // Compare `Matrix` and `Control group`
        let a: [f32; 9] = { let t: Float3x3 = matrix_t.into(); t.into() };
        let b: [f32; 9] = glam_t.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Transpose operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix4x4_transpose() {
    // `Float4x4` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin: [f32; 16] = rng.gen();
        
        // Matrix
        let matrix_a: Matrix = { let t: Float4x4 = origin.into(); t.into() };
        let matrix_t = matrix_a.transpose();

        // Control group
        let glam_a = glam::Mat4::from_cols_array(&origin);
        let glam_t = glam_a.transpose();

        // Compare `Matrix` and `Control group`
        let a: [f32; 16] = { let t: Float4x4 = matrix_t.into(); t.into() };
        let b: [f32; 16] = glam_t.to_cols_array();
        assert_eq!(a, b, "Test:{} >> Transpose operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn matrix3x3_determinant() {
    // `Float3x3` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin: [f32; 9] = rng.gen();
        
        // Matrix
        let matrix_a: Matrix = { let t: Float3x3 = origin.into(); t.into() };
        let matrix_d: f32 = { let t: Float4 = matrix_a.determinant().into(); t.x };

        // Control group
        let glam_a = glam::Mat3::from_cols_array(&origin);
        let glam_d = glam_a.determinant();

        // Compare `Matrix` and `Control group`
        let validate = (matrix_d - glam_d).abs() <= EPSILON;
        assert!(validate, "Test:{} >> Determinant operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, matrix_d, glam_d);
    }
}

#[test]
fn matrix4x4_determinant() {
    // `Float4x4` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin: [f32; 16] = rng.gen();
        
        // Matrix
        let matrix_a: Matrix = { let t: Float4x4 = origin.into(); t.into() };
        let matrix_d: f32 = { let t: Float4 = matrix_a.determinant().into(); t.x };

        // Control group
        let glam_a = glam::Mat4::from_cols_array(&origin);
        let glam_d = glam_a.determinant();

        // Compare `Matrix` and `Control group`
        let validate = (matrix_d - glam_d).abs() <= EPSILON;
        assert!(validate, "Test:{} >> Determinant operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, matrix_d, glam_d);
    }
}

// #[test]
// fn matrix3x3_inverse() {
//     // `Float3x3` data type does not support this operation.
//     let mut rng = rand::thread_rng();
//     for test in 0..NUM_TEST {
//         // Data
//         let origin: [f32; 9] = rng.gen();

//         // Matrix
//         let matrix_a: Matrix = { let t: Float3x3 = origin.into(); t.into() };
//         let matrix_inv = matrix_a.inverse();

//         // Control group
//         let glam_a = glam::Mat3::from_cols_array(&origin);
//         let glam_inv = (glam_a.determinant().abs() > f32::EPSILON).then(|| glam_a.inverse());

//         // Compare `Matrix` and `Control group`
//         if matrix_inv.is_none() & glam_inv.is_none() {
//             continue;
//         }

//         if let Some((matrix_inv, glam_inv)) = matrix_inv.zip(glam_inv) {
//             let a: [f32; 9] = { let t: Float3x3 = matrix_inv.into(); t.into() };
//             let b: [f32; 9] = glam_inv.to_cols_array();
//             for idx in 0..9 {
//                 let validate = (a[idx] - b[idx]).abs() <= EPSILON;
//                 assert!(validate, "Test:{}-{} >> Inverse operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, idx, a, b);
//             }
//         } else {
//             panic!("Test:{} >> Inverse operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, matrix_inv, glam_inv);
//         }
//     }
// }

#[test]
fn matrix4x4_inverse() {
    // `Float4x4` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin: [f32; 16] = rng.gen();

        // Matrix
        let matrix_a: Matrix = { let t: Float4x4 = origin.into(); t.into() };
        let matrix_det = matrix_a.determinant_into();
        let matrix_inv = matrix_a.inverse();

        // Control group
        let glam_a = glam::Mat4::from_cols_array(&origin);
        let glam_det = glam_a.determinant();
        let glam_inv = glam_a.inverse();

        // Compare `Matrix` and `Control group`
        assert!((matrix_det - glam_det).abs() <= f32::EPSILON, "Determinant operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", matrix_det, glam_det);
        if matrix_det.abs() <= f32::EPSILON {
            continue;
        }

        let a: [f32; 16] = { let t: Float4x4 = matrix_inv.into(); t.into() };
        let b: [f32; 16] = glam_inv.to_cols_array();
        for idx in 0..16 {
            let validate = (a[idx] - b[idx]).abs() <= EPSILON;
            assert!(validate, "Test:{}-{} >> Inverse operation on `Matrix` is invalid! (Matrix:{:?}, Control group:{:?})", test, idx, a, b);
        }
    }
}
