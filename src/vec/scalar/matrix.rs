use core::fmt;
use core::ops;
use crate::{ 
    Vector, Quaternion, 
    Float3, Float3x3, Float4, Float4x4 
};



/// This is a matrix data type that uses the `SIMD` instruction.
/// 
/// Using the `scalar-math` feature disables the use of `SIMD` instructions.
/// 
/// It is recommended not to use this data types as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Matrix(pub(crate) Float4x4);

impl Matrix {
    /// Create a matrix with the given `translation`.
    #[inline]
    #[must_use]
    pub fn from_translation(translation: Vector) -> Self {
        let v: Float3 = translation.into();
        Float4x4 {
            w_axis: Float4 { x: v.x, y: v.y, z: v.z, w: 1.0 }, 
            ..Default::default()
        }.into()
    }

    /// Creates a matrix with the given `rotation` and `translation`.
    /// 
    /// ※ The given `rotation` must be normalized.
    /// 
    /// # Panics
    /// If use-assertion is enabled 
    /// and the given quaternion is not a normalized quaternion, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_translation(
        rotation: Quaternion, 
        translation: Vector
    ) -> Self {
        let (x_axis, y_axis, z_axis) = rotation.to_rotation_axes();
        let x: Float3 = x_axis.into();
        let y: Float3 = y_axis.into();
        let z: Float3 = z_axis.into();
        let v: Float3 = translation.into();
        Float4x4 {
            x_axis: Float4 { x: x.x, y: x.y, z: x.z, w: 0.0 }, 
            y_axis: Float4 { x: y.x, y: y.y, z: y.z, w: 0.0 }, 
            z_axis: Float4 { x: z.x, y: z.y, z: z.z, w: 0.0 }, 
            w_axis: Float4 { x: v.x, y: v.y, z: v.z, w: 1.0 }
        }.into()
    }

    /// Creates a matrix with the given `scale`, `rotation` and `translation`.
    /// 
    /// ※ The given `rotation` must be normalized.
    /// 
    /// # Panics
    /// If use-assertion is enabled 
    /// and the given quaternion is not a normalized quaternion, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn from_scale_rotation_translation(
        scale: Vector, 
        rotation: Quaternion, 
        translation: Vector
    ) -> Self {
        let (x_axis, y_axis, z_axis) = rotation.to_rotation_axes();
        let s: Float3 = scale.into();
        let x: Float3 = (x_axis).into();
        let y: Float3 = (y_axis).into();
        let z: Float3 = (z_axis).into();
        let v: Float3 = translation.into();
        Float4x4 {
            x_axis: Float4 { x: x.x * s.x, y: x.y * s.x, z: x.z * s.x, w: 0.0 }, 
            y_axis: Float4 { x: y.x * s.y, y: y.y * s.y, z: y.z * s.y, w: 0.0 }, 
            z_axis: Float4 { x: z.x * s.z, y: z.y * s.z, z: z.z * s.z, w: 0.0 }, 
            w_axis: Float4 { x: v.x, y: v.y, z: v.z, w: 1.0 }
        }.into()
    }
    
    /// Creates a matrix rotated by a given x-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let y_axis = Float4 { x: 0.0, y: c, z: s, w: 0.0 };
        let z_axis = Float4 { x: 0.0, y: -s, z: c, w: 0.0 };
        Float4x4 { y_axis, z_axis, ..Default::default() }.into()
    }

    /// Creates a matrix rotated by a given y-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let x_axis = Float4 { x: c, y: 0.0, z: -s, w: 0.0 };
        let z_axis = Float4 { x: s, y: 0.0, z: c, w: 0.0 };
        Float4x4 { x_axis, z_axis, ..Default::default() }.into()
    }

    /// Creates a matrix rotated by a given z-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let x_axis = Float4 { x: c, y: s, z: 0.0,  w: 0.0 };
        let y_axis = Float4 { x: -s, y: c, z: 0.0, w: 0.0 };
        Float4x4 { x_axis, y_axis, ..Default::default() }.into()
    }

    /// Create a right-handed coordinate view matrix with the given `eye`, `dir`, and `up`.
    /// 
    /// ※ The given `dir` and `up` must be unit vectors.
    /// 
    /// # Panics
    /// If `use-assertion` is enabled
    /// and the given `dir` and `up` is not unit vectors, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn look_to_rh(eye: Vector, dir: Vector, up: Vector) -> Self {
        #[cfg(feature = "use-assertion")] {
            let validate = dir.is_vec3_normalized()
            & up.is_vec3_normalized();
            assert!(validate, "The given 'dir' and 'up' must be unit vectors!");
        }

        let look = dir;
        let right = look.vec3_cross(up);
        let up = right.vec3_cross(look);

        let pos_x: Float4 = eye.vec3_dot(right).into();
        let pos_y: Float4 = eye.vec3_dot(up).into();
        let pos_z: Float4 = eye.vec3_dot(look).into();
        
        let look: Float4 = look.into();
        let right: Float4 = right.into();
        let up: Float4 = up.into();

        Float4x4 {
            x_axis: Float4 { x: right.x, y: up.x, z: -look.x, w: 0.0 }, 
            y_axis: Float4 { x: right.y, y: up.y, z: -look.y, w: 0.0 }, 
            z_axis: Float4 { x: right.z, y: up.z, z: -look.z, w: 0.0 }, 
            w_axis: Float4 { x: -pos_x.x, y: -pos_y.x, z: pos_z.x, w: 1.0 } 
        }.into()
    }

    /// Create a left-handed coordinate view matrix with the given `eye`, `dir`, and `up`.
    /// 
    /// ※ The given `dir` and `up` must be unit vectors.
    /// 
    /// # Panics
    /// If `use-assertion` is enabled
    /// and the given `dir` and `up` is not unit vectors, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn look_to_lh(eye: Vector, dir: Vector, up: Vector) -> Self {
        Self::look_to_rh(eye, -dir, up)
    }

    /// Create a right-handed coordinate view matrix with the given `eye`, `at`, and `up`.
    /// 
    /// ※ The given position of `eye` and `at` must be different.
    /// 
    /// # Panics 
    /// If `use-assertion` is enabled
    /// and the given `eye` and `at` is are the same, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn look_at_rh(eye: Vector, at: Vector, up: Vector) -> Self {
        Self::look_to_rh(eye, at - eye, up)
    }

    /// Create a left-handed coordinate view matrix with the given `eye`, `at`, and `up`.
    /// 
    /// ※ The given position of `eye` and `at` must be different.
    /// 
    /// # Panics
    /// If `use-assertion` is enabled
    /// and the given `eye` and `at` is are the same, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn look_at_lh(eye: Vector, at: Vector, up: Vector) -> Self {
        Self::look_to_lh(eye, at - eye, up)
    }

    /// Create a right-handed coordinate perspective projection matrix
    /// with the given `fov_y`, `aspect_ratio`, `z_near`, `z_far`.
    /// 
    /// ※ The depth of the created frustum ranges from `0.0` to `1.0`. </br>
    /// ※ The given `fov_y` is in radians. </br>
    /// ※ The given value of `z_near` and `z_far` must be different.
    /// 
    /// # Panics
    /// If `use-assertion` is enabled
    /// and the given `z_near` and `z_far` is are same, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn perspective_rh(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        #[cfg(feature = "use-assertion")] {
            let invalidate = (z_far - z_near).abs() <= f32::EPSILON;
            assert!(!invalidate, "The given value of 'z_near' and 'z_far' must be different!");
        }

        let (s, c) = (0.5 * fov_y).sin_cos();
        let h = c / s;
        let w = h / aspect_ratio;
        let r = z_far / (z_near - z_far);
        Float4x4 {
            x_axis: Float4 { x: w, y: 0.0, z: 0.0, w: 0.0 }, 
            y_axis: Float4 { x: 0.0, y: h, z: 0.0, w: 0.0 }, 
            z_axis: Float4 { x: 0.0, y: 0.0, z: r, w: -1.0 }, 
            w_axis: Float4 { x: 0.0, y: 0.0, z: r * z_near, w: 0.0 }
        }.into()
    }

    /// Create a left-handed coordinate perspective projection matrix
    /// with the given `fov_y`, `aspect_ratio`, `z_near`, `z_far`.
    /// 
    /// ※ The depth of the created frustum ranges from `0.0` to `1.0`. </br>
    /// ※ The given `fov_y` is in radians. </br>
    /// ※ The given value of `z_near` and `z_far` must be different.
    /// 
    /// # Panics
    /// If use-assertion is enabled 
    /// and the given z_near and z_far is are same, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn perspective_lh(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        #[cfg(feature = "use-assertion")] {
            let invalidate = (z_far - z_near).abs() <= f32::EPSILON;
            assert!(!invalidate, "The given value of 'z_near' and 'z_far' must be different!");
        }

        let (s, c) = (0.5 * fov_y).sin_cos();
        let h = c / s;
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);
        Float4x4 {
            x_axis: Float4 { x: w, y: 0.0, z: 0.0, w: 0.0 }, 
            y_axis: Float4 { x: 0.0, y: h, z: 0.0, w: 0.0 }, 
            z_axis: Float4 { x: 0.0, y: 0.0, z: r, w: 1.0 }, 
            w_axis: Float4 { x: 0.0, y: 0.0, z: -r * z_near, w: 0.0 }
        }.into()
    }

    /// Create a right-handed coordinate orthographic projection matrix
    /// with the given `left`, `right`, `bottom`, `top`, `near`, `far`.
    /// 
    /// ※ The depth of the created frustum ranges from `0.0` to `1.0`. </br>
    /// ※ The given value of `left` and `right` must be different. </br>
    /// ※ The given value of `bottom` and `top` must be different. </br>
    /// ※ The given value of `near` and `far` must be different. </br>
    /// 
    /// # Panics
    /// If use-assertion is enabled 
    /// and given 'left' and 'right' are equal, 
    /// 'bottom' and 'top' are equal, or 'near' and 'far' are equal, 
    /// a [`panic!`] is called.
    /// 
    /// 
    #[inline]
    #[must_use]
    pub fn orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        #[cfg(feature = "use-assertion")] {
            let invalidate = (left - right).abs() <= f32::EPSILON;
            assert!(!invalidate, "The given value of 'left' and 'right' must be different!");
            let invalidate = (bottom - top).abs() <= f32::EPSILON;
            assert!(!invalidate, "The given value of 'bottom' and 'top' must be different!");
            let invalidate = (near - far).abs() <= f32::EPSILON;
            assert!(!invalidate, "The given value of 'near' and 'far' must be different!");
        }

        let recip_width = 1.0 / (right - left);
        let recip_height = 1.0 / (top - bottom);
        let recip_depth = 1.0 / (near - far);
        Float4x4 {
            x_axis: Float4 { x: 2.0 * recip_width, y: 0.0, z: 0.0, w: 0.0 }, 
            y_axis: Float4 { x: 0.0, y: 2.0 * recip_height, z: 0.0, w: 0.0 }, 
            z_axis: Float4 { x: 0.0, y: 0.0, z: recip_depth, w: 0.0 }, 
            w_axis: Float4 { 
                x: -(left + right) * recip_width, 
                y: -(bottom + top) * recip_height, 
                z: near * recip_depth, 
                w: 1.0 
            }
        }.into()
    }

    /// Create a left-handed coordinate orthographic projection matrix
    /// with the given `left`, `right`, `bottom`, `top`, `near`, `far`.
    /// 
    /// ※ The depth of the created frustum ranges from `0.0` to `1.0`. </br>
    /// ※ The given value of `left` and `right` must be different. </br>
    /// ※ The given value of `bottom` and `top` must be different. </br>
    /// ※ The given value of `near` and `far` must be different. </br>
    /// 
    /// # Panics
    /// If use-assertion is enabled 
    /// and given 'left' and 'right' are equal, 
    /// 'bottom' and 'top' are equal, or 'near' and 'far' are equal, 
    /// a [`panic!`] is called.
    /// 
    /// 
    #[inline]
    #[must_use]
    pub fn orthographic_lh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        #[cfg(feature = "use-assertion")] {
            let invalidate = (left - right).abs() <= f32::EPSILON;
            assert!(!invalidate, "The given value of 'left' and 'right' must be different!");
            let invalidate = (bottom - top).abs() <= f32::EPSILON;
            assert!(!invalidate, "The given value of 'bottom' and 'top' must be different!");
            let invalidate = (near - far).abs() <= f32::EPSILON;
            assert!(!invalidate, "The given value of 'near' and 'far' must be different!");
        }

        let recip_width = 1.0 / (right - left);
        let recip_height = 1.0 / (top - bottom);
        let recip_depth = 1.0 / (far - near);
        Float4x4 {
            x_axis: Float4 { x: 2.0 * recip_width, y: 0.0, z: 0.0, w: 0.0 }, 
            y_axis: Float4 { x: 0.0, y: 2.0 * recip_height, z: 0.0, w: 0.0 }, 
            z_axis: Float4 { x: 0.0, y: 0.0, z: recip_depth, w: 0.0 }, 
            w_axis: Float4 { 
                x: -(left + right) * recip_width, 
                y: -(bottom + top) * recip_height, 
                z: -near * recip_depth, 
                w: 1.0 
            }
        }.into()
    }
}

impl Matrix {
    /// Transpose of a matrix.
    pub fn transpose(self) -> Self {
        // Origin:
        // m00 m01 m02 m03 
        // m10 m11 m12 m13
        // m20 m21 m22 m23
        // m30 m31 m32 m33
        // 
        Float4x4 {
            x_axis: Float4 { x: self[0][0], y: self[1][0], z: self[2][0], w: self[3][0] }, 
            y_axis: Float4 { x: self[0][1], y: self[1][1], z: self[2][1], w: self[3][1] }, 
            z_axis: Float4 { x: self[0][2], y: self[1][2], z: self[2][2], w: self[3][2] }, 
            w_axis: Float4 { x: self[0][3], y: self[1][3], z: self[2][3], w: self[3][3] } 
        }.into()
    }

    /// Determinant of a matrix.
    pub fn determinant(self) -> Vector {
        // Reference: glm/detail/func_matrix.inl
        let fac0 = Float4::new(
            self[2][2]*self[3][3] - self[3][2]*self[2][3], 
            self[2][2]*self[3][3] - self[3][2]*self[2][3], 
            self[1][2]*self[3][3] - self[3][2]*self[1][3], 
            self[1][2]*self[2][3] - self[2][2]*self[1][3]
        );
        let fac1 = Float4::new(
            self[2][1]*self[3][3] - self[3][1]*self[2][3], 
            self[2][1]*self[3][3] - self[3][1]*self[2][3], 
            self[1][1]*self[3][3] - self[3][1]*self[1][3], 
            self[1][1]*self[2][3] - self[2][1]*self[1][3]
        );
        let fac2 = Float4::new(
            self[2][1]*self[3][2] - self[3][1]*self[2][2], 
            self[2][1]*self[3][2] - self[3][1]*self[2][2], 
            self[1][1]*self[3][2] - self[3][1]*self[1][2], 
            self[1][1]*self[2][2] - self[2][1]*self[1][2]
        );
        let fac3 = Float4::new(
            self[2][0]*self[3][3] - self[3][0]*self[2][3], 
            self[2][0]*self[3][3] - self[3][0]*self[2][3], 
            self[1][0]*self[3][3] - self[3][0]*self[1][3], 
            self[1][0]*self[2][3] - self[2][0]*self[1][3], 
        );
        let fac4 = Float4::new(
            self[2][0]*self[3][2] - self[3][0]*self[2][2], 
            self[2][0]*self[3][2] - self[3][0]*self[2][2], 
            self[1][0]*self[3][2] - self[3][0]*self[1][2], 
            self[1][0]*self[2][2] - self[2][0]*self[1][2]
        );
        let fac5 = Float4::new(
            self[2][0]*self[3][1] - self[3][0]*self[2][1], 
            self[2][0]*self[3][1] - self[3][0]*self[2][1], 
            self[1][0]*self[3][1] - self[3][0]*self[1][1], 
            self[1][0]*self[2][1] - self[2][0]*self[1][1]
        );


        let vec0 = Float4::new(self[1][0], self[0][0], self[0][0], self[0][0]);
        let vec1 = Float4::new(self[1][1], self[0][1], self[0][1], self[0][1]);
        let vec2 = Float4::new(self[1][2], self[0][2], self[0][2], self[0][2]);
        let vec3 = Float4::new(self[1][3], self[0][3], self[0][3], self[0][3]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let one_neg_one_neg = Float4::new(1.0, -1.0, 1.0, -1.0);
        let neg_one_neg_one = Float4::new(-1.0, 1.0, -1.0, 1.0);
        let inverse = Float4x4 {
            x_axis: inv0 * one_neg_one_neg, 
            y_axis: inv1 * neg_one_neg_one, 
            z_axis: inv2 * one_neg_one_neg, 
            w_axis: inv3 * neg_one_neg_one
        };

        let row0 = Float4::new(inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);
        let det = self[0] * row0;
        let det = det[0] + det[1] + det[2] + det[3];
        return Float4::fill(det).into();
    }

    /// Inverse of a matrix.
    /// If the inverse of matrix cannot be calculated, returns `None`.
    pub fn inverse(self) -> Option<Self> {
        // Reference: glm/detail/func_matrix.inl
        let fac0 = Float4::new(
            self[2][2]*self[3][3] - self[3][2]*self[2][3], 
            self[2][2]*self[3][3] - self[3][2]*self[2][3], 
            self[1][2]*self[3][3] - self[3][2]*self[1][3], 
            self[1][2]*self[2][3] - self[2][2]*self[1][3]
        );
        let fac1 = Float4::new(
            self[2][1]*self[3][3] - self[3][1]*self[2][3], 
            self[2][1]*self[3][3] - self[3][1]*self[2][3], 
            self[1][1]*self[3][3] - self[3][1]*self[1][3], 
            self[1][1]*self[2][3] - self[2][1]*self[1][3]
        );
        let fac2 = Float4::new(
            self[2][1]*self[3][2] - self[3][1]*self[2][2], 
            self[2][1]*self[3][2] - self[3][1]*self[2][2], 
            self[1][1]*self[3][2] - self[3][1]*self[1][2], 
            self[1][1]*self[2][2] - self[2][1]*self[1][2]
        );
        let fac3 = Float4::new(
            self[2][0]*self[3][3] - self[3][0]*self[2][3], 
            self[2][0]*self[3][3] - self[3][0]*self[2][3], 
            self[1][0]*self[3][3] - self[3][0]*self[1][3], 
            self[1][0]*self[2][3] - self[2][0]*self[1][3], 
        );
        let fac4 = Float4::new(
            self[2][0]*self[3][2] - self[3][0]*self[2][2], 
            self[2][0]*self[3][2] - self[3][0]*self[2][2], 
            self[1][0]*self[3][2] - self[3][0]*self[1][2], 
            self[1][0]*self[2][2] - self[2][0]*self[1][2]
        );
        let fac5 = Float4::new(
            self[2][0]*self[3][1] - self[3][0]*self[2][1], 
            self[2][0]*self[3][1] - self[3][0]*self[2][1], 
            self[1][0]*self[3][1] - self[3][0]*self[1][1], 
            self[1][0]*self[2][1] - self[2][0]*self[1][1]
        );


        let vec0 = Float4::new(self[1][0], self[0][0], self[0][0], self[0][0]);
        let vec1 = Float4::new(self[1][1], self[0][1], self[0][1], self[0][1]);
        let vec2 = Float4::new(self[1][2], self[0][2], self[0][2], self[0][2]);
        let vec3 = Float4::new(self[1][3], self[0][3], self[0][3], self[0][3]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let one_neg_one_neg = Float4::new(1.0, -1.0, 1.0, -1.0);
        let neg_one_neg_one = Float4::new(-1.0, 1.0, -1.0, 1.0);
        let inverse = Float4x4 {
            x_axis: inv0 * one_neg_one_neg, 
            y_axis: inv1 * neg_one_neg_one, 
            z_axis: inv2 * one_neg_one_neg, 
            w_axis: inv3 * neg_one_neg_one
        };

        let row0 = Float4::new(inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);
        let det = self[0] * row0;
        let det = det[0] + det[1] + det[2] + det[3];
        if det.abs() <= f32::EPSILON {
            return None;
        }

        let recip_det = det.recip();
        Some(Float4x4 {
            x_axis: inverse[0] * recip_det, 
            y_axis: inverse[1] * recip_det, 
            z_axis: inverse[2] * recip_det, 
            w_axis: inverse[3] * recip_det
        }.into())
    }
}

impl From<[f32; 16]> for Matrix {
    #[inline]
    fn from(value: [f32; 16]) -> Self {
        Self::from(Float4x4::from(value))
    }
}

impl Into<[f32; 16]> for Matrix {
    #[inline]
    fn into(self) -> [f32; 16] {
        let value: Float4x4 = self.into();
        value.into()
    }
}

impl From<Float3x3> for Matrix {
    #[inline]
    fn from(value: Float3x3) -> Self {
        Self::from(Float4x4::from(value))
    }
}

impl Into<Float3x3> for Matrix {
    #[inline]
    fn into(self) -> Float3x3 {
        let value: Float4x4 = self.into();
        return value.into();
    }
}

impl From<Float4x4> for Matrix {
    #[inline]
    fn from(value: Float4x4) -> Self {
        Self(value)
    }
}

impl Into<Float4x4> for Matrix {
    #[inline]
    fn into(self) -> Float4x4 {
        *self
    }
}

impl ops::Deref for Matrix {
    type Target = Float4x4;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Matrix {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ops::Add<Self> for Matrix {
    type Output = Self;
    /// Adds two matrices.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Matrix(*self + *rhs)
    }
}

impl ops::AddAssign<Self> for Matrix {
    /// Adds two matrices. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub<Self> for Matrix {
    type Output = Self;
    /// Subtracts two matrices.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix(*self - *rhs)
    }
}

impl ops::SubAssign<Self> for Matrix {
    /// Subtracts two matrices. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl ops::Neg for Matrix {
    type Output = Self;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        Matrix(-*self)
    }
}

impl ops::Mul<Vector> for Matrix {
    type Output = Vector;
    /// Transformation of the vector.
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(*self * *rhs)
    }
}

impl ops::Mul<Self> for Matrix {
    type Output = Self;
    /// Multiplies two matrices.
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix(*self * *rhs)
    }
}

impl ops::MulAssign<Self> for Matrix {
    /// Multiplies two matrices. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl fmt::Debug for Matrix {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Matrix))
            .field(&self[0])
            .field(&self[1])
            .field(&self[2])
            .field(&self[3])
            .finish()
    }
}
