use core::fmt;
use core::ops;
use core::arch::aarch64::*;
use crate::{ Vector, VectorInt, Float4 };



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
