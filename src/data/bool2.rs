use core::ops;
use crate::macros::impl_element2;
use super::bool3::Boolean3;
use super::bool4::Boolean4;



/// A structure that stores two-dimensional boolean data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean2 {
    pub x: bool,
    pub y: bool,
}

impl Boolean2 {
    /// All elements are `true`.
    pub const TRUE: Self = Self::fill(true);

    /// All elements are `false`.
    pub const FALSE: Self = Self::fill(false);

    /// Returns `true` if any of the elements are `true`.
    #[inline]
    pub const fn any(self) -> bool {
        self.x | self.y
    }

    /// Returns `true` if all the elements are `true`.
    #[inline]
    pub const fn all(self) -> bool {
        self.x & self.y
    }
}

// Vector swizzle code implementation.
impl Boolean2 {
    #[inline]
    pub const fn xx(self) -> Boolean2 {
        Boolean2 { x: self.x, y: self.x }
    }

    #[inline]
    pub const fn xy(self) -> Boolean2 {
        Boolean2 { x: self.x, y: self.y }
    }

    #[inline]
    pub const fn yx(self) -> Boolean2 {
        Boolean2 { x: self.y, y: self.x }
    }

    #[inline]
    pub const fn yy(self) -> Boolean2 {
        Boolean2 { x: self.y, y: self.y }
    }

    #[inline]
    pub const fn xxx(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn xxy(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn xyx(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn xyy(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn yxx(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn yxy(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn yyx(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn yyy(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn xxxx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xxxy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xxyx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xxyy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xyxx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xyxy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xyyx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xyyy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yxxx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yxxy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yxyx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yxyy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yyxx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yyxy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yyyx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yyyy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.y, w: self.y }
    }
}

impl_element2!(bool, Boolean2);

impl Default for Boolean2 {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

impl From<Boolean3> for Boolean2 {
    #[inline]
    fn from(value: Boolean3) -> Self {
        Boolean2 { x: value.x, y: value.y }
    }
}

impl From<Boolean4> for Boolean2 {
    #[inline]
    fn from(value: Boolean4) -> Self {
        Boolean2 { x: value.x, y: value.y }
    }
}

impl ops::BitAnd<Self> for Boolean2 {
    type Output = Boolean2;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Boolean2 {
            x: self.x & rhs.x, 
            y: self.y & rhs.y, 
        }
    }
}

impl ops::BitAndAssign for Boolean2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl ops::BitOr<Self> for Boolean2 {
    type Output = Boolean2;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Boolean2 {
            x: self.x | rhs.x, 
            y: self.y | rhs.y, 
        }
    }
}

impl ops::BitOrAssign for Boolean2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl ops::BitXor<Self> for Boolean2 {
    type Output = Boolean2;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Boolean2 {
            x: self.x ^ rhs.x, 
            y: self.y ^ rhs.y, 
        }
    }
}

impl ops::BitXorAssign for Boolean2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl ops::Not for Boolean2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Boolean2 {
            x: !self.x, 
            y: !self.y
        }
    }
}
