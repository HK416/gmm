use crate::macros::impl_element2;
use crate::macros::impl_element2_op;
use super::int3::Integer3;
use super::int4::Integer4;



/// A structure that stores two-dimensional integer data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Integer2 {
    pub x: i32,
    pub y: i32,
}

impl Integer2 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1);
    
    /// All elements are negative one.
    pub const NEG_ONE: Self = Self::fill(-1);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1, 0);

    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0, 1);

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self::new(-1, 0);

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self::new(0, -1);

    /// All elements are [`i32::MIN`].
    pub const MIN: Self = Self::fill(i32::MIN);

    /// All elements are [`i32::MAX`].
    pub const MAX: Self = Self::fill(i32::MAX);
}

// Vector swizzle code implementation.
impl Integer2 {
    #[inline]
    pub const fn xx(self) -> Integer2 {
        Integer2 { x: self.x, y: self.x }
    }

    #[inline]
    pub const fn xy(self) -> Integer2 {
        Integer2 { x: self.x, y: self.y }
    }

    #[inline]
    pub const fn yx(self) -> Integer2 {
        Integer2 { x: self.y, y: self.x }
    }

    #[inline]
    pub const fn yy(self) -> Integer2 {
        Integer2 { x: self.y, y: self.y }
    }

    #[inline]
    pub const fn xxx(self) -> Integer3 {
        Integer3 { x: self.x, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn xxy(self) -> Integer3 {
        Integer3 { x: self.x, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn xyx(self) -> Integer3 {
        Integer3 { x: self.x, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn xyy(self) -> Integer3 {
        Integer3 { x: self.x, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn yxx(self) -> Integer3 {
        Integer3 { x: self.y, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn yxy(self) -> Integer3 {
        Integer3 { x: self.y, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn yyx(self) -> Integer3 {
        Integer3 { x: self.y, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn yyy(self) -> Integer3 {
        Integer3 { x: self.y, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn xxxx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xxxy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xxyx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xxyy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xyxx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xyxy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xyyx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xyyy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yxxx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yxxy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yxyx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yxyy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yyxx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yyxy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yyyx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yyyy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.y, w: self.y }
    }
}

impl Default for Integer2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl_element2!(i32, Integer2);

impl_element2_op!(i32, Integer2);

impl From<Integer3> for Integer2 {
    #[inline]
    fn from(value: Integer3) -> Self {
        Integer2 { x: value.x, y: value.y }
    }
}

impl From<Integer4> for Integer2 {
    #[inline]
    fn from(value: Integer4) -> Self {
        Integer2 { x: value.x, y: value.y }
    }
}

impl core::ops::Neg for Integer2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x, 
            y: -self.y, 
        }
    }
}

impl core::ops::BitAnd<Self> for Integer2 {
    type Output = Self;
    /// Element-wise bit `AND` operation of two vectors. 
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x & rhs.x, 
            y: self.y & rhs.y 
        }
    }
}

impl core::ops::BitAndAssign<Self> for Integer2 {
    /// Element-wise bit `AND` operation of two vectors. (assign)
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl core::ops::BitOr<Self> for Integer2 {
    type Output = Self;
    /// Element-wise bit `OR` operation of two vectors.
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x | rhs.x, 
            y: self.y | rhs.y  
        }
    }
}

impl core::ops::BitOrAssign<Self> for Integer2 {
    /// Element-wise bit `OR` operation of two vectors. (assign)
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl core::ops::BitXor<Self> for Integer2 {
    type Output = Self;
    /// Element-wise bit `XOR` operation of two vectors.
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x ^ rhs.x, 
            y: self.y ^ rhs.y  
        }
    }
}

impl core::ops::BitXorAssign<Self> for Integer2 {
    /// Element-wise bit `XOR` operation of two vectors. (assign)
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl core::ops::Not for Integer2 {
    type Output = Self;
    /// Element-wise bit `NOT` operation of two vectors. (assign)
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            x: !self.x, 
            y: !self.y  
        }
    }
}
