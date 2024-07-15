use core::fmt;
use core::ops;
use std::mem;
use crate::{ 
    Vector, Boolean4, 
    Integer2, Integer3, Integer4, 
    UInteger2, UInteger3, UInteger4, 
};



/// This is a vector data type that uses the `SIMD` instruction.
/// 
/// Using the `scalar-math` feature disables the use of `SIMD` instructions.
/// 
/// It is recommended not to use this data type as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VectorInt(pub(crate) Integer4);

impl VectorInt {
    /// Takes the samller of the elements of the two vectors.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Integer4 {
            x: self[0].min(rhs[0]), 
            y: self[1].min(rhs[1]), 
            z: self[2].min(rhs[2]), 
            w: self[3].min(rhs[3]) 
        }.into()
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Integer4 {
            x: self[0].max(rhs[0]), 
            y: self[1].max(rhs[1]), 
            z: self[2].max(rhs[2]), 
            w: self[3].max(rhs[3]) 
        }.into()
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    pub fn lt(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].lt(&rhs[0]), 
            y: self[1].lt(&rhs[1]), 
            z: self[2].lt(&rhs[2]), 
            w: self[3].lt(&rhs[3]) 
        }).into()
    }

    /// Checks if the elements of two vectors are less than or eqaul.
    #[inline]
    pub fn le(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].le(&rhs[0]), 
            y: self[1].le(&rhs[1]), 
            z: self[2].le(&rhs[2]), 
            w: self[3].le(&rhs[3]) 
        }).into()
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    pub fn gt(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].gt(&rhs[0]), 
            y: self[1].gt(&rhs[1]), 
            z: self[2].gt(&rhs[2]), 
            w: self[3].gt(&rhs[3]) 
        }).into()
    }

    /// Checks if the elements of two vectors are greater than or eqaul.
    #[inline]
    pub fn ge(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].ge(&rhs[0]), 
            y: self[1].ge(&rhs[1]), 
            z: self[2].ge(&rhs[2]), 
            w: self[3].ge(&rhs[3]) 
        }).into()
    }

    /// Checks if the elements of two vectors are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn eq(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].eq(&rhs[0]), 
            y: self[1].eq(&rhs[1]), 
            z: self[2].eq(&rhs[2]), 
            w: self[3].eq(&rhs[3]) 
        }).into()
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

impl From<[i32; 4]> for VectorInt {
    #[inline]
    fn from(value: [i32; 4]) -> Self {
        Self::from(Integer4::from(value))
    }
}

impl Into<[i32; 4]> for VectorInt {
    #[inline]
    fn into(self) -> [i32; 4] {
        let value: Integer4 = self.into();
        value.into()
    }
}

impl From<[u32; 4]> for VectorInt {
    #[inline]
    fn from(value: [u32; 4]) -> Self {
        Self::from(UInteger4::from(value))
    }
}

impl Into<[u32; 4]> for VectorInt {
    #[inline]
    fn into(self) -> [u32; 4] {
        let value: UInteger4 = self.into();
        value.into()
    }
}

impl From<Vector> for VectorInt {
    #[inline]
    fn from(value: Vector) -> Self {
        use core::mem;
        unsafe {
            Self(Integer4 {
                x: mem::transmute(value[0]), 
                y: mem::transmute(value[1]), 
                z: mem::transmute(value[2]), 
                w: mem::transmute(value[3]) 
            })
        }
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
        Self(value)
    }
}

impl Into<Integer4> for VectorInt {
    #[inline]
    fn into(self) -> Integer4 {
        *self
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
        use core::mem;
        unsafe {
            Self(Integer4 { 
                x: mem::transmute(value[0]), 
                y: mem::transmute(value[1]), 
                z: mem::transmute(value[2]), 
                w: mem::transmute(value[3]) 
            })
        }
    }
}

impl Into<UInteger4> for VectorInt {
    #[inline]
    fn into(self) -> UInteger4 {
        use core::mem;
        unsafe {
            UInteger4 { 
                x: mem::transmute(self[0]), 
                y: mem::transmute(self[1]), 
                z: mem::transmute(self[2]), 
                w: mem::transmute(self[3]) 
            }
        }
    }
}

impl ops::Deref for VectorInt {
    type Target = Integer4;
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
        VectorInt(*self + *rhs)
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
        VectorInt(*self + *rhs)
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
        VectorInt(-*self)
    }
}

impl ops::Mul<Self> for VectorInt {
    type Output = Self;
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        VectorInt(*self * *rhs)
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
        VectorInt(*self & *rhs)
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
        VectorInt(*self | *rhs)
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
        VectorInt(*self ^ *rhs)
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
        VectorInt(!*self)
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
