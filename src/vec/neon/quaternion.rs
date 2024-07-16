use core::fmt;
use core::ops;
use core::arch::aarch64::*;
use crate::{ Vector, Matrix, VectorInt, Float3, Float4, Float4x4 };



/// This is a quaternion data type that uses the `SIMD` instruction.
/// 
/// Using the `scalar-math` feature disables the use of `SIMD` instructions.
/// 
/// It is recommended not to use this data types as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Quaternion(pub(crate) float32x4_t);

impl Quaternion {
    /// Creates a quaternion rotated by a given x-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = (0.5 * angle).sin_cos();
        Float4 { x: s, y: 0.0, z: 0.0, w: c }.into()
    }

    /// Creates a quaternion rotated by a given y-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = (0.5 * angle).sin_cos();
        Float4 { x: 0.0, y: s, z: 0.0, w: c }.into()
    }

    /// Creates a quaternion rotated by a given z-axis angle.
    /// 
    /// ※ The angles given are in radians.
    /// 
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = (0.5 * angle).sin_cos();
        Float4 { x: 0.0, y: 0.0, z: s, w: c }.into()
    }

    /// Creates a quaternion rotated about a given `axis` by a given `angle`.
    /// 
    /// ※ The angles given are in radians. </br>
    /// ※ The given axis must be a unit vector. </br>
    /// 
    /// # Panics
    /// If `use-assertion` is enabled
    /// and the given axis is not a unit vector, it will call [`panic!`].
    /// 
    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: Vector, angle: f32) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(axis.is_vec3_normalized(), "The given axis must be a unit vector!");

        let (s, c) = (0.5 * angle).sin_cos();
        unsafe {
            let mut v = vmulq_n_f32(*axis, s);
            v = vsetq_lane_f32::<0b11>(c, v);
            return Quaternion(v);
        }
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
    fn to_rotation_axes(self) -> (Vector, Vector, Vector) {
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
}

impl Quaternion {
    /// Checks if the elements of two quaternions are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn eq(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vceqq_f32(*self, *rhs);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt(cast);
        }
    }

    /// Checks if the elements of two quaternions are not eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn ne(self, rhs: Self) -> VectorInt {
        !self.eq(rhs)
    }
}

impl Quaternion {
    /// Dot product of two quaternions.
    #[inline]
    pub fn dot(self, rhs: Self) -> Vector {
        unsafe {
            let mul = vmulq_f32(*self, *rhs);
            let sum = vpaddq_f32(mul, mul);
            let sum = vpaddq_f32(sum, sum);
            return Vector(sum);
        }
    }

    /// Length squared of a quaternion.
    #[inline]
    pub fn len_sq(self) -> f32 {
        unsafe { vgetq_lane_f32::<0b00>(*self.dot(self)) }
    }

    /// Length of a quaternion.
    #[inline]
    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
    }

    /// Returns `true` if it is a unit vector.
    #[inline]
    pub fn is_normalize(self) -> bool {
        (self.len_sq() - 1.0).abs() <= f32::EPSILON 
    }

    /// Normalizes a quaternion.
    /// If normalization fails, `None`is returned.
    #[inline]
    pub fn normalize(self) -> Option<Self> {
        let len = self.len();
        match len <= f32::EPSILON {
            false => unsafe { Some(Quaternion(vmulq_n_f32(*self, len.recip()))) }, 
            true => None,
        }
    }

    /// Returns the conjugate of the quaternion.
    #[inline]
    pub fn conjugate(self) -> Self {
        const NEG_NEG_NEG_ONE: [f32; 4] = [-1.0, -1.0, -1.0, 1.0];
        unsafe {
            let neg_neg_neg_one = vld1q_f32(&NEG_NEG_NEG_ONE as *const f32);
            let conjugate = vmulq_f32(*self, neg_neg_neg_one);
            return Quaternion(conjugate);
        }
    }

    /// Returns the inverse of the quaternion.
    /// If normalization fails, `None` is returned.
    #[inline]
    pub fn inverse(self) -> Option<Self> {
        self.normalize().map(|norm| {
            norm.conjugate()
        })
    }
}

impl ops::Deref for Quaternion {
    type Target = float32x4_t;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Quaternion {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[f32; 4]> for Quaternion {
    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self::from(Float4::from(value))
    }
}

impl Into<[f32; 4]> for Quaternion {
    #[inline]
    fn into(self) -> [f32; 4] {
        let value: Float4 = self.into();
        value.into()
    }
}

impl From<Vector> for Quaternion {
    #[inline]
    fn from(value: Vector) -> Self {
        Quaternion(value.0)
    }
}

impl Into<Vector> for Quaternion {
    #[inline]
    fn into(self) -> Vector {
        Vector(*self)
    }
}

impl TryFrom<Matrix> for Quaternion {
    type Error = Quaternion;

    /// Convert a matrix to a quaternion.
    /// 
    /// # Errors
    /// If each axis of the matrix cannot be converted to a unit vector, 
    /// it returns an identity quaternion.
    /// 
    fn try_from(value: Matrix) -> Result<Self, Self::Error> {
        let x_axis = Vector(value[0]).vec3_normalize();
        let y_axis = Vector(value[1]).vec3_normalize();
        let z_axis = Vector(value[2]).vec3_normalize();
        match (x_axis, y_axis, z_axis) {
            (Some(x_axis), Some(y_axis), Some(z_axis)) => Ok(
                Self::from_rotation_axes(x_axis, y_axis, z_axis)
            ), 
            _ => { Err(Float4::W.into()) }
        }
    }
}

impl TryInto<Matrix> for Quaternion {
    type Error = Matrix;

    /// Convert a quaternion to a matrix.
    /// 
    /// # Errors
    /// If the quaternion cannot be normalized, 
    /// it returns an identity matrix.
    /// 
    fn try_into(self) -> Result<Matrix, Self::Error> {
        self.normalize()
            .map(|quat| {
                quat.to_rotation_axes()
            })
            .map(|(x_axis, y_axis, z_axis)| {
                let w_axis: Vector = Float4::W.into();
                Matrix([*x_axis, *y_axis, *z_axis, *w_axis])
            })
            .ok_or(Float4x4::IDENTITY.into())
    }
}

impl From<Float4> for Quaternion {
    #[inline]
    fn from(value: Float4) -> Self {
        unsafe { Quaternion(vld1q_f32(&value as *const _ as *const f32)) }
    }
}

impl Into<Float4> for Quaternion {
    #[inline]
    fn into(self) -> Float4 {
        let mut value = Float4::default();
        unsafe { vst1q_f32(&mut value as *mut _ as *mut f32, *self) };
        return value;
    }
}

impl ops::Mul<Self> for Quaternion {
    type Output = Self;
    /// Multiplies two quaternions.
    fn mul(self, rhs: Self) -> Self::Output {
        const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
        const ONE_ONE_NEG_NEG: [f32; 4] = [1.0, 1.0, -1.0, -1.0];
        const NEG_ONE_ONE_NEG: [f32; 4] = [-1.0, 1.0, 1.0, -1.0];
        // self: a, rhs: b
        // i: aw*bx + ax*bw + ay*bz - az*by
        // j: aw*by - ax*bz + ay*bw + az*bx
        // k: aw*bz + ax*by - ay*bx + az*bw
        // w: aw*bw - ax*bx - ay*by - az*bz
        //
        unsafe {
            let bx_by = vget_low_f32(*rhs);
            let bz_bw = vget_high_f32(*rhs);
            let by_bx = vext_f32::<0b01>(bx_by, bx_by);
            let bw_bz = vext_f32::<0b01>(bz_bw, bz_bw);

            let bx_by_bz_bw = *rhs;
            let bw_bz_by_bx = vcombine_f32(bw_bz, by_bx);
            let bz_bw_bx_by = vcombine_f32(bz_bw, bx_by);
            let by_bx_bw_bz = vcombine_f32(by_bx, bw_bz);
            
            let one_neg_one_neg = vld1q_f32(&ONE_NEG_ONE_NEG as *const f32);
            let one_one_neg_neg = vld1q_f32(&ONE_ONE_NEG_NEG as *const f32);
            let neg_one_one_neg = vld1q_f32(&NEG_ONE_ONE_NEG as *const f32);

            let aw = vdupq_laneq_f32::<0b11>(*self);
            let e0 = vmulq_f32(aw, bx_by_bz_bw);

            let ax = vdupq_laneq_f32::<0b00>(*self);
            let e1 = vmulq_f32(ax, bw_bz_by_bx);
            let e1 = vmulq_f32(e1, one_neg_one_neg);

            let ay = vdupq_laneq_f32::<0b01>(*self);
            let e2 = vmulq_f32(ay, bz_bw_bx_by);
            let e2 = vmulq_f32(e2, one_one_neg_neg);

            let az = vdupq_laneq_f32::<0b10>(*self);
            let e3 = vmulq_f32(az, by_bx_bw_bz);
            let e3 = vmulq_f32(e3, neg_one_one_neg);

            let mut result = vaddq_f32(e0, e1);
            result = vaddq_f32(result, e2);
            result = vaddq_f32(result, e3);

            return Quaternion(result);
        }
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
            .field(&*self)
            .finish()
    }
}
