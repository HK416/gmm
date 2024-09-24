use core::fmt;
use core::ops;
use core::arch::aarch64::*;
use crate::{ Vector, Quaternion, Float3x3, Float4x4 };



/// This is a matrix data type that uses the `SIMD` instruction.
/// 
/// Using the `arm neon` instruction.
/// 
#[repr(C)]
#[derive(Clone, Copy)]
pub union Matrix {
    /// member variables for constant variables.
    arr: [f32; 16], 

    pub(crate) columns: [Vector; 4], 

    pub(crate) inner: float32x4x4_t, 
}

impl Matrix {
    /// All elements are zeros.
    pub const ZERO: Self = Self { arr: [0.0; 16] };

    /// Identity matrix.
    pub const IDENTITY: Self = Self { 
        arr: [
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0
        ] 
    };
}

impl Matrix {
    /// Creates with given elements.
    #[inline]
    #[must_use]
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32, 
        m10: f32, m11: f32, m12: f32, m13: f32, 
        m20: f32, m21: f32, m22: f32, m23: f32, 
        m30: f32, m31: f32, m32: f32, m33: f32
    ) -> Self {
        Self::from_columns(
            Vector::new(m00, m01, m02, m03), 
            Vector::new(m10, m11, m12, m13), 
            Vector::new(m20, m21, m22, m23), 
            Vector::new(m30, m31, m32, m33)
        )
    }

    /// Creates a diagonal matrix.
    #[inline]
    #[must_use]
    pub fn diagonal(diagonal: Vector) -> Self {
        Self::new(
            diagonal.get_x(), 0.0, 0.0, 0.0, 
            0.0, diagonal.get_y(), 0.0, 0.0, 
            0.0, 0.0, diagonal.get_z(), 0.0, 
            0.0, 0.0, 0.0, diagonal.get_w()
        )
    }

    /// Creates with given column vectors.
    #[inline]
    #[must_use]
    pub const fn from_columns(
        x_axis: Vector, 
        y_axis: Vector, 
        z_axis: Vector, 
        w_axis: Vector
) -> Self {
        Self { columns: [x_axis, y_axis, z_axis, w_axis] }
    }

    /// Creates from a given array.
    #[inline]
    #[must_use]
    pub fn from_column_array(arr: [f32; 16]) -> Self {
        unsafe { Self { inner: vld1q_f32_x4(arr.as_ptr()) } }
    }

    /// Stores the value in an array.
    #[inline]
    #[must_use]
    pub fn into_column_array(self) -> [f32; 16] {
        let mut arr = [0.0; 16];
        unsafe { vst1q_f32_x4(arr.as_mut_ptr(), self.inner) };
        return arr;
    }

    /// Creates from a given array of slice.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the array slice has less than sixteen elements.
    /// 
    #[inline]
    #[must_use]
    pub fn from_column_slice(slice: &[f32]) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(slice.len() >= 16, "The given array slice has less than sixteen elements!");
        unsafe { Self { inner: vld1q_f32_x4(slice.as_ptr()) } }
    }

    /// Loads a value from a given `Float4x4`.
    #[inline]
    #[must_use]
    pub fn load_float3x3(val: Float3x3) -> Self {
        Self::from_columns(
            Vector::load_float3(val.x_axis), 
            Vector::load_float3(val.x_axis), 
            Vector::load_float3(val.x_axis), 
            Vector::W
        )
    }

    /// Stores the value in a `Float4x4`.
    #[inline]
    #[must_use]
    pub fn store_float3x3(self) -> Float3x3 {
        Float3x3 {
            x_axis: unsafe { self.columns[0].store_float3() }, 
            y_axis: unsafe { self.columns[1].store_float3() }, 
            z_axis: unsafe { self.columns[2].store_float3() }, 
        }
    }

    /// Loads a value from a given `Float4x4`.
    #[inline]
    #[must_use]
    pub fn load_float4x4(val: Float4x4) -> Self {
        Self::from_column_array(val.to_column_array())
    }

    /// Stores the value in a `Float4x4`.
    #[inline]
    #[must_use]
    pub fn store_float4x4(self) -> Float4x4 {
        Float4x4::from_column_array(self.into_column_array())
    }

    /// Create a matrix with the given `translation`.
    #[inline]
    #[must_use]
    pub fn from_translation(mut translation: Vector) -> Self {
        translation.set_w(1.0);
        Self::from_columns(Vector::X, Vector::Y, Vector::Z, translation)
    }

    /// Creates a matrix with the given `rotation` and `translation`.
    /// 
    /// ※ The given `rotation` must be normalized.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given quaternion is not a normalized quaternion.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_translation(
        rotation: Quaternion, 
        mut translation: Vector
    ) -> Self {
        translation.set_w(1.0);
        let (x_axis, y_axis, z_axis) = rotation.to_rotation_axes();
        Self::from_columns(x_axis, y_axis, z_axis, translation)
    }

    /// Creates a matrix with the given `scale`, `rotation` and `translation`.
    /// 
    /// ※ The given `rotation` must be normalized.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given quaternion is not a normalized quaternion.
    /// 
    #[inline]
    #[must_use]
    pub fn from_scale_rotation_translation(
        scale: Vector, 
        rotation: Quaternion, 
        mut translation: Vector
    ) -> Self {
        translation.set_w(1.0);
        let (x_axis, y_axis, z_axis) = rotation.to_rotation_axes();
        Self::from_columns(
            x_axis * scale.get_x(), 
            y_axis * scale.get_y(), 
            z_axis * scale.get_z(), 
            translation
        )
    }
    
    /// Creates a matrix rotated by a given x-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self::from_columns(
            Vector::X, 
            Vector::new(0.0, c, s, 0.0), 
            Vector::new(0.0, -s, c, 0.0), 
            Vector::W
        )
    }

    /// Creates a matrix rotated by a given y-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self::from_columns(
            Vector::new(c, 0.0, -s, 0.0), 
            Vector::Y, 
            Vector::new(s, 0.0, c, 0.0), 
            Vector::W
        )
    }

    /// Creates a matrix rotated by a given z-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self::from_columns(
            Vector::new(c, s, 0.0, 0.0), 
            Vector::new(-s, c, 0.0, 0.0), 
            Vector::Z, 
            Vector::W
        )
    }

    /// Create a right-handed coordinate view matrix with the given `eye`, `dir`, and `up`.
    /// 
    /// ※ The given `dir` and `up` must be unit vectors.
    /// 
    /// # Panics
    /// 
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given `dir` and `up` is not unit vectors.
    /// 
    #[inline]
    #[must_use]
    pub fn look_to_rh(eye: Vector, dir: Vector, up: Vector) -> Self {
        #[cfg(feature = "use-assertion")] {
            assert!(dir.is_vec3_normalized(), "The given `dir` must be unit vector!");
            assert!(up.is_vec3_normalized(), "The given `up` must be unit vector!");
        }

        let mut look = dir;
        let mut right = look.vec3_cross(up);
        let mut up = right.vec3_cross(look);

        let pos_x = eye.vec3_dot_into(right);
        let pos_y = eye.vec3_dot_into(up);
        let pos_z = eye.vec3_dot_into(look);

        right.set_w(-pos_x);
        up.set_w(-pos_y);
        look.set_w(-pos_z);

        Self::from_columns(right, up, -look, Vector::W)
            .transpose()
    }

    /// Create a left-handed coordinate view matrix with the given `eye`, `dir`, and `up`.
    /// 
    /// ※ The given `dir` and `up` must be unit vectors.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given `dir` and `up` is not unit vectors.
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
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given `eye` and `at` are equal.
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
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given `eye` and `at` are equal.
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
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given `z_near` and `z_far` are equal.
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
        Self::new(
            w, 0.0, 0.0, 0.0, 
            0.0, h, 0.0, 0.0, 
            0.0, 0.0, r, -1.0, 
            0.0, 0.0, r * z_near, 0.0
        )
    }

    /// Create a left-handed coordinate perspective projection matrix
    /// with the given `fov_y`, `aspect_ratio`, `z_near`, `z_far`.
    /// 
    /// ※ The depth of the created frustum ranges from `0.0` to `1.0`. </br>
    /// ※ The given `fov_y` is in radians. </br>
    /// ※ The given value of `z_near` and `z_far` must be different.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given `z_near` and `z_far` are equal.
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
        Self::new(
            w, 0.0, 0.0, 0.0, 
            0.0, h, 0.0, 0.0, 
            0.0, 0.0, r, 1.0, 
            0.0, 0.0, -r * z_near, 0.0
        )
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
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given `left` and `right` are equal 
    /// or `bottom` and `top` are equal
    /// or `near` and `far` are equal.
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
        Self::new(
            2.0 * recip_width, 0.0, 0.0, 0.0, 
            0.0, 2.0 * recip_height, 0.0, 0.0, 
            0.0, 0.0, recip_depth, 0.0, 
            -(left + right) * recip_width, 
            -(bottom + top) * recip_height, 
            near * recip_depth, 
            1.0
        )
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
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the given `left` and `right` are equal 
    /// or `bottom` and `top` are equal
    /// or `near` and `far` are equal.
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
        Self::new(
            2.0 * recip_width, 0.0, 0.0, 0.0, 
            0.0, 2.0 * recip_height, 0.0, 0.0, 
            0.0, 0.0, recip_depth, 0.0, 
            -(left + right) * recip_width, 
            -(bottom + top) * recip_height, 
            -near * recip_depth, 
            1.0
        )
    }
}

impl Matrix {
    /// Get the x-axis of a matrix.
    #[inline]
    #[must_use]
    pub fn get_x_axis(&self) -> &Vector {
        unsafe { self.columns.get_unchecked(0) }
    }

    /// Set the x-axis of a matrix.
    #[inline]
    #[must_use]
    pub fn set_x_axis(&mut self, v: Vector) {
        unsafe { *self.columns.get_unchecked_mut(0) = v }
    }

    /// Get the y-axis of a matrix.
    #[inline]
    #[must_use]
    pub fn get_y_axis(&self) -> &Vector {
        unsafe { self.columns.get_unchecked(1) }
    }

    /// Set the y-axis of a matrix.
    #[inline]
    #[must_use]
    pub fn set_y_axis(&mut self, v: Vector) {
        unsafe { *self.columns.get_unchecked_mut(1) = v }
    }

    /// Get the z-axis of a matrix.
    #[inline]
    #[must_use]
    pub fn get_z_axis(&self) -> &Vector {
        unsafe { self.columns.get_unchecked(2) }
    }

    /// Set the z-axis of a matrix.
    #[inline]
    #[must_use]
    pub fn set_z_axis(&mut self, v: Vector) {
        unsafe { *self.columns.get_unchecked_mut(2) = v }
    }

    /// Get the w-axis of a matrix.
    #[inline]
    #[must_use]
    pub fn get_w_axis(&self) -> &Vector {
        unsafe { self.columns.get_unchecked(3) }
    }

    /// Set the x-axis of a matrix.
    #[inline]
    #[must_use]
    pub fn set_w_axis(&mut self, v: Vector) {
        unsafe { *self.columns.get_unchecked_mut(3) = v }
    }

    /// Transpose of a matrix.
    #[must_use]
    pub fn transpose(self) -> Self {
        // Origin:
        // m00 m01 m02 m03 
        // m10 m11 m12 m13
        // m20 m21 m22 m23
        // m30 m31 m32 m33
        // 
        unsafe {
            let tran_r0r1 = vtrnq_f32(self.inner.0, self.inner.1);
            let tran_r2r3 = vtrnq_f32(self.inner.2, self.inner.3);
            let m00_m10_m02_m12 = tran_r0r1.0;
            let m01_m11_m03_m13 = tran_r0r1.1;
            let m20_m30_m22_m32 = tran_r2r3.0;
            let m21_m31_m23_m33 = tran_r2r3.1;

            let m00_m10 = vget_low_f32(m00_m10_m02_m12);
            let m20_m30 = vget_low_f32(m20_m30_m22_m32);
            let m00_m10_m20_m30 = vcombine_f32(m00_m10, m20_m30);

            let m01_m11 = vget_low_f32(m01_m11_m03_m13);
            let m21_m31 = vget_low_f32(m21_m31_m23_m33);
            let m01_m11_m21_m31 = vcombine_f32(m01_m11, m21_m31);

            let m02_m12 = vget_high_f32(m00_m10_m02_m12);
            let m22_m32 = vget_high_f32(m20_m30_m22_m32);
            let m02_m12_m22_m32 = vcombine_f32(m02_m12, m22_m32);
            
            let m03_m13 = vget_high_f32(m01_m11_m03_m13);
            let m23_m33 = vget_high_f32(m21_m31_m23_m33);
            let m03_m13_m23_m33 = vcombine_f32(m03_m13, m23_m33);

            Self::from_columns(
                Vector { inner: m00_m10_m20_m30 }, 
                Vector { inner: m01_m11_m21_m31 }, 
                Vector { inner: m02_m12_m22_m32 }, 
                Vector { inner: m03_m13_m23_m33 }
            )
        }
    }

    /// Determinant of a matrix.
    #[must_use]
    pub fn determinant(self) -> Vector {
        // Reference: glm/detail/func_matrix.inl
        const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
        const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];
        unsafe {
            let m00_m01 = vget_low_f32(self.inner.0);
            let m02_m03 = vget_high_f32(self.inner.0);
            let m01_m00 = vext_f32::<0b01>(m00_m01, m00_m01);
            let m03_m02 = vext_f32::<0b01>(m02_m03, m02_m03);
            let m00_m00 = vext_f32::<0b01>(m01_m00, m00_m01);
            let m01_m01 = vext_f32::<0b01>(m00_m01, m01_m00);
            let m02_m02 = vext_f32::<0b01>(m03_m02, m02_m03);
            let m03_m03 = vext_f32::<0b01>(m02_m03, m03_m02);

            let m10_m11 = vget_low_f32(self.inner.1);
            let m12_m13 = vget_high_f32(self.inner.1);
            let m11_m10 = vext_f32::<0b01>(m10_m11, m10_m11);
            let m13_m12 = vext_f32::<0b01>(m12_m13, m12_m13);
            let m10_m10 = vext_f32::<0b01>(m11_m10, m10_m11);
            let m11_m11 = vext_f32::<0b01>(m10_m11, m11_m10);
            let m12_m12 = vext_f32::<0b01>(m13_m12, m12_m13);
            let m13_m13 = vext_f32::<0b01>(m12_m13, m13_m12);

            let m20_m21 = vget_low_f32(self.inner.2);
            let m22_m23 = vget_high_f32(self.inner.2);
            let m21_m20 = vext_f32::<0b01>(m20_m21, m20_m21);
            let m23_m22 = vext_f32::<0b01>(m22_m23, m22_m23);
            let m20_m20 = vext_f32::<0b01>(m21_m20, m20_m21);
            let m21_m21 = vext_f32::<0b01>(m20_m21, m21_m20);
            let m22_m22 = vext_f32::<0b01>(m23_m22, m22_m23);
            let m23_m23 = vext_f32::<0b01>(m22_m23, m23_m22);

            let m30_m31 = vget_low_f32(self.inner.3);
            let m32_m33 = vget_high_f32(self.inner.3);
            let m31_m30 = vext_f32::<0b01>(m30_m31, m30_m31);
            let m33_m32 = vext_f32::<0b01>(m32_m33, m32_m33);
            let m30_m30 = vext_f32::<0b01>(m31_m30, m30_m31);
            let m31_m31 = vext_f32::<0b01>(m30_m31, m31_m30);
            let m32_m32 = vext_f32::<0b01>(m33_m32, m32_m33);
            let m33_m33 = vext_f32::<0b01>(m32_m33, m33_m32);

            let m10_m00 = vext_f32::<0b01>(m11_m10, m00_m01);
            let m11_m01 = vext_f32::<0b01>(m10_m11, m01_m00);
            let m12_m02 = vext_f32::<0b01>(m13_m12, m02_m03);
            let m13_m03 = vext_f32::<0b01>(m12_m13, m03_m02);
            let m30_m20 = vext_f32::<0b01>(m31_m30, m20_m21);
            let m31_m21 = vext_f32::<0b01>(m30_m31, m21_m20);
            let m32_m22 = vext_f32::<0b01>(m33_m32, m22_m23);
            let m33_m23 = vext_f32::<0b01>(m32_m33, m23_m22);

            let m20_m20_m10_m10 = vcombine_f32(m20_m20, m10_m10);
            let m21_m21_m11_m11 = vcombine_f32(m21_m21, m11_m11);
            let m22_m22_m12_m12 = vcombine_f32(m22_m22, m12_m12);
            let m23_m23_m13_m13 = vcombine_f32(m23_m23, m13_m13);
            let m30_m30_m30_m20 = vcombine_f32(m30_m30, m30_m20);
            let m31_m31_m31_m21 = vcombine_f32(m31_m31, m31_m21);
            let m32_m32_m32_m22 = vcombine_f32(m32_m32, m32_m22);
            let m33_m33_m33_m23 = vcombine_f32(m33_m33, m33_m23);



            let a = vmulq_f32(m22_m22_m12_m12, m33_m33_m33_m23);
            let b = vmulq_f32(m32_m32_m32_m22, m23_m23_m13_m13);
            let fac0 = vsubq_f32(a, b);

            let a = vmulq_f32(m21_m21_m11_m11, m33_m33_m33_m23);
            let b = vmulq_f32(m31_m31_m31_m21, m23_m23_m13_m13);
            let fac1 = vsubq_f32(a, b);

            let a = vmulq_f32(m21_m21_m11_m11, m32_m32_m32_m22);
            let b = vmulq_f32(m31_m31_m31_m21, m22_m22_m12_m12);
            let fac2 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m33_m33_m33_m23);
            let b = vmulq_f32(m30_m30_m30_m20, m23_m23_m13_m13);
            let fac3 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m32_m32_m32_m22);
            let b = vmulq_f32(m30_m30_m30_m20, m22_m22_m12_m12);
            let fac4 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m31_m31_m31_m21);
            let b = vmulq_f32(m30_m30_m30_m20, m21_m21_m11_m11);
            let fac5 = vsubq_f32(a, b);

            let vec0 = vcombine_f32(m10_m00, m00_m00);
            let vec1 = vcombine_f32(m11_m01, m01_m01);
            let vec2 = vcombine_f32(m12_m02, m02_m02);
            let vec3 = vcombine_f32(m13_m03, m03_m03);

            let inv0 = vaddq_f32(vsubq_f32(vmulq_f32(vec1, fac0), vmulq_f32(vec2, fac1)), vmulq_f32(vec3, fac2));
            let inv1 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac0), vmulq_f32(vec2, fac3)), vmulq_f32(vec3, fac4));
            let inv2 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac1), vmulq_f32(vec1, fac3)), vmulq_f32(vec3, fac5));
            let inv3 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac2), vmulq_f32(vec1, fac4)), vmulq_f32(vec2, fac5));

            let one_neg_one_neg = vld1q_f32(&ONE_NEG_ONE_NEG as *const f32);
            let neg_one_neg_one = vld1q_f32(&NEG_ONE_NEG_ONE as *const f32);
            let inverse = [
                vmulq_f32(inv0, one_neg_one_neg), 
                vmulq_f32(inv1, neg_one_neg_one), 
                vmulq_f32(inv2, one_neg_one_neg), 
                vmulq_f32(inv3, neg_one_neg_one), 
            ];

            let inv00_inv10 = vget_low_f32(vtrnq_f32(inverse[0], inverse[1]).0); 
            let inv20_inv30 = vget_low_f32(vtrnq_f32(inverse[2], inverse[3]).0);
            let row0 = vcombine_f32(inv00_inv10, inv20_inv30);
            Vector { inner: vmulq_f32(self.inner.0, row0) }.sum()
        }
    }

    /// Determinant of a matrix.
    #[inline]
    #[must_use]
    pub fn determinant_into(self) -> f32 {
        self.determinant().get_x()
    }

    /// Inverse of a matrix.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the determinant of a matrix is less than or equal to [`f32::EPSILON`].
    /// 
    #[must_use]
    pub fn inverse(self) -> Self {
        // Reference: glm/detail/func_matrix.inl
        const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
        const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];
        unsafe {
            let m00_m01 = vget_low_f32(self.inner.0);
            let m02_m03 = vget_high_f32(self.inner.0);
            let m01_m00 = vext_f32::<0b01>(m00_m01, m00_m01);
            let m03_m02 = vext_f32::<0b01>(m02_m03, m02_m03);
            let m00_m00 = vext_f32::<0b01>(m01_m00, m00_m01);
            let m01_m01 = vext_f32::<0b01>(m00_m01, m01_m00);
            let m02_m02 = vext_f32::<0b01>(m03_m02, m02_m03);
            let m03_m03 = vext_f32::<0b01>(m02_m03, m03_m02);

            let m10_m11 = vget_low_f32(self.inner.1);
            let m12_m13 = vget_high_f32(self.inner.1);
            let m11_m10 = vext_f32::<0b01>(m10_m11, m10_m11);
            let m13_m12 = vext_f32::<0b01>(m12_m13, m12_m13);
            let m10_m10 = vext_f32::<0b01>(m11_m10, m10_m11);
            let m11_m11 = vext_f32::<0b01>(m10_m11, m11_m10);
            let m12_m12 = vext_f32::<0b01>(m13_m12, m12_m13);
            let m13_m13 = vext_f32::<0b01>(m12_m13, m13_m12);

            let m20_m21 = vget_low_f32(self.inner.2);
            let m22_m23 = vget_high_f32(self.inner.2);
            let m21_m20 = vext_f32::<0b01>(m20_m21, m20_m21);
            let m23_m22 = vext_f32::<0b01>(m22_m23, m22_m23);
            let m20_m20 = vext_f32::<0b01>(m21_m20, m20_m21);
            let m21_m21 = vext_f32::<0b01>(m20_m21, m21_m20);
            let m22_m22 = vext_f32::<0b01>(m23_m22, m22_m23);
            let m23_m23 = vext_f32::<0b01>(m22_m23, m23_m22);

            let m30_m31 = vget_low_f32(self.inner.3);
            let m32_m33 = vget_high_f32(self.inner.3);
            let m31_m30 = vext_f32::<0b01>(m30_m31, m30_m31);
            let m33_m32 = vext_f32::<0b01>(m32_m33, m32_m33);
            let m30_m30 = vext_f32::<0b01>(m31_m30, m30_m31);
            let m31_m31 = vext_f32::<0b01>(m30_m31, m31_m30);
            let m32_m32 = vext_f32::<0b01>(m33_m32, m32_m33);
            let m33_m33 = vext_f32::<0b01>(m32_m33, m33_m32);

            let m10_m00 = vext_f32::<0b01>(m11_m10, m00_m01);
            let m11_m01 = vext_f32::<0b01>(m10_m11, m01_m00);
            let m12_m02 = vext_f32::<0b01>(m13_m12, m02_m03);
            let m13_m03 = vext_f32::<0b01>(m12_m13, m03_m02);
            let m30_m20 = vext_f32::<0b01>(m31_m30, m20_m21);
            let m31_m21 = vext_f32::<0b01>(m30_m31, m21_m20);
            let m32_m22 = vext_f32::<0b01>(m33_m32, m22_m23);
            let m33_m23 = vext_f32::<0b01>(m32_m33, m23_m22);

            let m20_m20_m10_m10 = vcombine_f32(m20_m20, m10_m10);
            let m21_m21_m11_m11 = vcombine_f32(m21_m21, m11_m11);
            let m22_m22_m12_m12 = vcombine_f32(m22_m22, m12_m12);
            let m23_m23_m13_m13 = vcombine_f32(m23_m23, m13_m13);
            let m30_m30_m30_m20 = vcombine_f32(m30_m30, m30_m20);
            let m31_m31_m31_m21 = vcombine_f32(m31_m31, m31_m21);
            let m32_m32_m32_m22 = vcombine_f32(m32_m32, m32_m22);
            let m33_m33_m33_m23 = vcombine_f32(m33_m33, m33_m23);



            let a = vmulq_f32(m22_m22_m12_m12, m33_m33_m33_m23);
            let b = vmulq_f32(m32_m32_m32_m22, m23_m23_m13_m13);
            let fac0 = vsubq_f32(a, b);

            let a = vmulq_f32(m21_m21_m11_m11, m33_m33_m33_m23);
            let b = vmulq_f32(m31_m31_m31_m21, m23_m23_m13_m13);
            let fac1 = vsubq_f32(a, b);

            let a = vmulq_f32(m21_m21_m11_m11, m32_m32_m32_m22);
            let b = vmulq_f32(m31_m31_m31_m21, m22_m22_m12_m12);
            let fac2 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m33_m33_m33_m23);
            let b = vmulq_f32(m30_m30_m30_m20, m23_m23_m13_m13);
            let fac3 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m32_m32_m32_m22);
            let b = vmulq_f32(m30_m30_m30_m20, m22_m22_m12_m12);
            let fac4 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m31_m31_m31_m21);
            let b = vmulq_f32(m30_m30_m30_m20, m21_m21_m11_m11);
            let fac5 = vsubq_f32(a, b);

            let vec0 = vcombine_f32(m10_m00, m00_m00);
            let vec1 = vcombine_f32(m11_m01, m01_m01);
            let vec2 = vcombine_f32(m12_m02, m02_m02);
            let vec3 = vcombine_f32(m13_m03, m03_m03);

            let inv0 = vaddq_f32(vsubq_f32(vmulq_f32(vec1, fac0), vmulq_f32(vec2, fac1)), vmulq_f32(vec3, fac2));
            let inv1 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac0), vmulq_f32(vec2, fac3)), vmulq_f32(vec3, fac4));
            let inv2 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac1), vmulq_f32(vec1, fac3)), vmulq_f32(vec3, fac5));
            let inv3 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac2), vmulq_f32(vec1, fac4)), vmulq_f32(vec2, fac5));

            let one_neg_one_neg = vld1q_f32(&ONE_NEG_ONE_NEG as *const f32);
            let neg_one_neg_one = vld1q_f32(&NEG_ONE_NEG_ONE as *const f32);
            let inverse = [
                vmulq_f32(inv0, one_neg_one_neg), 
                vmulq_f32(inv1, neg_one_neg_one), 
                vmulq_f32(inv2, one_neg_one_neg), 
                vmulq_f32(inv3, neg_one_neg_one), 
            ];

            let inv00_inv10 = vget_low_f32(vtrnq_f32(inverse[0], inverse[1]).0); 
            let inv20_inv30 = vget_low_f32(vtrnq_f32(inverse[2], inverse[3]).0);
            let row0 = vcombine_f32(inv00_inv10, inv20_inv30);
            let det = vmulq_f32(self.inner.0, row0);
            let det = vpaddq_f32(det, det);
            let det = vpaddq_f32(det, det);
            let det = vgetq_lane_f32::<0b00>(det);

            let recip_det = det.recip();
            Self::from_columns(
                Vector { inner: vmulq_n_f32(inverse[0], recip_det) }, 
                Vector { inner: vmulq_n_f32(inverse[1], recip_det) }, 
                Vector { inner: vmulq_n_f32(inverse[2], recip_det) }, 
                Vector { inner: vmulq_n_f32(inverse[3], recip_det) } 
            )
        }
    }

    /// Inverse of a matrix.
    /// 
    /// Returns `None` if the determinant of a matrix is less than or equal to [`f32::EPSILON`].
    /// 
    pub fn try_inverse(self) -> Option<Self> {
        let det = self.determinant_into();
        if det <= f32::EPSILON {
            return None;
        }
        Some(self * det.recip())
    }
}

impl Default for Matrix {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
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
        self.store_float3x3()
    }
}

impl From<Float4x4> for Matrix {
    #[inline]
    fn from(value: Float4x4) -> Self {
        Self::load_float4x4(value)
    }
}

impl Into<Float4x4> for Matrix {
    #[inline]
    fn into(self) -> Float4x4 {
        self.store_float4x4()
    }
}

impl From<[f32; 16]> for Matrix {
    #[inline]
    fn from(value: [f32; 16]) -> Self {
        Self::from_column_array(value)
    }
}

impl Into<[f32; 16]> for Matrix {
    #[inline]
    fn into(self) -> [f32; 16] {
        self.into_column_array()
    }
}

impl ops::Add<Self> for Matrix {
    type Output = Self;
    /// Adds two matrices.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            columns: [
                unsafe { self.columns[0] + rhs.columns[0] }, 
                unsafe { self.columns[1] + rhs.columns[1] }, 
                unsafe { self.columns[2] + rhs.columns[2] }, 
                unsafe { self.columns[3] + rhs.columns[3] } 
            ]
        }
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
        Self {
            columns: [
                unsafe { self.columns[0] - rhs.columns[0] }, 
                unsafe { self.columns[1] - rhs.columns[1] }, 
                unsafe { self.columns[2] - rhs.columns[2] }, 
                unsafe { self.columns[3] - rhs.columns[3] } 
            ]
        }
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
        Self {
            columns: [
                unsafe { -self.columns[0] }, 
                unsafe { -self.columns[1] }, 
                unsafe { -self.columns[2] }, 
                unsafe { -self.columns[3] } 
            ]
        }
    }
}

impl ops::Mul<Matrix> for f32 {
    type Output = Matrix;
    /// Multiplies each element of a matrix by a scalar value.
    #[inline]
    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix {
            columns: [
                unsafe { self * rhs.columns[0] }, 
                unsafe { self * rhs.columns[1] }, 
                unsafe { self * rhs.columns[2] }, 
                unsafe { self * rhs.columns[3] } 
            ]
        }
    }
}

impl ops::Mul<f32> for Matrix {
    type Output = Self;
    /// Multiplies each element of a matrix by a scalar value.
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            columns: [
                unsafe { self.columns[0] * rhs }, 
                unsafe { self.columns[1] * rhs }, 
                unsafe { self.columns[2] * rhs }, 
                unsafe { self.columns[3] * rhs } 
            ]
        }
    }
}

impl ops::Mul<Vector> for Matrix {
    type Output = Vector;
    /// Transformation of the vector.
    fn mul(self, rhs: Vector) -> Self::Output {
        unsafe {
            let rows = self.transpose();

            let e0 = vmulq_f32(rows.inner.0, rhs.inner);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows.inner.1, rhs.inner);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows.inner.2, rhs.inner);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows.inner.3, rhs.inner);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col0 = vextq_f32::<0b10>(tran0, tran1);

            return Vector{ inner: col0 };
        }
    }
}

impl ops::Mul<Self> for Matrix {
    type Output = Self;
    /// Multiplies two matrices.
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            let rows = self.transpose();

            let e0 = vmulq_f32(rows.inner.0, rhs.inner.0);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows.inner.1, rhs.inner.0);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows.inner.2, rhs.inner.0);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows.inner.3, rhs.inner.0);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col0 = Vector { inner: vextq_f32::<0b10>(tran0, tran1) };


            let e0 = vmulq_f32(rows.inner.0, rhs.inner.1);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows.inner.1, rhs.inner.1);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows.inner.2, rhs.inner.1);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows.inner.3, rhs.inner.1);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col1 = Vector { inner: vextq_f32::<0b10>(tran0, tran1) };


            let e0 = vmulq_f32(rows.inner.0, rhs.inner.2);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows.inner.1, rhs.inner.2);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows.inner.2, rhs.inner.2);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows.inner.3, rhs.inner.2);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col2 = Vector { inner: vextq_f32::<0b10>(tran0, tran1) };


            let e0 = vmulq_f32(rows.inner.0, rhs.inner.3);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows.inner.1, rhs.inner.3);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows.inner.2, rhs.inner.3);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows.inner.3, rhs.inner.3);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col3 = Vector { inner: vextq_f32::<0b10>(tran0, tran1) };
            
            Self::from_columns(col0, col1, col2, col3)
        }
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
            .field(unsafe { &self.columns })
            .finish()
    }
}
