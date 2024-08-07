use core::ops;
use crate::macros::impl_element3;
use super::bool2::Boolean2;
use super::bool4::Boolean4;



/// A structure that stores three-dimensional boolean data.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl Boolean3 {
    /// All elements are `true`.
    pub const TRUE: Self = Self::fill(true);

    /// All elements are `false`.
    pub const FALSE: Self = Self::fill(false);

    /// Returns `true` if any of the elements are `true`.
    #[inline]
    pub const fn any(self) -> bool {
        self.x | self.y | self.z
    }

    /// Returns `true` if all elements are `true`.
    #[inline]
    pub const fn all(self) -> bool {
        self.x & self.y & self.z
    }
}

// Vector swizzle code implementation.
impl Boolean3 {
    #[inline]
    pub const fn xx(self) -> Boolean2 {
        Boolean2 { x: self.x, y: self.x }
    }

    #[inline]
    pub const fn xy(self) -> Boolean2 {
        Boolean2 { x: self.x, y: self.y }
    }

    #[inline]
    pub const fn xz(self) -> Boolean2 {
        Boolean2 { x: self.x, y: self.z }
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
    pub const fn yz(self) -> Boolean2 {
        Boolean2 { x: self.y, y: self.z }
    }

    #[inline]
    pub const fn zx(self) -> Boolean2 {
        Boolean2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(self) -> Boolean2 {
        Boolean2 { x: self.z, y: self.y }
    }

    #[inline]
    pub const fn zz(self) -> Boolean2 {
        Boolean2 { x: self.z, y: self.z }
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
    pub const fn xxz(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.x, z: self.z }
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
    pub const fn xyz(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn xzx(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn xzy(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn xzz(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.z, z: self.z }
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
    pub const fn yxz(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.x, z: self.z }
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
    pub const fn yyz(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn yzx(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn yzy(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn yzz(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn zxx(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn zxy(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn zxz(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn zyx(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn zyy(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn zyz(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn zzx(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn zzy(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn zzz(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.z, z: self.z }
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
    pub const fn xxxz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.x, w: self.z }
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
    pub const fn xxyz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xxzx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xxzy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xxzz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.z, w: self.z }
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
    pub const fn xyxz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.x, w: self.z }
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
    pub const fn xyyz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xyzx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xyzy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xyzz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xzxx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xzxy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xzxz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xzyx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xzyy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xzyz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xzzx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xzzy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xzzz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.z, w: self.z }
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
    pub const fn yxxz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.x, w: self.z }
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
    pub const fn yxzx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yxzy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yxzz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.z, w: self.z }
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
    pub const fn yyxz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yyyx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yyyy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yyyz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yyzx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yyzy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yyzz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yzxx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yzxy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yzxz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yzyx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yzyy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yzyz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yzzx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yzzy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yzzz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zxxx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zxxy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zxxz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zxyx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zxyy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zxyz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zxzx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zxzy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zxzz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zyxx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zyxy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zyxz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zyyx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zyyy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zyyz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zyzx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zyzy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zyzz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zzxx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zzxy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zzxz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zzyx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zzyy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zzyz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zzzx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zzzy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.z, w: self.y }
    }
    
    #[inline]
    pub const fn zzzz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.z, w: self.z }
    }
}

impl_element3!(bool, Boolean3);

impl Default for Boolean3 {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

impl From<Boolean2> for Boolean3 {
    #[inline]
    fn from(value: Boolean2) -> Self {
        Boolean3 { x: value.x, y: value.y, z: false }
    }
}

impl From<Boolean4> for Boolean3 {
    #[inline]
    fn from(value: Boolean4) -> Self {
        Boolean3 { x: value.x, y: value.y, z: value.z }
    }
}

impl ops::BitAnd<Self> for Boolean3 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Boolean3 {
            x: self.x & rhs.x, 
            y: self.y & rhs.y, 
            z: self.z & rhs.z, 
        }
    }
}

impl ops::BitAndAssign for Boolean3 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl ops::BitOr<Self> for Boolean3 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Boolean3 {
            x: self.x | rhs.x, 
            y: self.y | rhs.y, 
            z: self.z | rhs.z, 
        }
    }
}

impl ops::BitOrAssign for Boolean3 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl ops::BitXor<Self> for Boolean3 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Boolean3 {
            x: self.x ^ rhs.x, 
            y: self.y ^ rhs.y, 
            z: self.z ^ rhs.z, 
        }
    }
}

impl ops::BitXorAssign for Boolean3 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl ops::Not for Boolean3 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Boolean3 {
            x: !self.x, 
            y: !self.y, 
            z: !self.z, 
        }
    }
}
