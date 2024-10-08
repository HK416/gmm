use crate::macros::impl_element3;
use crate::macros::impl_element3_op;
use super::bool3::Boolean3;
use super::uint2::UInteger2;
use super::uint4::UInteger4;



/// A structure that stores three-dimensional unsigned integer data.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UInteger3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl UInteger3 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1, 0, 0);
    
    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0, 1, 0);

    /// positive unit vector on z-axis.
    pub const Z: Self = Self::new(0, 0, 1);

    /// All elements are [`u32::MIN`].
    pub const MIN: Self = Self::fill(u32::MIN);

    /// All elements are [`u32::MAX`].
    pub const MAX: Self = Self::fill(u32::MAX);
}

// Vector swizzle code implementation.
impl UInteger3 {
    #[inline]
    pub const fn xx(self) -> UInteger2 {
        UInteger2 { x: self.x, y: self.x }
    }

    #[inline]
    pub const fn xy(self) -> UInteger2 {
        UInteger2 { x: self.x, y: self.y }
    }

    #[inline]
    pub const fn xz(self) -> UInteger2 {
        UInteger2 { x: self.x, y: self.z }
    }

    #[inline]
    pub const fn yx(self) -> UInteger2 {
        UInteger2 { x: self.y, y: self.x }
    }

    #[inline]
    pub const fn yy(self) -> UInteger2 {
        UInteger2 { x: self.y, y: self.y }
    }

    #[inline]
    pub const fn yz(self) -> UInteger2 {
        UInteger2 { x: self.y, y: self.z }
    }

    #[inline]
    pub const fn zx(self) -> UInteger2 {
        UInteger2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(self) -> UInteger2 {
        UInteger2 { x: self.z, y: self.y }
    }

    #[inline]
    pub const fn zz(self) -> UInteger2 {
        UInteger2 { x: self.z, y: self.z }
    }

    #[inline]
    pub const fn xxx(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn xxy(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn xxz(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn xyx(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn xyy(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn xyz(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn xzx(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn xzy(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn xzz(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn yxx(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn yxy(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn yxz(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn yyx(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn yyy(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn yyz(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn yzx(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn yzy(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn yzz(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn zxx(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn zxy(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn zxz(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn zyx(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn zyy(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn zyz(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn zzx(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn zzy(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn zzz(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn xxxx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xxxy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xxxz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xxyx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xxyy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xxyz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xxzx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xxzy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xxzz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.z, w: self.z }
    }
    
    #[inline]
    pub const fn xyxx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xyxy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xyxz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xyyx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xyyy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xyyz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xyzx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xyzy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xyzz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xzxx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xzxy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xzxz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xzyx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xzyy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xzyz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xzzx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xzzy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xzzz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yxxx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yxxy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yxxz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yxyx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yxyy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yxzx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yxzy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yxzz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yyxx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yyxy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yyxz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yyyx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yyyy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yyyz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yyzx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yyzy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yyzz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yzxx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yzxy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yzxz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yzyx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yzyy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yzyz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yzzx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yzzy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yzzz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zxxx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zxxy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zxxz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zxyx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zxyy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zxyz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zxzx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zxzy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zxzz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zyxx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zyxy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zyxz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zyyx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zyyy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zyyz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zyzx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zyzy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zyzz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zzxx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zzxy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zzxz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zzyx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zzyy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zzyz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zzzx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zzzy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.z, w: self.y }
    }
    
    #[inline]
    pub const fn zzzz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.z, w: self.z }
    }
}

impl_element3!(u32, UInteger3);

impl_element3_op!(u32, UInteger3);

impl Default for UInteger3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Into<Boolean3> for UInteger3 {
    #[inline]
    fn into(self) -> Boolean3 {
        Boolean3 { 
            x: self.x == 0xFFFFFFFF, 
            y: self.y == 0xFFFFFFFF, 
            z: self.z == 0xFFFFFFFF 
        }
    }
}

impl From<Boolean3> for UInteger3 {
    #[inline]
    fn from(value: Boolean3) -> Self {
        UInteger3 { 
            x: if value.x { 0xFFFFFFFF } else { 0 }, 
            y: if value.y { 0xFFFFFFFF } else { 0 }, 
            z: if value.z { 0xFFFFFFFF } else { 0 } 
        }
    }
}

impl From<UInteger2> for UInteger3 {
    #[inline]
    fn from(value: UInteger2) -> Self {
        UInteger3 { x: value.x, y: value.y, z: 0 }
    }
}

impl From<UInteger4> for UInteger3 {
    #[inline]
    fn from(value: UInteger4) -> Self {
        UInteger3 { x: value.x, y: value.y, z: value.z }
    }
}

impl core::ops::BitAnd<Self> for UInteger3 {
    type Output = Self;
    /// Element-wise bit `AND` operation of two vectors. 
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x & rhs.x, 
            y: self.y & rhs.y, 
            z: self.z & rhs.z 
        }
    }
}

impl core::ops::BitAndAssign<Self> for UInteger3 {
    /// Element-wise bit `AND` operation of two vectors. (assign)
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl core::ops::BitOr<Self> for UInteger3 {
    type Output = Self;
    /// Element-wise bit `OR` operation of two vectors.
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x | rhs.x, 
            y: self.y | rhs.y, 
            z: self.z | rhs.z  
        }
    }
}

impl core::ops::BitOrAssign<Self> for UInteger3 {
    /// Element-wise bit `OR` operation of two vectors. (assign)
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl core::ops::BitXor<Self> for UInteger3 {
    type Output = Self;
    /// Element-wise bit `XOR` operation of two vectors.
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x ^ rhs.x, 
            y: self.y ^ rhs.y, 
            z: self.z ^ rhs.z  
        }
    }
}

impl core::ops::BitXorAssign<Self> for UInteger3 {
    /// Element-wise bit `XOR` operation of two vectors. (assign)
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl core::ops::Not for UInteger3 {
    type Output = Self;
    /// Element-wise bit `NOT` operation of two vectors. (assign)
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            x: !self.x, 
            y: !self.y, 
            z: !self.z  
        }
    }
}
