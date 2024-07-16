use core::fmt;
use core::ops;

#[cfg(target_pointer_width = "32")]
use core::arch::x86::*;

#[cfg(target_pointer_width = "64")]
use core::arch::x86_64::*;

use crate::{ Vector, VectorInt, Float4 };



/// This is a quaternion data type that uses the `SIMD` instruction.
/// 
/// Using the `scalar-math` feature disables the use of `SIMD` instructions.
/// 
/// It is recommended not to use this data types as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Quaternion(pub(crate) __m128);

impl Quaternion {
    /// Checks if the elements of two quaternions are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn eq(self, rhs: Self) -> VectorInt {
        Vector(*self).eq(Vector(*rhs))
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
        Vector(*self).vec4_dot(Vector(*rhs))
    }

    /// Length squared of a quaternion.
    #[inline]
    pub fn len_sq(self) -> f32 {
        Vector(*self).vec4_len_sq()
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
        Vector(*self).vec4_normalize()
            .map(|norm| Quaternion(*norm))
    }

    /// Returns the conjugate of the quaternion.
    #[inline]
    pub fn conjugate(self) -> Self {
        const NEG_NEG_NEG_ONE: [f32; 4] = [-1.0, -1.0, -1.0, 1.0];
        unsafe {
            let neg_neg_neg_one = _mm_loadu_ps(&NEG_NEG_NEG_ONE as *const f32);
            let conjugate = _mm_mul_ps(*self, neg_neg_neg_one);
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
    type Target = __m128;
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
        unsafe { Quaternion(_mm_loadu_ps(&value as *const _ as *const f32)) }
    }
}

impl Into<Float4> for Quaternion {
    #[inline]
    fn into(self) -> Float4 {
        let mut value: Float4 = Float4::default();
        unsafe { _mm_storeu_ps(&mut value as *mut _ as *mut f32, *self) };
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
            let bx_by_bz_bw = *rhs;
            let bw_bz_by_bx = _mm_shuffle_ps::<0b_00_01_10_11>(*rhs, *rhs);
            let bz_bw_bx_by = _mm_shuffle_ps::<0b_01_00_11_10>(*rhs, *rhs);
            let by_bx_bw_bz = _mm_shuffle_ps::<0b_10_11_00_01>(*rhs, *rhs);
            
            let one_neg_one_neg = _mm_loadu_ps(&ONE_NEG_ONE_NEG as *const f32);
            let one_one_neg_neg = _mm_loadu_ps(&ONE_ONE_NEG_NEG as *const f32);
            let neg_one_one_neg = _mm_loadu_ps(&NEG_ONE_ONE_NEG as *const f32);

            let aw = _mm_shuffle_ps::<0b_11_11_11_11>(*self, *self);
            let e0 = _mm_mul_ps(aw, bx_by_bz_bw);

            let ax = _mm_shuffle_ps::<0b_00_00_00_00>(*self, *self);
            let e1 = _mm_mul_ps(ax, bw_bz_by_bx);
            let e1 = _mm_mul_ps(e1, one_neg_one_neg);

            let ay = _mm_shuffle_ps::<0b_01_01_01_01>(*self, *self);
            let e2 = _mm_mul_ps(ay, bz_bw_bx_by);
            let e2 = _mm_mul_ps(e2, one_one_neg_neg);

            let az = _mm_shuffle_ps::<0b_10_10_10_10>(*self, *self);
            let e3 = _mm_mul_ps(az, by_bx_bw_bz);
            let e3 = _mm_mul_ps(e3, neg_one_one_neg);

            let mut result = _mm_add_ps(e0, e1);
            result = _mm_add_ps(result, e2);
            result = _mm_add_ps(result, e3);

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
