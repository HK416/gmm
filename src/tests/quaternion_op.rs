use rand::Rng;
use crate::{Float4, Quaternion};
use super::{NUM_TEST, EPSILON};



#[test]
fn quaternion_mul() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin_a: [f32; 4] = rng.gen();
        let origin_b: [f32; 4] = rng.gen();

        // Quaternion
        let quat_a: Quaternion = { let t: Float4 = origin_a.into(); t.into() };
        let quat_b: Quaternion = { let t: Float4 = origin_b.into(); t.into() };
        let quat_c = quat_a * quat_b;

        // Control group
        let glam_a = glam::Quat::from_array(origin_a);
        let glam_b = glam::Quat::from_array(origin_b);
        let glam_c = glam_a * glam_b;

        // Compare `Quaternion` and `Control group`
        let a: [f32; 4] = { let t: Float4 = quat_c.into(); t.into() };
        let b: [f32; 4] = glam_c.into();
        let mut invaildate = false;
        for idx in 0..4 {
            invaildate |= (a[idx] - b[idx]).abs() > EPSILON;
        }
        assert!(!invaildate, "Test:{} >> Multiply operation on `Quaternion` is invalid! (Quaternion:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn quaternion_conjugate() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin: [f32; 4] = rng.gen();

        // Quaternion
        let quat_a: Quaternion = { let t: Float4 = origin.into(); t.into() };
        let quat_b = quat_a.conjugate();

        // Control group
        let glam_a = glam::Quat::from_array(origin);
        let glam_b = glam_a.conjugate();

        // Compare `Quaternion` and `Control group`
        let a: [f32; 4] = { let t: Float4 = quat_b.into(); t.into() };
        let b: [f32; 4] = glam_b.into();
        assert_eq!(a, b, "Test:{} >> Conjugate operation on `Quaternion` is invalid! (Quaternion:{:?}, Control group:{:?})", test, a, b);
    }
}

#[test]
fn quaternion_inverse() {
    let mut rng = rand::thread_rng();
    for test in 0..NUM_TEST {
        // Data 
        let origin: [f32; 4] = rng.gen();

        // Quaternion
        let quat_a: Quaternion = { let t: Float4 = origin.into(); t.into() };
        let quat_b = quat_a.inverse();

        // Control group
        let glam_a = glam::Quat::from_array(origin);
        let glam_b = (glam_a.length() > EPSILON).then(|| glam_a.normalize().inverse());

        // Compare `Quaternion` and `Control group`
        if let Some((quat_b, glam_b)) = quat_b.zip(glam_b) {
            let a: [f32; 4] = { let t: Float4 = quat_b.into(); t.into() };
            let b: [f32; 4] = glam_b.into();
            let mut invalidate = false;
            for idx in 0..4 {
                invalidate |= (a[idx] - b[idx]).abs() > EPSILON;
            }
            assert!(!invalidate, "Test:{} >> Conjudgate operation on `Quaternion` is invalid! (Quaternion:{:?}, Control group:{:?})", test, a, b);
        } else {
            panic!("Test:{} >> Inverse operation on `Quaternion` is invalid! (Quaternion:{:?}, Control group:{:?})", test, quat_b, glam_b);
        }
    }
}
