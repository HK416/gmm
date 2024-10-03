use core::fmt;
use core::ops;
use crate::{ Vector, Quaternion, Float3x3, Float4x4 };



/// This is a matrix data type that uses the `Scalar` instruction.
/// 
#[repr(C)]
#[derive(Clone, Copy)]
pub union Matrix {
    /// member variables for constant variables.
    arr: [f32; 16], 

    pub(crate) columns: [Vector; 4], 
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
        Self { columns: [
            Vector::from_slice(&arr[0..4]), 
            Vector::from_slice(&arr[4..8]), 
            Vector::from_slice(&arr[8..12]), 
            Vector::from_slice(&arr[12..16]) 
        ] }
    }

    /// Stores the value in an array.
    #[inline]
    #[must_use]
    pub fn into_column_array(self) -> [f32; 16] {
        let mut arr = [0.0; 16];
        unsafe { 
            let mut iter = self.columns.into_iter()
                .map(|v| v.into_array())
                .flatten();
            for e in arr.iter_mut() {
                *e = iter.next().unwrap_unchecked()
            }
        }
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
        Self { columns: [
            Vector::from_slice(&slice[0..4]), 
            Vector::from_slice(&slice[4..8]), 
            Vector::from_slice(&slice[8..12]), 
            Vector::from_slice(&slice[12..16]) 
        ] }
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

    /// Creates a matrix from a given quaternion.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, [`panic!`] will be called 
    /// if the quaternion is not a normalized quaternion.
    /// 
    #[inline]
    #[must_use]
    pub fn from_quaternion(q: Quaternion) -> Self {
        q.into_matrix()
    }

    /// Creates a matrix from a given quaternion.
    /// 
    /// If the quaternion is not normalized, `None` is returned.
    /// 
    #[inline]
    #[must_use]
    pub fn try_from_quaternion(q: Quaternion) -> Option<Self> {
        q.try_into_matrix()
    }

    /// Converts a matrix to a quaternion.
    /// 
    /// # Panics
    /// When `use-assertion` feature is enabled, [`panic!`] will be called 
    /// if the length of each axis of the matrix is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn into_quaternion(self) -> Quaternion {
        Quaternion::from_matrix(self)
    }

    /// Converts a matrix to a quaternion.
    /// 
    /// Returns `None` if the length of each axis of the matrix is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn try_into_quaternion(self) -> Option<Quaternion> {
        Quaternion::try_from_matrix(self)
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
        Self { arr: [
            self.get_x_axis().get_x(), self.get_y_axis().get_x(), self.get_z_axis().get_x(), self.get_w_axis().get_x(), 
            self.get_x_axis().get_y(), self.get_y_axis().get_y(), self.get_z_axis().get_y(), self.get_w_axis().get_y(), 
            self.get_x_axis().get_z(), self.get_y_axis().get_z(), self.get_z_axis().get_z(), self.get_w_axis().get_z(), 
            self.get_x_axis().get_w(), self.get_y_axis().get_w(), self.get_z_axis().get_w(), self.get_w_axis().get_w()
        ] }
    }

    /// Determinant of a matrix.
    #[must_use]
    pub fn determinant(self) -> Vector {
        // Reference: glm/detail/func_matrix.inl
        let fac0 = Vector::new(
            self.get_z_axis().get_z() * self.get_w_axis().get_w() - self.get_w_axis().get_z() * self.get_z_axis().get_w(), 
            self.get_z_axis().get_z() * self.get_w_axis().get_w() - self.get_w_axis().get_z() * self.get_z_axis().get_w(), 
            self.get_y_axis().get_z() * self.get_w_axis().get_w() - self.get_w_axis().get_z() * self.get_y_axis().get_w(), 
            self.get_y_axis().get_z() * self.get_z_axis().get_w() - self.get_z_axis().get_z() * self.get_y_axis().get_w()
        );
        let fac1 = Vector::new(
            self.get_z_axis().get_y() * self.get_w_axis().get_w() - self.get_w_axis().get_y() * self.get_z_axis().get_w(), 
            self.get_z_axis().get_y() * self.get_w_axis().get_w() - self.get_w_axis().get_y() * self.get_z_axis().get_w(), 
            self.get_y_axis().get_y() * self.get_w_axis().get_w() - self.get_w_axis().get_y() * self.get_y_axis().get_w(), 
            self.get_y_axis().get_y() * self.get_z_axis().get_w() - self.get_z_axis().get_y() * self.get_y_axis().get_w()
        );
        let fac2 = Vector::new(
            self.get_z_axis().get_y() * self.get_w_axis().get_z() - self.get_w_axis().get_y() * self.get_z_axis().get_z(), 
            self.get_z_axis().get_y() * self.get_w_axis().get_z() - self.get_w_axis().get_y() * self.get_z_axis().get_z(), 
            self.get_y_axis().get_y() * self.get_w_axis().get_z() - self.get_w_axis().get_y() * self.get_y_axis().get_z(), 
            self.get_y_axis().get_y() * self.get_z_axis().get_z() - self.get_z_axis().get_y() * self.get_y_axis().get_z()
        );
        let fac3 = Vector::new(
            self.get_z_axis().get_x() * self.get_w_axis().get_w() - self.get_w_axis().get_x() * self.get_z_axis().get_w(), 
            self.get_z_axis().get_x() * self.get_w_axis().get_w() - self.get_w_axis().get_x() * self.get_z_axis().get_w(), 
            self.get_y_axis().get_x() * self.get_w_axis().get_w() - self.get_w_axis().get_x() * self.get_y_axis().get_w(), 
            self.get_y_axis().get_x() * self.get_z_axis().get_w() - self.get_z_axis().get_x() * self.get_y_axis().get_w(), 
        );
        let fac4 = Vector::new(
            self.get_z_axis().get_x() * self.get_w_axis().get_z() - self.get_w_axis().get_x() * self.get_z_axis().get_z(), 
            self.get_z_axis().get_x() * self.get_w_axis().get_z() - self.get_w_axis().get_x() * self.get_z_axis().get_z(), 
            self.get_y_axis().get_x() * self.get_w_axis().get_z() - self.get_w_axis().get_x() * self.get_y_axis().get_z(), 
            self.get_y_axis().get_x() * self.get_z_axis().get_z() - self.get_z_axis().get_x() * self.get_y_axis().get_z()
        );
        let fac5 = Vector::new(
            self.get_z_axis().get_x() * self.get_w_axis().get_y() - self.get_w_axis().get_x() * self.get_z_axis().get_y(), 
            self.get_z_axis().get_x() * self.get_w_axis().get_y() - self.get_w_axis().get_x() * self.get_z_axis().get_y(), 
            self.get_y_axis().get_x() * self.get_w_axis().get_y() - self.get_w_axis().get_x() * self.get_y_axis().get_y(), 
            self.get_y_axis().get_x() * self.get_z_axis().get_y() - self.get_z_axis().get_x() * self.get_y_axis().get_y()
        );


        let vec0 = Vector::new(self.get_y_axis().get_x(), self.get_x_axis().get_x(), self.get_x_axis().get_x(), self.get_x_axis().get_x());
        let vec1 = Vector::new(self.get_y_axis().get_y(), self.get_x_axis().get_y(), self.get_x_axis().get_y(), self.get_x_axis().get_y());
        let vec2 = Vector::new(self.get_y_axis().get_z(), self.get_x_axis().get_z(), self.get_x_axis().get_z(), self.get_x_axis().get_z());
        let vec3 = Vector::new(self.get_y_axis().get_w(), self.get_x_axis().get_w(), self.get_x_axis().get_w(), self.get_x_axis().get_w());

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let one_neg_one_neg = Vector::new(1.0, -1.0, 1.0, -1.0);
        let neg_one_neg_one = Vector::new(-1.0, 1.0, -1.0, 1.0);
        let inverse = Matrix::from_columns(
            inv0 * one_neg_one_neg, 
            inv1 * neg_one_neg_one, 
            inv2 * one_neg_one_neg, 
            inv3 * neg_one_neg_one
        );

        let row0 = Vector::new(inverse.get_x_axis().get_x(), inverse.get_y_axis().get_x(), inverse.get_z_axis().get_x(), inverse.get_w_axis().get_x());
        let det = *self.get_x_axis() * row0;
        let det = det.get_x() + det.get_y() + det.get_z() + det.get_w();
        return Vector::fill(det);
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
        let fac0 = Vector::new(
            self.get_z_axis().get_z() * self.get_w_axis().get_w() - self.get_w_axis().get_z() * self.get_z_axis().get_w(), 
            self.get_z_axis().get_z() * self.get_w_axis().get_w() - self.get_w_axis().get_z() * self.get_z_axis().get_w(), 
            self.get_y_axis().get_z() * self.get_w_axis().get_w() - self.get_w_axis().get_z() * self.get_y_axis().get_w(), 
            self.get_y_axis().get_z() * self.get_z_axis().get_w() - self.get_z_axis().get_z() * self.get_y_axis().get_w()
        );
        let fac1 = Vector::new(
            self.get_z_axis().get_y() * self.get_w_axis().get_w() - self.get_w_axis().get_y() * self.get_z_axis().get_w(), 
            self.get_z_axis().get_y() * self.get_w_axis().get_w() - self.get_w_axis().get_y() * self.get_z_axis().get_w(), 
            self.get_y_axis().get_y() * self.get_w_axis().get_w() - self.get_w_axis().get_y() * self.get_y_axis().get_w(), 
            self.get_y_axis().get_y() * self.get_z_axis().get_w() - self.get_z_axis().get_y() * self.get_y_axis().get_w()
        );
        let fac2 = Vector::new(
            self.get_z_axis().get_y() * self.get_w_axis().get_z() - self.get_w_axis().get_y() * self.get_z_axis().get_z(), 
            self.get_z_axis().get_y() * self.get_w_axis().get_z() - self.get_w_axis().get_y() * self.get_z_axis().get_z(), 
            self.get_y_axis().get_y() * self.get_w_axis().get_z() - self.get_w_axis().get_y() * self.get_y_axis().get_z(), 
            self.get_y_axis().get_y() * self.get_z_axis().get_z() - self.get_z_axis().get_y() * self.get_y_axis().get_z()
        );
        let fac3 = Vector::new(
            self.get_z_axis().get_x() * self.get_w_axis().get_w() - self.get_w_axis().get_x() * self.get_z_axis().get_w(), 
            self.get_z_axis().get_x() * self.get_w_axis().get_w() - self.get_w_axis().get_x() * self.get_z_axis().get_w(), 
            self.get_y_axis().get_x() * self.get_w_axis().get_w() - self.get_w_axis().get_x() * self.get_y_axis().get_w(), 
            self.get_y_axis().get_x() * self.get_z_axis().get_w() - self.get_z_axis().get_x() * self.get_y_axis().get_w(), 
        );
        let fac4 = Vector::new(
            self.get_z_axis().get_x() * self.get_w_axis().get_z() - self.get_w_axis().get_x() * self.get_z_axis().get_z(), 
            self.get_z_axis().get_x() * self.get_w_axis().get_z() - self.get_w_axis().get_x() * self.get_z_axis().get_z(), 
            self.get_y_axis().get_x() * self.get_w_axis().get_z() - self.get_w_axis().get_x() * self.get_y_axis().get_z(), 
            self.get_y_axis().get_x() * self.get_z_axis().get_z() - self.get_z_axis().get_x() * self.get_y_axis().get_z()
        );
        let fac5 = Vector::new(
            self.get_z_axis().get_x() * self.get_w_axis().get_y() - self.get_w_axis().get_x() * self.get_z_axis().get_y(), 
            self.get_z_axis().get_x() * self.get_w_axis().get_y() - self.get_w_axis().get_x() * self.get_z_axis().get_y(), 
            self.get_y_axis().get_x() * self.get_w_axis().get_y() - self.get_w_axis().get_x() * self.get_y_axis().get_y(), 
            self.get_y_axis().get_x() * self.get_z_axis().get_y() - self.get_z_axis().get_x() * self.get_y_axis().get_y()
        );


        let vec0 = Vector::new(self.get_y_axis().get_x(), self.get_x_axis().get_x(), self.get_x_axis().get_x(), self.get_x_axis().get_x());
        let vec1 = Vector::new(self.get_y_axis().get_y(), self.get_x_axis().get_y(), self.get_x_axis().get_y(), self.get_x_axis().get_y());
        let vec2 = Vector::new(self.get_y_axis().get_z(), self.get_x_axis().get_z(), self.get_x_axis().get_z(), self.get_x_axis().get_z());
        let vec3 = Vector::new(self.get_y_axis().get_w(), self.get_x_axis().get_w(), self.get_x_axis().get_w(), self.get_x_axis().get_w());

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let one_neg_one_neg = Vector::new(1.0, -1.0, 1.0, -1.0);
        let neg_one_neg_one = Vector::new(-1.0, 1.0, -1.0, 1.0);
        let inverse = Matrix::from_columns(
            inv0 * one_neg_one_neg, 
            inv1 * neg_one_neg_one, 
            inv2 * one_neg_one_neg, 
            inv3 * neg_one_neg_one
        );

        let row0 = Vector::new(inverse.get_x_axis().get_x(), inverse.get_y_axis().get_x(), inverse.get_z_axis().get_x(), inverse.get_w_axis().get_x());
        let det = *self.get_x_axis() * row0;
        let det = det.get_x() + det.get_y() + det.get_z() + det.get_w();
        return inverse * det.recip();
    }

    /// Inverse of a matrix.
    /// 
    /// Returns `None` if the determinant of a matrix is less than or equal to [`f32::EPSILON`].
    /// 
    #[must_use]
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
        Vector::new(
            self.get_x_axis().get_x() * rhs.get_x() + self.get_x_axis().get_y() * rhs.get_y() + self.get_x_axis().get_z() * rhs.get_z() + self.get_x_axis().get_w() * rhs.get_w(), 
            self.get_y_axis().get_x() * rhs.get_x() + self.get_y_axis().get_y() * rhs.get_y() + self.get_y_axis().get_z() * rhs.get_z() + self.get_y_axis().get_w() * rhs.get_w(), 
            self.get_z_axis().get_x() * rhs.get_x() + self.get_z_axis().get_y() * rhs.get_y() + self.get_z_axis().get_z() * rhs.get_z() + self.get_z_axis().get_w() * rhs.get_w(), 
            self.get_w_axis().get_x() * rhs.get_x() + self.get_w_axis().get_y() * rhs.get_y() + self.get_w_axis().get_z() * rhs.get_z() + self.get_w_axis().get_w() * rhs.get_w() 
        )
    }
}

impl ops::Mul<Self> for Matrix {
    type Output = Self;
    /// Multiplies two matrices.
    fn mul(self, rhs: Self) -> Self::Output {
        let mut value = [0.0; 16];
        let lhs = self.into_column_array();
        let rhs = rhs.into_column_array();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += lhs[4 * k + j] * rhs[4 * i + k];
                }
                value[4 * i + j] = sum;
            }
        }
        return Matrix::from_column_array(value);
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
