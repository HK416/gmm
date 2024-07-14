use core::fmt;
use core::ops;

#[cfg(target_pointer_width = "32")]
use core::arch::x86::*;

#[cfg(target_pointer_width = "64")]
use core::arch::x86_64::*;

use super::VectorInt;
use crate::{ Float2, Float3, Float4 };



/// This is a vector data type that uses the `SIMD` instruction.
/// 
/// It is recommended not to use this data type as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vector(pub(crate) __m128);

impl Vector {
    /// Takes the samller of the elements of the two vectors.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        unsafe { Vector(_mm_min_ps(*self, *rhs)) }
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        unsafe { Vector(_mm_max_ps(*self, *rhs)) }
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    pub fn lt(self, rhs: Self) -> VectorInt {
        unsafe { 
            let comp = _mm_cmplt_ps(*self, *rhs);
            let cast = _mm_castps_si128(comp);
            return VectorInt(cast); 
        }
    }

    /// Checks if the elements of two vectors are less than or eqaul.
    #[inline]
    pub fn le(self, rhs: Self) -> VectorInt { 
        self.lt(rhs) | self.eq(rhs)
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    pub fn gt(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = _mm_cmpgt_ps(*self, *rhs);
            let cast = _mm_castps_si128(comp);
            return VectorInt(cast);
        }
    }

    /// Checks if the elements of two vectors are greater than or eqaul.
    #[inline]
    pub fn ge(self, rhs: Self) -> VectorInt {
        self.gt(rhs) | self.eq(rhs)
    }

    /// Checks if the elements of two vectors are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn eq(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = _mm_cmpeq_ps(*self, *rhs);
            let cast = _mm_castps_si128(comp);
            return VectorInt(cast);
        }
    }

    /// Checks if the elements of two vectors are not eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn ne(self, rhs: Self) -> VectorInt {
        !self.eq(rhs)
    }
}

impl Vector {
    /// Absolute value on vector elements.
    #[inline]
    pub fn abs(self) -> Self {
        self.max(-self)
    }
    
    /// Return a vector filled by adding all the elements of the vector.
    #[inline]
    pub fn sum(self) -> Self {
        unsafe {
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(*self, *self);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(*self, *self);
            let sum = _mm_add_ps(low, high);
            let mix = _mm_shuffle_ps::<0b_10_11_00_01>(sum, sum);
            let sum = _mm_add_ps(sum, mix);
            return Vector(sum);
        }
    }

    /// Dot product of tow two-element vectors.
    #[inline]
    pub fn vec2_dot(self, rhs: Self) -> Vector {
        let mul = self * rhs;
        unsafe {
            let a = _mm_shuffle_ps::<0b_01_00_01_00>(*mul, *mul);
            let b = _mm_shuffle_ps::<0b_00_01_00_01>(*mul, *mul);
            let sum = _mm_add_ps(a, b);
            return Vector(sum);
        }
    }

    /// Dot product of tow three-element vectors.
    #[inline]
    pub fn vec3_dot(self, rhs: Self) -> Vector {
        let mul = self * rhs;
        unsafe {
            let a = _mm_shuffle_ps::<0b_01_00_01_00>(*mul, *mul);
            let b = _mm_shuffle_ps::<0b_00_01_00_01>(*mul, *mul);
            let c = _mm_shuffle_ps::<0b_10_10_10_10>(*mul, *mul);
            let sum = _mm_add_ps(a, b);
            let sum = _mm_add_ps(sum, c);
            return Vector(sum);
        }
    }

    /// Dot product of two four-element vectors.
    #[inline]
    pub fn vec4_dot(self, rhs: Self) -> Vector {
        (self * rhs).sum()
    }

    /// Cross product of two three-element vectors.
    #[inline]
    pub fn vec3_cross(self, rhs: Self) -> Vector {
        const MASK_XYZ: [f32; 4] = [1.0, 1.0, 1.0, 0.0];
        unsafe {
            let ly_lz_lx = _mm_shuffle_ps::<0b_00_00_10_01>(*self, *self);
            let lz_lx_ly = _mm_shuffle_ps::<0b_00_01_00_10>(*self, *self);
            let rz_rx_ry = _mm_shuffle_ps::<0b_00_01_00_10>(*rhs, *rhs);
            let ry_rz_rx = _mm_shuffle_ps::<0b_00_00_10_01>(*rhs, *rhs);
            
            let a = _mm_mul_ps(ly_lz_lx, rz_rx_ry);
            let b = _mm_mul_ps(lz_lx_ly, ry_rz_rx);
            let result = _mm_sub_ps(a, b);
            
            let mask = _mm_loadu_ps(&MASK_XYZ as *const _ as *const f32);
            let result = _mm_mul_ps(result, mask);

            return Vector(result);
        }
    }

    /// Length squared of a two-element vector.
    #[inline]
    pub fn vec2_len_sq(self) -> f32 {
        unsafe { _mm_cvtss_f32(*self.vec2_dot(self)) }
    }

    /// Length of a two-element vector.
    #[inline]
    pub fn vec2_len(self) -> f32 {
        self.vec2_len_sq().sqrt()
    }

    /// Length squared of a three-element vector.
    #[inline]
    pub fn vec3_len_sq(self) -> f32 {
        unsafe { _mm_cvtss_f32(*self.vec3_dot(self)) }
    }

    /// Length of a three-element vector.
    #[inline]
    pub fn vec3_len(self) -> f32 {
        self.vec3_len_sq().sqrt()
    }

    /// Length squared of a four-element vector.
    #[inline]
    pub fn vec4_len_sq(self) -> f32 {
        unsafe { _mm_cvtss_f32(*self.vec4_dot(self)) }
    }

    /// Length of a four-element vector.
    #[inline]
    pub fn vec4_len(self) -> f32 {
        self.vec4_len_sq().sqrt()
    }

    /// Normalizes a two-element vector.
    /// If normalization fails, `None` is returned.
    #[inline]
    pub fn vec2_normalize(self) -> Option<Self> {
        const MASK_XY: Float4 = Float4::new(1.0, 1.0, 0.0, 0.0);
        let len = self.vec2_len();
        match len <= f32::EPSILON {
            false => {
                let mask: Vector = MASK_XY.into();
                let recip_len: Vector = Float4::fill(len.recip()).into();
                Some(self * recip_len * mask)
            },
            true => None,
        }
    }

    /// Normalizes a three-element vector.
    /// If normalization fails, `None` is returned.
    #[inline]
    pub fn vec3_normalize(self) -> Option<Self> {
        const MASK_XYZ: Float4 = Float4::new(1.0, 1.0, 1.0, 0.0);
        let len = self.vec3_len();
        match len <= f32::EPSILON {
            false => {
                let mask: Vector = MASK_XYZ.into();
                let recip_len: Vector = Float4::fill(len.recip()).into();
                Some(self * recip_len * mask)
            },
            true => None,
        }
    }

    /// Normalizes a four-element vector.
    /// If normalization fails, `None` is returned.
    #[inline]
    pub fn vec4_normalize(self) -> Option<Self> {
        let len = self.vec4_len();
        match len <= f32::EPSILON {
            false => {
                let recip_len: Vector = Float4::fill(len.recip()).into();
                Some(self * recip_len)
            },
            true => None,
        }
    }
}

impl From<VectorInt> for Vector {
    #[inline]
    fn from(value: VectorInt) -> Self {
        unsafe { Vector(_mm_castsi128_ps(*value)) }
    }
}

impl From<Float2> for Vector {
    #[inline]
    fn from(value: Float2) -> Self {
        Self::from(Float4::from(value))
    }
}

impl Into<Float2> for Vector {
    #[inline]
    fn into(self) -> Float2 {
        let value: Float4 = self.into();
        return Float2::from(value);
    }
}

impl From<Float3> for Vector {
    #[inline]
    fn from(value: Float3) -> Self {
        Self::from(Float4::from(value))
    }
}

impl Into<Float3> for Vector {
    #[inline]
    fn into(self) -> Float3 {
        let value: Float4 = self.into();
        return Float3::from(value);
    }
}

impl From<Float4> for Vector {
    #[inline]
    fn from(value: Float4) -> Self {
        unsafe { Vector(_mm_loadu_ps(&value as * const _ as *const f32)) }
    }
}

impl Into<Float4> for Vector {
    #[inline]
    fn into(self) -> Float4 {
        let mut value = Float4::default();
        unsafe { _mm_storeu_ps(&mut value as *mut _ as *mut f32, self.0) };
        return value;
    }
}

impl ops::Deref for Vector {
    type Target = __m128;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Vector {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ops::Add<Self> for Vector {
    type Output = Self;
    /// Adds two vectors. 
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Vector(_mm_add_ps(*self, *rhs)) }
    }
}

impl ops::AddAssign<Self> for Vector {
    /// Adds two vectors. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub<Self> for Vector {
    type Output = Self;
    /// Subtracts two vectors.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Vector(_mm_sub_ps(*self, *rhs)) }
    }
}

impl ops::SubAssign<Self> for Vector {
    /// Subtracts two vectors. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl ops::Neg for Vector {
    type Output = Self;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        unsafe { Vector(_mm_sub_ps(_mm_setzero_ps(), *self)) }
    }
}

impl ops::Mul<Self> for Vector {
    type Output = Self;
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Vector(_mm_mul_ps(*self, *rhs)) }
    }
}

impl ops::MulAssign<Self> for Vector {
    /// Element-wise multiplication of two vectors. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl ops::Div<Self> for Vector {
    type Output = Self;
    /// Element-wise division of two vectors.
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Vector(_mm_div_ps(*self, *rhs)) }
    }
}

impl ops::DivAssign<Self> for Vector {
    /// Element-wise division of two vectors. (assign)
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl fmt::Debug for Vector {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vector))
            .field(&*self)
            .finish()
    }
}
