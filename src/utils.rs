//! This is a utility function module.
//! 
use crate::vec::*;
use crate::{
    Float3, Float4, Float4x4, 
};



/// Returns the quaternion of the x-axis rotated by a given angle. (radian)
#[inline]
#[must_use]
pub fn quaternion_rotation_x(angle: f32) -> Float4 {
    let (s, c) = (0.5 * angle).sin_cos();
    Float4 { x: s, y: 0.0, z: 0.0, w: c }
}

/// Returns the quaternion of the y-axis rotated by a given angle. (radian)
#[inline]
#[must_use]
pub fn quaternion_rotation_y(angle: f32) -> Float4 {
    let (s, c) = (0.5 * angle).sin_cos();
    Float4 { x: 0.0, y: s, z: 0.0, w: c }
}

/// Returns the quaternion of the z-axis rotated by a given angle. (radian)
#[inline]
#[must_use]
pub fn quaternion_rotation_z(angle: f32) -> Float4 {
    let (s, c) = (0.5 * angle).sin_cos();
    Float4 { x: 0.0, y: 0.0, z: s, w: c }
}

/// Returns each axis of a given quaternion.
/// 
/// Returns `None` if the given quaternion cannot be normalized.
/// 
#[inline]
#[must_use]
pub fn quaternion_to_axes(quat: Float4) -> Option<(Float4, Float4, Float4)> {
    let quat = load_float4(quat);
    let quat = vector4_normalize(quat)?;
    let quat = store_float4(quat);

    let x2 = quat.x + quat.x;
    let y2 = quat.y + quat.y;
    let z2 = quat.z + quat.z;
    let xx = quat.x * x2;
    let xy = quat.x * y2;
    let xz = quat.x * z2;
    let yy = quat.y * y2;
    let yz = quat.y * z2;
    let zz = quat.z * z2;
    let wx = quat.w * x2;
    let wy = quat.w * y2;
    let wz = quat.w * z2;

    let x_axis = Float4 { x: 1.0 - (yy + zz), y: xy + wz, z: xz - wy, w: 0.0 };
    let y_axis = Float4 { x: xy - wz, y: 1.0 - (xx + zz), z: yz + wx, w: 0.0 };
    let z_axis = Float4 { x: xz + wy, y: yz - wx, z: 1.0 - (xx + yy), w: 0.0 };
    Some((x_axis, y_axis, z_axis))
}

/// Returns the given quaternion as a matrix.
/// 
/// Returns `None` if the given quaternion cannot be normalized.
/// 
#[inline]
#[must_use]
pub fn quaternion_to_matrix(quat: Float4) -> Option<Float4x4> {
    let (x_axis, y_axis, z_axis) = quaternion_to_axes(quat)?;
    Some(Float4x4::from_columns(x_axis, y_axis, z_axis, Float4::W))
}

/// Returns the matrix of the x-axis rotated by a given angle. (radian)
#[inline]
#[must_use]
pub fn matrix_rotation_x(angle: f32) -> Float4x4 {
    let (s, c) = angle.sin_cos();
    Float4x4::from_columns(
        Float4::X, 
        Float4 { x: 0.0, y: c, z: s, w: 0.0 }, 
        Float4 { x: 0.0, y: -s, z: c, w: 0.0 }, 
        Float4::W
    )
}

/// Returns the matrix of the y-axis rotated by a given angle. (radian)
#[inline]
#[must_use]
pub fn matrix_rotation_y(angle: f32) -> Float4x4 {
    let (s, c) = angle.sin_cos();
    Float4x4::from_columns(
        Float4 { x: c, y: 0.0, z: -s, w: 0.0 }, 
        Float4::Y, 
        Float4 { x: s, y: 0.0, z: c, w: 0.0 }, 
        Float4::W
    )
}

/// Returns the matrix of the z-axis rotated by a given angle. (radian)
#[inline]
#[must_use]
pub fn matrix_rotation_z(angle: f32) -> Float4x4 {
    let (s, c) = angle.sin_cos();
    Float4x4::from_columns(
        Float4 { x: c, y: s, z: 0.0, w: 0.0 }, 
        Float4 { x: -s, y: c, z: 0.0, w: 0.0 }, 
        Float4::Z, 
        Float4::W
    )
}

/// Create a left-handed coordinate view matrix with the given `eye`, `dir`, and `up`.
/// 
/// Returns `None` if the view matrix cannot be created.
/// 
#[inline]
#[must_use]
pub fn matrix_look_to_lh(eye: Float3, dir: Float3, up: Float3) -> Option<Float4x4> {
    let neg_dir = store_float3(vector_neg(load_float3(dir)));
    matrix_look_to_rh(eye, neg_dir, up)
}

/// Create a right-handed coordinate view matrix with the given `eye`, `dir`, and `up`.
/// 
/// Returns `None` if the view matrix cannot be created.
/// 
#[inline]
#[must_use]
pub fn matrix_look_to_rh(eye: Float3, dir: Float3, up: Float3) -> Option<Float4x4> {
    let eye = load_float3(eye);
    let dir = load_float3(dir);
    let up = load_float3(up);

    let look = vector3_normalize(dir)?;
    let right = vector3_normalize(vector3_cross(look, up))?;
    let up = vector3_cross(right, look);
    
    let pos_x = -vector3_dot(eye, right);
    let pos_y = -vector3_dot(eye, up);
    let pos_z = vector3_dot(eye, look);

    let look = store_float3(look);
    let right = store_float3(right);
    let up = store_float3(up);

    Some(Float4x4::from_columns(
        Float4::new(right.x, up.x, -look.x, 0.0), 
        Float4::new(right.y, up.y, -look.y, 0.0), 
        Float4::new(right.z, up.z, -look.z, 0.0), 
        Float4::new(pos_x, pos_y, pos_z, 1.0)
    ))
}

/// Create a left-handed coordinate view matrix with the given `eye`, `at`, and `up`.
/// 
/// Returns `None` if the view matrix cannot be created.
/// 
#[inline]
#[must_use]
pub fn matrix_look_at_lh(eye: Float3, at: Float3, up: Float3) -> Option<Float4x4> {
    let dir = {
        let at = load_float3(at);
        let eye = load_float3(eye);
        let dir = vector_sub(at, eye);
        store_float3(dir)
    };
    matrix_look_to_lh(eye, dir, up)
}

/// Create a right-handed coordinate view matrix with the given `eye`, `at`, and `up`.
/// 
/// Returns `None` if the view matrix cannot be created.
/// 
#[inline]
#[must_use]
pub fn matrix_look_at_rh(eye: Float3, at: Float3, up: Float3) -> Option<Float4x4> {
    let dir = {
        let at = load_float3(at);
        let eye = load_float3(eye);
        let dir = vector_sub(at, eye);
        store_float3(dir)
    };
    matrix_look_to_rh(eye, dir, up)
}

/// Create a left-handed coordinate perspective projection matrix with the given `fov_y`, `aspect_ratio`, `z_near`, `z_far`.
/// - fov_y: radian
/// - depth range: 0.0 ~ 1.0
/// 
/// Returns `None` if the given `z_near` and `z_far` are less than or equal to zero.
/// 
#[inline]
#[must_use]
pub fn matrix_perspective_lh(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Option<Float4x4> {
    if z_near <= 0.0 || z_far <= 0.0 {
        return None;
    }

    let (s, c) = (fov_y * 0.5).sin_cos();
    let h = c / s;
    let w = h / aspect_ratio;
    let r = z_far / (z_far - z_near);
    Some(Float4x4::from_columns(
        Float4::new(w, 0.0, 0.0, 0.0), 
        Float4::new(0.0, h, 0.0, 0.0), 
        Float4::new(0.0, 0.0, r, 1.0), 
        Float4::new(0.0, 0.0, -r * z_near, 0.0)
    ))
}

/// Create a right-handed coordinate perspective projection matrix with the given `fov_y`, `aspect_ratio`, `z_near`, `z_far`.
/// - fov_y: radian
/// - depth range: 0.0 ~ 1.0
/// 
/// Returns `None` if the given `z_near` and `z_far` are less than or equal to zero.
/// 
#[inline]
#[must_use]
pub fn matrix_perspective_rh(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Option<Float4x4> {
    if z_near <= 0.0 || z_far <= 0.0 {
        return None;
    }

    let (s, c) = (fov_y * 0.5).sin_cos();
    let h = c / s;
    let w = h / aspect_ratio;
    let r = z_far / (z_near - z_far);
    Some(Float4x4::from_columns(
        Float4::new(w, 0.0, 0.0, 0.0), 
        Float4::new(0.0, h, 0.0, 0.0), 
        Float4::new(0.0, 0.0, r, -1.0), 
        Float4::new(0.0, 0.0, r * z_near, 0.0)
    ))
}

/// Create a left-handed coordinate orthographic projection matrix with the given `left`, `right`, `bottom`, `top`, `near`, `far`.
/// - depth range: 0.0 ~ 1.0
/// 
/// 1. If the given `left` and `right` are equal, returns `None`.
/// 2. If the given `bottom` and `top` are equal, returns `None`.
/// 3. If the given `near` and `far` are equal, returns `None`.
/// 
#[inline]
#[must_use]
pub fn matrix_orthographic_lh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Option<Float4x4> {
    let mut invalidate = (left - right).abs() <= f32::EPSILON;
    invalidate |= (bottom - top).abs() <= f32::EPSILON;
    invalidate |= (near - far).abs() <= f32::EPSILON;

    if invalidate {
        return None;
    }

    let recip_width = 1.0 / (right - left);
    let recip_height = 1.0 / (top - bottom);
    let recip_depth = 1.0 / (far - near);
    Some(Float4x4::from_columns(
        Float4::new(2.0 * recip_width, 0.0, 0.0, 0.0), 
        Float4::new(0.0, 2.0 * recip_height, 0.0, 0.0), 
        Float4::new(0.0, 0.0, recip_depth, 0.0), 
        Float4::new(
            -(left + right) * recip_width, 
            -(top + bottom) * recip_height, 
            -near * recip_depth, 
            1.0
        )
    ))
}

/// Create a right-handed coordinate orthographic projection matrix with the given `left`, `right`, `bottom`, `top`, `near`, `far`.
/// - depth range: 0.0 ~ 1.0
/// 
/// 1. If the given `left` and `right` are equal, returns `None`.
/// 2. If the given `bottom` and `top` are equal, returns `None`.
/// 3. If the given `near` and `far` are equal, returns `None`.
/// 
#[inline]
#[must_use]
pub fn matrix_orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Option<Float4x4> {
    let mut invalidate = (left - right).abs() <= f32::EPSILON;
    invalidate |= (bottom - top).abs() <= f32::EPSILON;
    invalidate |= (near - far).abs() <= f32::EPSILON;

    if invalidate {
        return None;
    }

    let recip_width = 1.0 / (right - left);
    let recip_height = 1.0 / (top - bottom);
    let recip_depth = 1.0 / (near - far);
    Some(Float4x4::from_columns(
        Float4::new(2.0 * recip_width, 0.0, 0.0, 0.0), 
        Float4::new(0.0, 2.0 * recip_height, 0.0, 0.0), 
        Float4::new(0.0, 0.0, recip_depth, 0.0), 
        Float4::new(
            -(left + right) * recip_width, 
            -(bottom + top) * recip_height, 
            near * recip_depth, 
            1.0
        )
    ))
}



#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::{Float3, Float4};
    use rand::{self, Rng};

    /// Some SIMD instructions do not conform to IEEE-754. (for performance benefits)
    /// 
    /// So we compare using a separate Epsilon constant.
    /// 
    const EPSILON: f32 = 1.192092896e-5;

    /// Verification number of tests
    const NUM_TEST: usize = 1_000_000;

    #[test]
    fn quaternion_to_matrix_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let arr: [f32; 4] = rng.gen();
            let quat = Float4::from_array(arr);
            let matrix = quaternion_to_matrix(quat);

            let g_quat = glam::Quat::from_array(arr).normalize();
            let g_matrix = glam::Mat4::from_quat(g_quat);

            if matrix.is_some() ^ g_quat.is_normalized() {
                panic!("invalid quaternion to matrix operation (test:{}, this:{}, glam:{})", test, quat, g_quat);
            }

            if let Some(matrix) = matrix {
                let raw_matrix = matrix.as_ref();
                let raw_g_matrix = g_matrix.as_ref();
                for idx in 0..16 {
                    let validate = (raw_matrix[idx] - raw_g_matrix[idx]).abs() <= EPSILON;
                    assert!(validate, "invalid quaternion to matrix operation (test:{}, this:{}, glam:{})", test, matrix, g_matrix);
                }
            }
        }
    }

    #[test]
    fn matrix_look_to_lh_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let pos_data: [f32; 3] = rng.gen();
            let rot_data: [f32; 4] = rng.gen();
            
            let pos = Float3::from_array(pos_data);
            let quat = Float4::from_array(rot_data);
            let axis = quaternion_to_axes(quat);

            let g_pos = glam::Vec3::from_array(pos_data);
            let g_quat = glam::Quat::from_array(rot_data).normalize();
            let g_axis = glam::Mat3::from_quat(g_quat);
            
            if axis.is_some() ^ g_quat.is_normalized() {
                panic!("invalid matrix look to lh operation (test:{}, this:{:?}, glam:{})", test, axis, g_axis);
            }

            if let Some((_, up, dir)) = axis {
                let view = matrix_look_to_lh(pos, dir.xyz(), up.xyz()).unwrap();
                let g_view = glam::Mat4::look_to_lh(g_pos, g_axis.z_axis, g_axis.y_axis);

                let raw_view = view.as_ref();
                let raw_g_view = g_view.as_ref();
                for idx in 0..16 {
                    let validate = (raw_view[idx] - raw_g_view[idx]).abs() <= EPSILON;
                    assert!(validate, "invalid matrix look to lh operation (test:{}, this:{}, glam:{})", test, view, g_view);
                }
            }
        }
    }

    #[test]
    fn matrix_look_to_rh_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let pos_data: [f32; 3] = rng.gen();
            let rot_data: [f32; 4] = rng.gen();
            
            let pos = Float3::from_array(pos_data);
            let quat = Float4::from_array(rot_data);
            let axis = quaternion_to_axes(quat);

            let g_pos = glam::Vec3::from_array(pos_data);
            let g_quat = glam::Quat::from_array(rot_data).normalize();
            let g_axis = glam::Mat3::from_quat(g_quat);
            
            if axis.is_some() ^ g_quat.is_normalized() {
                panic!("invalid matrix look to rh operation (test:{}, this:{:?}, glam:{})", test, axis, g_axis);
            }

            if let Some((_, up, dir)) = axis {
                let view = matrix_look_to_rh(pos, dir.xyz(), up.xyz()).unwrap();
                let g_view = glam::Mat4::look_to_rh(g_pos, g_axis.z_axis, g_axis.y_axis);

                let raw_view = view.as_ref();
                let raw_g_view = g_view.as_ref();
                for idx in 0..16 {
                    let validate = (raw_view[idx] - raw_g_view[idx]).abs() <= EPSILON;
                    assert!(validate, "invalid matrix look to rh operation (test:{}, this:{}, glam:{})", test, view, g_view);
                }
            }
        }
    }

    #[test]
    fn matrix_look_at_lh_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let mut pos_data = vec![0.0; 3];
            let mut at_data = vec![0.0; 3];
            for (pos, at) in pos_data.iter_mut().zip(at_data.iter_mut()) {
                *pos = rng.gen_range(-10.0f32..10.0f32);
                *at = rng.gen_range(-10.0f32..10.0f32);
            }

            let eye = Float3::from_slice(&pos_data);
            let at = Float3::from_slice(&at_data);
            let matrix = matrix_look_at_lh(eye, at, Float3::Y);

            let g_eye = glam::Vec3::from_slice(&pos_data);
            let g_at = glam::Vec3::from_slice(&at_data);
            let g_matrix = glam::Mat4::look_at_lh(g_eye, g_at, glam::Vec3::Y);

            if matrix.is_some() ^ ((g_at - g_eye).length() > f32::EPSILON) {
                panic!("invalid matrix look at lh operation (test:{}, this:{:?}, glam:{})", test, matrix, g_matrix);
            }

            if let Some(matrix) = matrix {
                let raw_matrix = matrix.as_ref();
                let raw_g_matrix = g_matrix.as_ref();
                for idx in 0..16 {
                    let validate = (raw_matrix[idx] - raw_g_matrix[idx]).abs() <= EPSILON;
                    assert!(validate, "invalid matrix look at lh operation (test:{}, this:{}, glam:{})", test, matrix, g_matrix);
                }
            }
        }
    }

    #[test]
    fn matrix_look_at_rh_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let mut pos_data = vec![0.0; 3];
            let mut at_data = vec![0.0; 3];
            for (pos, at) in pos_data.iter_mut().zip(at_data.iter_mut()) {
                *pos = rng.gen_range(-10.0f32..10.0f32);
                *at = rng.gen_range(-10.0f32..10.0f32);
            }

            let eye = Float3::from_slice(&pos_data);
            let at = Float3::from_slice(&at_data);
            let matrix = matrix_look_at_rh(eye, at, Float3::Y);

            let g_eye = glam::Vec3::from_slice(&pos_data);
            let g_at = glam::Vec3::from_slice(&at_data);
            let g_matrix = glam::Mat4::look_at_rh(g_eye, g_at, glam::Vec3::Y);

            if matrix.is_some() ^ ((g_at - g_eye).length() > f32::EPSILON) {
                panic!("invalid matrix look at rh operation (test:{}, this:{:?}, glam:{})", test, matrix, g_matrix);
            }

            if let Some(matrix) = matrix {
                let raw_matrix = matrix.as_ref();
                let raw_g_matrix = g_matrix.as_ref();
                for idx in 0..16 {
                    let validate = (raw_matrix[idx] - raw_g_matrix[idx]).abs() <= EPSILON;
                    assert!(validate, "invalid matrix look at rh operation (test:{}, this:{}, glam:{})", test, matrix, g_matrix);
                }
            }
        }
    }

    #[test]
    fn matrix_perspective_lh_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let fov_y = rng.gen_range(1.0f32..180.0f32).to_radians();
            let width = rng.gen_range(800.0f32..1920.0f32);
            let height = rng.gen_range(600.0..1080.0f32);
            let aspect_ratio = width / height;
            let z_near = rng.gen_range(0.001f32..0.1f32);
            let z_far = rng.gen_range(1.0f32..1000.0f32);

            let matrix = matrix_perspective_lh(fov_y, aspect_ratio, z_near, z_far).unwrap();
            let g_matrix = glam::Mat4::perspective_lh(fov_y, aspect_ratio, z_near, z_far);

            let raw_matrix = matrix.as_ref();
            let raw_g_matrix = g_matrix.as_ref();
            for idx in 0..16 {
                let validate = (raw_matrix[idx] - raw_g_matrix[idx]).abs() <= EPSILON;
                assert!(validate, "invalid matrix perspective lh operation! (test:{}, this:{}, glam:{})", test, matrix, g_matrix);
            }
        }
    }

    #[test]
    fn matrix_perspective_rh_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let fov_y = rng.gen_range(1.0f32..180.0f32).to_radians();
            let width = rng.gen_range(800.0f32..1920.0f32);
            let height = rng.gen_range(600.0..1080.0f32);
            let aspect_ratio = width / height;
            let z_near = rng.gen_range(0.001f32..0.1f32);
            let z_far = rng.gen_range(1.0f32..1000.0f32);

            let matrix = matrix_perspective_rh(fov_y, aspect_ratio, z_near, z_far).unwrap();
            let g_matrix = glam::Mat4::perspective_rh(fov_y, aspect_ratio, z_near, z_far);

            let raw_matrix = matrix.as_ref();
            let raw_g_matrix = g_matrix.as_ref();
            for idx in 0..16 {
                let validate = (raw_matrix[idx] - raw_g_matrix[idx]).abs() <= EPSILON;
                assert!(validate, "invalid matrix perspective rh operation! (test:{}, this:{}, glam:{})", test, matrix, g_matrix);
            }
        }
    }

    #[test]
    fn matrix_orthographic_lh_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let left = rng.gen_range(0.1f32..100.0f32) * -1.0;
            let right = rng.gen_range(0.1f32..100.0f32) * 1.0;
            let bottom = rng.gen_range(0.1f32..100.0f32) * -1.0;
            let top = rng.gen_range(0.1f32..100.0f32) * 1.0;
            let near = rng.gen_range(0.001f32..0.1f32);
            let far = rng.gen_range(1.0f32..1000.0f32);

            let matrix = matrix_orthographic_lh(left, right, bottom, top, near, far).unwrap();
            let g_matrix = glam::Mat4::orthographic_lh(left, right, bottom, top, near, far);

            let raw_matrix = matrix.as_ref();
            let raw_g_matrix = g_matrix.as_ref();
            for idx in 0..16 {
                let validate = (raw_matrix[idx] - raw_g_matrix[idx]).abs() <= EPSILON;
                assert!(validate, "invalid matrix orthographic lh operation! (test:{}, this:{}, glam:{})", test, matrix, g_matrix);
            }
        }
    }

    #[test]
    fn matrix_orthographic_rh_test() {
        let mut rng = rand::thread_rng();
        for test in 0..NUM_TEST {
            let left = rng.gen_range(0.1f32..100.0f32) * -1.0;
            let right = rng.gen_range(0.1f32..100.0f32) * 1.0;
            let bottom = rng.gen_range(0.1f32..100.0f32) * -1.0;
            let top = rng.gen_range(0.1f32..100.0f32) * 1.0;
            let near = rng.gen_range(0.001f32..0.1f32);
            let far = rng.gen_range(1.0f32..1000.0f32);

            let matrix = matrix_orthographic_rh(left, right, bottom, top, near, far).unwrap();
            let g_matrix = glam::Mat4::orthographic_rh(left, right, bottom, top, near, far);

            let raw_matrix = matrix.as_ref();
            let raw_g_matrix = g_matrix.as_ref();
            for idx in 0..16 {
                let validate = (raw_matrix[idx] - raw_g_matrix[idx]).abs() <= EPSILON;
                assert!(validate, "invalid matrix orthographic rh operation! (test:{}, this:{}, glam:{})", test, matrix, g_matrix);
            }
        }
    }
}
