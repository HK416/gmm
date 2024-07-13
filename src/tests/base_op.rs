use rand::Rng;
use crate::{Float2, Float3, Float4, Vector};

/// Number of tests.
const NUM_TEST: usize = 1_000_000;



#[test]
fn vector2_add() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: [f32; 2] = rng.gen();

        // Float2
        let float_a: Float2 = origin_a.into();
        let float_b: Float2 = origin_b.into();
        let float_c = float_a + float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a + vector_b;

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b: glam::Vec2 = origin_b.into();
        let glam_c = glam_a + glam_b;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_c.into();
        let b: [f32; 2] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Add operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 2] = { let t: Float2 = vector_c.into(); t.into() };
        let b: [f32; 2] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Add operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_add() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: [f32; 3] = rng.gen();

        // Float3
        let float_a: Float3 = origin_a.into();
        let float_b: Float3 = origin_b.into();
        let float_c = float_a + float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a + vector_b;

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b: glam::Vec3 = origin_b.into();
        let glam_c = glam_a + glam_b;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_c.into();
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Add operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 3] = { let t: Float3 = vector_c.into(); t.into() };
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Add operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_add() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Float4
        let float_a: Float4 = origin_a.into();
        let float_b: Float4 = origin_b.into();
        let float_c = float_a + float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a + vector_b;

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a + glam_b;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_c.into();
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Add operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 4] = { let t: Float4 = vector_c.into(); t.into() };
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Add operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_add_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float2
        let float_a: Float2 = origin_a.into();
        let float_b = float_a + origin_b;

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b = glam_a + origin_b;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_b.into();
        let b: [f32; 2] = glam_b.into();
    assert_eq!(a, b, "Test:{} >> Add scalar operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_add_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float3
        let float_a: Float3 = origin_a.into();
        let float_b = float_a + origin_b;

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b = glam_a + origin_b;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_b.into();
        let b: [f32; 3] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Add scalar operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_add_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float4
        let float_a: Float4 = origin_a.into();
        let float_b = float_a + origin_b;

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b = glam_a + origin_b;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_b.into();
        let b: [f32; 4] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Add scalar operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_sub() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: [f32; 2] = rng.gen();

        // Float2
        let float_a: Float2 = origin_a.into();
        let float_b: Float2 = origin_b.into();
        let float_c = float_a - float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a - vector_b;

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b: glam::Vec2 = origin_b.into();
        let glam_c = glam_a - glam_b;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_c.into();
        let b: [f32; 2] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 2] = { let t: Float2 = vector_c.into(); t.into() };
        let b: [f32; 2] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_sub() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: [f32; 3] = rng.gen();

        // Float3
        let float_a: Float3 = origin_a.into();
        let float_b: Float3 = origin_b.into();
        let float_c = float_a - float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a - vector_b;

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b: glam::Vec3 = origin_b.into();
        let glam_c = glam_a - glam_b;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_c.into();
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 3] = { let t: Float3 = vector_c.into(); t.into() };
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_sub() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Float4
        let float_a: Float4 = origin_a.into();
        let float_b: Float4 = origin_b.into();
        let float_c = float_a - float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a - vector_b;

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a - glam_b;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_c.into();
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 4] = { let t: Float4 = vector_c.into(); t.into() };
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Subtract operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_sub_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float2
        let float_a: Float2 = origin_a.into();
        let float_b = float_a - origin_b;

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b = glam_a - origin_b;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_b.into();
        let b: [f32; 2] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Subtract scalar operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_sub_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float3
        let float_a: Float3 = origin_a.into();
        let float_b = float_a - origin_b;

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b = glam_a - origin_b;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_b.into();
        let b: [f32; 3] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Subtract scalar operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_sub_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float4
        let float_a: Float4 = origin_a.into();
        let float_b = float_a - origin_b;

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b = glam_a - origin_b;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_b.into();
        let b: [f32; 4] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Subtract scalar operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_neg() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin: [f32; 2] = rng.gen();

        // Float2
        let float_a: Float2 = origin.into();
        let float_b = -float_a;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b = -vector_a;

        // Control group
        let glam_a: glam::Vec2 = origin.into();
        let glam_b = -glam_a;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_b.into();
        let b: [f32; 2] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 2] = { let t: Float2 = vector_b.into(); t.into() };
        let b: [f32; 2] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_neg() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin: [f32; 3] = rng.gen();

        // Float3
        let float_a: Float3 = origin.into();
        let float_b = -float_a;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b = -vector_a;

        // Control group
        let glam_a: glam::Vec3 = origin.into();
        let glam_b = -glam_a;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_b.into();
        let b: [f32; 3] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 3] = { let t: Float3 = vector_b.into(); t.into() };
        let b: [f32; 3] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_neg() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin: [f32; 4] = rng.gen();

        // Float4
        let float_a: Float4 = origin.into();
        let float_b = -float_a;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b = -vector_a;

        // Control group
        let glam_a: glam::Vec4 = origin.into();
        let glam_b = -glam_a;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_b.into();
        let b: [f32; 4] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 4] = { let t: Float4 = vector_b.into(); t.into() };
        let b: [f32; 4] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Negative operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_mul() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: [f32; 2] = rng.gen();

        // Float2
        let float_a: Float2 = origin_a.into();
        let float_b: Float2 = origin_b.into();
        let float_c = float_a * float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a * vector_b;

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b: glam::Vec2 = origin_b.into();
        let glam_c = glam_a * glam_b;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_c.into();
        let b: [f32; 2] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Multiply operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 2] = { let t: Float2 = vector_c.into(); t.into() };
        let b: [f32; 2] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Multiply operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_mul() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: [f32; 3] = rng.gen();

        // Float3
        let float_a: Float3 = origin_a.into();
        let float_b: Float3 = origin_b.into();
        let float_c = float_a * float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a * vector_b;

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b: glam::Vec3 = origin_b.into();
        let glam_c = glam_a * glam_b;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_c.into();
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Multiply operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 3] = { let t: Float3 = vector_c.into(); t.into() };
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Multiply operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_mul() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Float4
        let float_a: Float4 = origin_a.into();
        let float_b: Float4 = origin_b.into();
        let float_c = float_a * float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a * vector_b;

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a * glam_b;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_c.into();
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Multiply operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 4] = { let t: Float4 = vector_c.into(); t.into() };
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Multiply operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_mul_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float2
        let float_a: Float2 = origin_a.into();
        let float_b = float_a * origin_b;

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b = glam_a * origin_b;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_b.into();
        let b: [f32; 2] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Multiply scalar operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_mul_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float3
        let float_a: Float3 = origin_a.into();
        let float_b = float_a * origin_b;

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b = glam_a * origin_b;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_b.into();
        let b: [f32; 3] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Multiply scalar operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_mul_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float4
        let float_a: Float4 = origin_a.into();
        let float_b = float_a * origin_b;

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b = glam_a * origin_b;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_b.into();
        let b: [f32; 4] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Multiply scalar operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_div() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: [f32; 2] = rng.gen();

        // Float2
        let float_a: Float2 = origin_a.into();
        let float_b: Float2 = origin_b.into();
        let float_c = float_a / float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a / vector_b;

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b: glam::Vec2 = origin_b.into();
        let glam_c = glam_a / glam_b;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_c.into();
        let b: [f32; 2] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Divide operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 2] = { let t: Float2 = vector_c.into(); t.into() };
        let b: [f32; 2] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Divide operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_div() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: [f32; 3] = rng.gen();

        // Float3
        let float_a: Float3 = origin_a.into();
        let float_b: Float3 = origin_b.into();
        let float_c = float_a / float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a / vector_b;

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b: glam::Vec3 = origin_b.into();
        let glam_c = glam_a / glam_b;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_c.into();
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Divide operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 3] = { let t: Float3 = vector_c.into(); t.into() };
        let b: [f32; 3] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Divide operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_div() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Float4
        let float_a: Float4 = origin_a.into();
        let float_b: Float4 = origin_b.into();
        let float_c = float_a / float_b;

        // Vector
        let vector_a: Vector = float_a.into();
        let vector_b: Vector = float_b.into();
        let vector_c = vector_a / vector_b;

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b: glam::Vec4 = origin_b.into();
        let glam_c = glam_a / glam_b;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_c.into();
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Divide operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);

        // Compare `Vector` and `Control group`
        let a: [f32; 4] = { let t: Float4 = vector_c.into(); t.into() };
        let b: [f32; 4] = glam_c.into();
        assert_eq!(a, b, "Test:{} >> Divide operation on `Vector` is invalid! (Vector:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector2_div_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 2] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float2
        let float_a: Float2 = origin_a.into();
        let float_b = float_a / origin_b;

        // Control group
        let glam_a: glam::Vec2 = origin_a.into();
        let glam_b = glam_a / origin_b;

        // Compare `Float2` and `Control group`
        let a: [f32; 2] = float_b.into();
        let b: [f32; 2] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Divide scalar operation on `Float2` is invalid! (Float2:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector3_div_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 3] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float3
        let float_a: Float3 = origin_a.into();
        let float_b = float_a / origin_b;

        // Control group
        let glam_a: glam::Vec3 = origin_a.into();
        let glam_b = glam_a / origin_b;

        // Compare `Float3` and `Control group`
        let a: [f32; 3] = float_b.into();
        let b: [f32; 3] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Divide scalar operation on `Float3` is invalid! (Float3:{:?}, Control Group:{:?})", test, a, b);
    }
}

#[test]
fn vector4_div_scalar() {
    // `Vector` data type does not support this operation.
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: f32 = rng.gen();

        // Float4
        let float_a: Float4 = origin_a.into();
        let float_b = float_a / origin_b;

        // Control group
        let glam_a: glam::Vec4 = origin_a.into();
        let glam_b = glam_a / origin_b;

        // Compare `Float4` and `Control group`
        let a: [f32; 4] = float_b.into();
        let b: [f32; 4] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Divide scalar operation on `Float4` is invalid! (Float4:{:?}, Control Group:{:?})", test, a, b);
    }
}
