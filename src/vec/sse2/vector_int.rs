use core::fmt;
use core::ops;

#[cfg(target_pointer_width = "32")]
use core::arch::x86::*;

#[cfg(target_pointer_width = "64")]
use core::arch::x86_64::*;

use super::Vector;
use crate::{
    Integer2, Integer3, Integer4, 
    UInteger2, UInteger3, UInteger4, 
};


/// This is a vector data type that uses the `SIMD` instruction.
/// 
/// It is recommended not to use this data type as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VectorInt(pub(crate) __m128i);

impl VectorInt {
    /// Takes the samller of the elements of the two vectors.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        unsafe { VectorInt(_mm_min_epi32(*self, *rhs)) }
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        unsafe { VectorInt(_mm_max_epi32(*self, *rhs)) }
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    pub fn lt(self, rhs: Self) -> VectorInt {
        unsafe { VectorInt(_mm_cmplt_epi32(*self, *rhs)) }
    }

    /// Checks if the elements of two vectors are less than or eqaul.
    #[inline]
    pub fn le(self, rhs: Self) -> VectorInt {
        self.lt(rhs) | self.eq(rhs)
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    pub fn gt(self, rhs: Self) -> VectorInt {
        unsafe { VectorInt(_mm_cmpgt_epi32(*self, *rhs)) }
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
        unsafe { VectorInt(_mm_cmpeq_epi32(*self, *rhs)) }
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

impl From<Vector> for VectorInt {
    #[inline]
    fn from(value: Vector) -> Self {
        unsafe { VectorInt(_mm_castps_si128(*value)) }
    }
}

impl From<Integer2> for VectorInt {
    #[inline]
    fn from(value: Integer2) -> Self {
        Self::from(Integer4::from(value))
    }
}

impl Into<Integer2> for VectorInt {
    #[inline]
    fn into(self) -> Integer2 {
        let value: Integer4 = self.into();
        return Integer2::from(value);
    }
}

impl From<Integer3> for VectorInt {
    #[inline]
    fn from(value: Integer3) -> Self {
        Self::from(Integer4::from(value))
    }
}

impl Into<Integer3> for VectorInt {
    #[inline]
    fn into(self) -> Integer3 {
        let value: Integer4 = self.into();
        return Integer3::from(value);
    }
}

impl From<Integer4> for VectorInt {
    #[inline]
    fn from(value: Integer4) -> Self {
        unsafe { VectorInt(_mm_loadu_si128(&value as *const _ as *const __m128i)) }
    }
}

impl Into<Integer4> for VectorInt {
    #[inline]
    fn into(self) -> Integer4 {
        let mut result = Integer4::default();
        unsafe { _mm_storeu_si128(&mut result as *mut _ as *mut __m128i, *self) };
        return result;
    }
}

impl From<UInteger2> for VectorInt {
    #[inline]
    fn from(value: UInteger2) -> Self {
        Self::from(UInteger4::from(value))
    }
}

impl Into<UInteger2> for VectorInt {
    #[inline]
    fn into(self) -> UInteger2 {
        let value: UInteger4 = self.into();
        return UInteger2::from(value);
    }
}

impl From<UInteger3> for VectorInt {
    #[inline]
    fn from(value: UInteger3) -> Self {
        Self::from(UInteger4::from(value))
    }
}

impl Into<UInteger3> for VectorInt {
    #[inline]
    fn into(self) -> UInteger3 {
        let value: UInteger4 = self.into();
        return UInteger3::from(value);
    }
}

impl From<UInteger4> for VectorInt {
    #[inline]
    fn from(value: UInteger4) -> Self {
        unsafe { VectorInt(_mm_loadu_si128(&value as *const _ as *const __m128i)) }
    }
}

impl Into<UInteger4> for VectorInt {
    #[inline]
    fn into(self) -> UInteger4 {
        let mut result = UInteger4::default();
        unsafe { _mm_storeu_si128(&mut result as *mut _ as *mut __m128i, *self) };
        return result;
    }
}

impl ops::Deref for VectorInt {
    type Target = __m128i;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for VectorInt {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ops::Add<Self> for VectorInt {
    type Output = Self;
    /// Adds two vectors.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { VectorInt(_mm_add_epi32(*self, *rhs)) }
    }
}

impl ops::AddAssign<Self> for VectorInt {
    /// Adds two vectors. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub<Self> for VectorInt {
    type Output = Self;
    /// Subtracts two vectors.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { VectorInt(_mm_sub_epi32(*self, *rhs)) }
    }
}

impl ops::SubAssign<Self> for VectorInt {
    /// Subtracts two vectors. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl ops::Neg for VectorInt {
    type Output = Self;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        unsafe { VectorInt(_mm_sub_epi32(_mm_setzero_si128(), *self)) }
    }
}

impl ops::Mul<Self> for VectorInt {
    type Output = Self;
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { VectorInt(_mm_mul_epi32(*self, *rhs)) }
    }
}

impl ops::MulAssign<Self> for VectorInt {
    /// Element-wise multiplication of two vectors. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl ops::BitAnd<Self> for VectorInt {
    type Output = Self;
    /// Element-wise bit `AND` operation of two vectors. 
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { VectorInt(_mm_and_si128(*self, *rhs)) }
    }
}

impl ops::BitAndAssign<Self> for VectorInt {
    /// Element-wise bit `AND` operation of two vectors. (assign)
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl ops::BitOr<Self> for VectorInt {
    type Output = Self;
    /// Element-wise bit `OR` operation of two vectors.
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { VectorInt(_mm_or_si128(*self, *rhs)) }
    }
}

impl ops::BitOrAssign<Self> for VectorInt {
    /// Element-wise bit `OR` operation of two vectors. (assign)
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl ops::BitXor<Self> for VectorInt {
    type Output = Self;
    /// Element-wise bit `XOR` operation of two vectors. 
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { VectorInt(_mm_xor_si128(*self, *rhs)) }
    }
}

impl ops::BitXorAssign<Self> for VectorInt {
    /// Element-wise bit `XOR` operation of two vectors. (assign)
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl ops::Not for VectorInt {
    type Output = Self;
    /// Element-wise bit `NOT` operation of two vectors.
    #[inline]
    fn not(self) -> Self::Output {
        unsafe { VectorInt(_mm_xor_si128(_mm_set1_epi32(-1) ,*self)) }
    }
}

impl fmt::Debug for VectorInt {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(VectorInt))
            .field(&*self)
            .finish()
    }
}
