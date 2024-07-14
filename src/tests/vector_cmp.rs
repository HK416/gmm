use rand::Rng;
use crate::{Boolean4, Float4, UInteger4, Vector};
use super::NUM_TEST;



#[test]
fn vector_min() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float4 = origin_b.into(); t.into() };
        let vector_c = vector_a.min(vector_b);

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a.min(glam_b);

        // Compare `Vector` and `Control group`
        let a: [f32; 4] = { let t: Float4 = vector_c.into(); t.into() };
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Min comparison on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector_max() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float4 = origin_b.into(); t.into() };
        let vector_c = vector_a.max(vector_b);

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a.max(glam_b);

        // Compare `Vector` and `Control group`
        let a: [f32; 4] = { let t: Float4 = vector_c.into(); t.into() };
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Max comparison on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector_lt() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float4 = origin_b.into(); t.into() };
        let vector_c = vector_a.lt(vector_b);

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a.cmplt(glam_b);

        // Compare `Vector` and `Control group`
        let a: [bool; 4] = { 
            let t: UInteger4 = vector_c.into(); 
            let t: Boolean4 = t.into();
            t.into()
        };
        let b: [bool; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Less comparison on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector_le() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float4 = origin_b.into(); t.into() };
        let vector_c = vector_a.le(vector_b);

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a.cmple(glam_b);

        // Compare `Vector` and `Control group`
        let a: [bool; 4] = { 
            let t: UInteger4 = vector_c.into(); 
            let t: Boolean4 = t.into();
            t.into()
        };
        let b: [bool; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Less than or eqaul comparison on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector_gt() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float4 = origin_b.into(); t.into() };
        let vector_c = vector_a.gt(vector_b);

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a.cmpgt(glam_b);

        // Compare `Vector` and `Control group`
        let a: [bool; 4] = { 
            let t: UInteger4 = vector_c.into(); 
            let t: Boolean4 = t.into();
            t.into()
        };
        let b: [bool; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Greater comparison on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector_ge() {
    // `Float*` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Vector
        let vector_a: Vector = { let t: Float4 = origin_a.into(); t.into() };
        let vector_b: Vector = { let t: Float4 = origin_b.into(); t.into() };
        let vector_c = vector_a.ge(vector_b);

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a.cmpge(glam_b);

        // Compare `Vector` and `Control group`
        let a: [bool; 4] = { 
            let t: UInteger4 = vector_c.into(); 
            let t: Boolean4 = t.into();
            t.into()
        };
        let b: [bool; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Greater than or equal comparison on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}
