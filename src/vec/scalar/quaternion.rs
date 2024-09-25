use core::fmt;
use core::ops;
use crate::{ Matrix, Vector, VectorInt, Float3, Float4 };



/// This is a quaternion data type that uses the `Scalar` instruction.
/// 
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Quaternion {
    pub(crate) arr: [f32; 4], 
}

impl Quaternion {
    /// All elements are zeros.
    pub const ZERO: Self = Self { arr: [0.0; 4] };

    /// Identity quaternion.
    pub const IDENTITY: Self = Self { arr: [0.0, 0.0, 0.0, 1.0] };
}

impl Quaternion {
    /// Creates with given elements.
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { arr: [x, y, z, w] }
    }

    /// Fills all elements with the given values.
    #[inline]
    #[must_use]
    pub fn fill(v: f32) -> Self {
        Self { arr: [v; 4] }
    }

    /// Creates from a given array.
    #[inline]
    #[must_use]
    pub fn from_array(arr: [f32; 4]) -> Self {
        Self { arr }
    }

    /// Stores the value in an array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> [f32; 4] {
        self.arr
    }

    /// Creates from a given array of slice.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the array slice has less than four elements.
    /// 
    #[inline]
    #[must_use]
    pub fn from_slice(slice: &[f32]) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(slice.len() >= 4, "The given array slice has less than four elements!");
        let mut arr = [0.0; 4];
        for (i, e) in arr.iter_mut().enumerate() {
            *e = slice[i]
        }
        Self { arr }
    }

    /// Loads a value from a given `Float4`.
    #[inline]
    #[must_use]
    pub fn load_float4(val: Float4) -> Self {
        Self { arr: val.to_array() }
    }

    /// Stores the value in a `Float4`.
    #[inline]
    #[must_use]
    pub fn store_float4(self) -> Float4 {
        Float4::from_array(self.into_array())
    }
    
    /// Creates a quaternion rotated by a given x-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = (0.5 * angle).sin_cos();
        Self::new(s, 0.0, 0.0, c)
    }

    /// Creates a quaternion rotated by a given y-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = (0.5 * angle).sin_cos();
        Self::new(0.0, s, 0.0, c)
    }

    /// Creates a quaternion rotated by a given z-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = (0.5 * angle).sin_cos();
        Self::new(0.0, 0.0, s, c)
    }

    /// Creates a quaternion rotated about a given `axis` by a given `angle`.
    /// 
    /// ※ The angles given are in radians. </br>
    /// ※ The given axis must be a unit vector. </br>
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, [`panic!`] will be called
    /// if the given axis is not a unit vector.
    /// 
    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: Vector, angle: f32) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(axis.is_vec3_normalized(), "The given axis must be a unit vector!");

        let (s, c) = (0.5 * angle).sin_cos();
        let mut v = axis * s;
        v.set_w(c);
        v.into()
    }

    /// Create a quaternion from each axis.
    /// 
    /// ※ Each axis must be a unit vector. 
    /// 
    /// # Panics
    /// If `use-assertion` is enabled
    /// and the given axis is not a unit vector, it will call [`panic!`].
    /// 
    fn from_rotation_axes(x_axis: Vector, y_axis: Vector, z_axis: Vector) -> Self {
        #[cfg(feature = "use-assertion")] {
            let validate = x_axis.is_vec3_normalized() 
            & y_axis.is_vec3_normalized();
            & z_axis.is_vec3_normalized();
            assert!(validate, "The given axis must be a unit vector!");
        }
        let x_axis: Float3 = x_axis.into();
        let y_axis: Float3 = y_axis.into();
        let z_axis: Float3 = z_axis.into();

        // Reference: DirectXMath/Inc/DirectXMathMise.inl
        let (m00, m01, m02) = x_axis.into();
        let (m10, m11, m12) = y_axis.into();
        let (m20, m21, m22) = z_axis.into();
        if m22 <= 0.0 {
            let dif10  = m11 - m00;
            let omr22 = 1.0 - m22;
            if dif10 <= 0.0 {
                let four_x_sqr = omr22 - dif10;
                let inv4x = 0.5 / four_x_sqr.sqrt();
                return Float4 {
                    x: four_x_sqr * inv4x, 
                    y: (m01 + m10) * inv4x, 
                    z: (m02 + m20) * inv4x, 
                    w: (m12 - m21) * inv4x,
                }.into();
            } else {
                let four_y_sqr = omr22 + dif10;
                let inv4y = 0.5 / four_y_sqr.sqrt();
                return Float4 {
                    x: (m01 + m10) * inv4y, 
                    y: four_y_sqr * inv4y, 
                    z: (m12 + m21) * inv4y, 
                    w: (m20 - m02) * inv4y, 
                }.into();
            }
        } else {
            let sum10 = m11 + m00;
            let opr22 = 1.0 + m22;
            if sum10 <= 0.0 {
                let four_z_sqr = opr22 - sum10;
                let inv4z = 0.5 / four_z_sqr.sqrt();
                return Float4 {
                    x: (m02 + m20) * inv4z, 
                    y: (m12 + m21) * inv4z, 
                    z: four_z_sqr * inv4z, 
                    w: (m01 - m10) * inv4z, 
                }.into();
            } else {
                let four_w_sqr = opr22 + sum10;
                let inv4w = 0.5 / four_w_sqr.sqrt();
                return Float4 {
                    x: (m12 - m21) * inv4w, 
                    y: (m20 - m02) * inv4w, 
                    z: (m01 - m10) * inv4w, 
                    w: four_w_sqr * inv4w, 
                }.into();
            }
        }
    }

    /// Convert quaternions to each axis.
    /// 
    /// ※ The quaternion must be a normalized quaternion.
    /// 
    /// # Panics
    /// If `use-assertion` is enabled
    /// and the given quaternion is not a normalized quaternion, it will call [`panic!`].
    /// 
    pub fn to_rotation_axes(self) -> (Vector, Vector, Vector) {
        #[cfg(feature = "use-assertion")]
        assert!(self.is_normalize(), "The quaternion must be normalized!");

        let quat: Float4 = self.into();
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
    
        let x_axis = Float4 { x: 1.0 - (yy + zz), y: xy + wz, z: xz - wy, w: 0.0 }.into();
        let y_axis = Float4 { x: xy - wz, y: 1.0 - (xx + zz), z: yz + wx, w: 0.0 }.into();
        let z_axis = Float4 { x: xz + wy, y: yz - wx, z: 1.0 - (xx + yy), w: 0.0 }.into();
        
        (x_axis, y_axis, z_axis)
    }

    /// Creates from a given matrix.
    /// 
    /// # Panics
    /// When `use-assertion` feature is enabled, [`panic!`] will be called 
    /// if the length of each axis of the matrix is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn from_matrix(matrix: Matrix) -> Quaternion {
        Self::from_rotation_axes(
            matrix.get_x_axis().vec3_normalize(), 
            matrix.get_y_axis().vec3_normalize(), 
            matrix.get_z_axis().vec3_normalize() 
        )
    }

    /// Creates from a given matrix.
    /// 
    /// Returns `None` if the length of each axis of the matrix is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn try_from_matrix(matrix: Matrix) -> Option<Quaternion> {
        matrix.get_x_axis().try_vec3_normalize()
            .map(|x_axis| matrix.get_y_axis().try_vec3_normalize()
                .map(|y_axis| matrix.get_z_axis().try_vec3_normalize()
                    .map(|z_axis| Self::from_rotation_axes(x_axis, y_axis, z_axis))
                ).flatten()
            ).flatten()
    }

    /// Stores the value in a matrix.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, [`panic!`] will be called 
    /// if the quaternion is not a normalized quaternion.
    /// 
    #[inline]
    #[must_use]
    pub fn into_matrix(self) -> Matrix {
        let (x_axis, y_axis, z_axis) = self.to_rotation_axes();
        Matrix::from_columns(x_axis, y_axis, z_axis, Vector::W)
    }

    /// Stores the value in a matrix.
    /// 
    /// Returns `None` if the quaternion is not a normalized quaternion.
    /// 
    pub fn try_into_matrix(self) -> Option<Matrix> {
        if !self.is_normalized() {
            return None;
        }
        Some(self.into_matrix())
    }
}

impl Quaternion {
    /// Get the `x` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_x(&self) -> f32 {
        self.arr[0]
    }

    /// Set the `x` element of a vector.
    #[inline]
    pub fn set_x(&mut self, v: f32) {
        self.arr[0] = v
    }

    /// Get the `y element of a vector.
    #[inline]
    #[must_use]
    pub fn get_y(&self) -> f32 {
        self.arr[1]
    }

    /// Set the `y` element of a vector.
    #[inline]
    pub fn set_y(&mut self, v: f32) {
        self.arr[1] = v
    }

    /// Get the `z` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_z(&self) -> f32 {
        self.arr[2]
    }

    /// Set the `z` element of a vector.
    #[inline]
    pub fn set_z(&mut self, v: f32) {
        self.arr[2] = v
    }

    /// Get the `w` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_w(&self) -> f32 {
        self.arr[3]
    }

    /// Set the `w` element of a vector.
    #[inline]
    pub fn set_w(&mut self, v: f32) {
        self.arr[3] = v
    }

    /// Checks if the elements of two vectors are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn eq(self, rhs: Self) -> VectorInt {
        VectorInt::new(
            self.get_x().eq(&rhs.get_x()).then(|| -1).unwrap_or(0), 
            self.get_y().eq(&rhs.get_y()).then(|| -1).unwrap_or(0), 
            self.get_z().eq(&rhs.get_z()).then(|| -1).unwrap_or(0), 
            self.get_w().eq(&rhs.get_w()).then(|| -1).unwrap_or(0) 
        )
    }

    /// Checks if the elements of two vectors are not eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn ne(self, rhs: Self) -> VectorInt {
        !self.eq(rhs)
    }
    
    /// Returns a vector filled with the dot products of the quaternions.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> Vector {
        let a = Vector::from_array(self.into_array());
        let b = Vector::from_array(rhs.into_array());
        a.vec4_dot(b)
    }

    /// Dot product of the quaternions.
    #[inline]
    #[must_use]
    pub fn dot_into(self, rhs: Self) -> f32 {
        self.dot(rhs).get_x()
    }

    /// Length squared of a quaternion.
    #[inline]
    #[must_use]
    pub fn len_sq(self) -> f32 {
        self.dot_into(self)
    }

    /// Length of a quaternion.
    #[inline]
    #[must_use]
    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
    }

    /// Returns `true` if it is a normalized quaternion.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        (self.len() - 1.0).abs() <= f32::EPSILON 
    }

    /// Normalizes a quaternion.
    /// 
    /// Undefined behavior may occur if the length of the quaternion is less than or equal to [`f32::EPSILON`].
    /// 
    /// # Panics
    /// When `use-assertion` is enabled, [`panic!`] will be called 
    /// if the length of the quaternion is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(self.len() <= f32::EPSILON, "The length of the vector is less than or equal to `f32::EPSILON`!");
        self * self.len().recip()
    }

    /// Normalizes a quaternion. 
    /// 
    /// Returns `None` if the length of the quaternion is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let length = self.len();
        if length <= f32::EPSILON {
            return None;
        }
        Some(self * length.recip())
    }

    /// Returns the conjugate of the quaternion.
    #[inline]
    #[must_use]
    pub fn conjugate(self) -> Self {
        Self { arr: [
            self.get_x() * -1.0, 
            self.get_y() * -1.0, 
            self.get_z() * -1.0, 
            self.get_w() * 1.0, 
        ] }
    }

    /// Returns the inverse of the quaternion.
    /// 
    /// Undefined behavior may occur if the length of the quaternion is less than or equal to [`f32::EPSILON`].
    /// 
    /// # Panics
    /// When `use-assertion` feature is enabled, [`panic!`] will be called
    /// if the length of the quaternion is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn inverse(self) -> Self {
        self.normalize().conjugate()
    }

    /// Returns the inverse of the quaternion.
    /// 
    /// If the quaternion cannot be normalized, `None` is returned.
    /// 
    #[inline]
    #[must_use]
    pub fn try_inverse(self) -> Option<Self> {
        self.try_normalize().map(|q| q.conjugate())
    }

    /// Returns a quaternion that is a linear interpolation of two quaternion.
    /// 
    /// The given `t` must be in the range zero to one.
    ///  
    /// The closer `t` is to one, the more it becomes equal to the given `rhs`.
    /// 
    /// # Panics
    /// When `use-assertion` feature is enabled, [`panic!`] will be called 
    /// if the quaternion is not normalized.
    /// 
    /// 
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        self * (1.0 - t) + rhs * t
    }
}

impl Default for Quaternion {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<[f32; 4]> for Quaternion {
    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self::from_array(value)
    }
}

impl Into<[f32; 4]> for Quaternion {
    #[inline]
    fn into(self) -> [f32; 4] {
        self.into_array()
    }
}

impl From<Vector> for Quaternion {
    #[inline]
    fn from(value: Vector) -> Self {
        Quaternion::from_array(value.into_array())
    }
}

impl Into<Vector> for Quaternion {
    #[inline]
    fn into(self) -> Vector {
        Vector::from_array(self.into_array())
    }
}

impl From<Float4> for Quaternion {
    #[inline]
    fn from(value: Float4) -> Self {
        Self::load_float4(value)
    }
}

impl Into<Float4> for Quaternion {
    #[inline]
    fn into(self) -> Float4 {
        self.store_float4()
    }
}

impl ops::Add<Self> for Quaternion {
    type Output = Self;
    /// Element-wise addition of two quaternions.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { arr: [
            self.get_x() + rhs.get_x(), 
            self.get_y() + rhs.get_y(), 
            self.get_z() + rhs.get_z(), 
            self.get_w() + rhs.get_w() 
        ] }
    }
}

impl ops::AddAssign<Self> for Quaternion {
    /// Element-wise addition of two quaternions.
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Mul<Quaternion> for f32 {
    type Output = Quaternion;
    /// Scalar multiplication of a quaternion.
    #[inline]
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion { arr: [
            self * rhs.get_x(), 
            self * rhs.get_y(), 
            self * rhs.get_z(), 
            self * rhs.get_w() 
        ] }
    }
}

impl ops::Mul<f32> for Quaternion {
    type Output = Self;
    /// Scalar multiplication of a quaternion.
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self { arr: [
            self.get_x() * rhs, 
            self.get_y() * rhs, 
            self.get_z() * rhs, 
            self.get_w() * rhs 
        ] }
    }
}

impl ops::Mul<Self> for Quaternion {
    type Output = Self;
    /// Multiplies two quaternions.
    fn mul(self, rhs: Self) -> Self::Output {
        // self: a, rhs: b
        // i: aw*bx + ax*bw + ay*bz - az*by
        // j: aw*by - ax*bz + ay*bw + az*bx
        // k: aw*bz + ax*by - ay*bx + az*bw
        // w: aw*bw - ax*bx - ay*by - az*bz
        //
        Self { arr: [
            self.get_w() * rhs.get_x() + self.get_x() * rhs.get_w() + self.get_y() * rhs.get_z() - self.get_z() * rhs.get_y(), 
            self.get_w() * rhs.get_y() - self.get_x() * rhs.get_z() + self.get_y() * rhs.get_w() + self.get_z() * rhs.get_x(), 
            self.get_w() * rhs.get_z() + self.get_x() * rhs.get_y() - self.get_y() * rhs.get_x() + self.get_z() * rhs.get_w(), 
            self.get_w() * rhs.get_w() - self.get_x() * rhs.get_x() - self.get_y() * rhs.get_y() - self.get_z() * rhs.get_z()
        ] }
    }
}

impl ops::MulAssign<Self> for Quaternion {
    /// Multiplies two quaternions. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl fmt::Debug for Quaternion {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Quaternion))
            .field(&self.arr)
            .finish()
    }
}
