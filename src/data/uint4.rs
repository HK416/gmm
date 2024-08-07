use crate::macros::impl_element4;
use crate::macros::impl_element4_op;
use super::bool4::Boolean4;
use super::uint2::UInteger2;
use super::uint3::UInteger3;



/// A structure that stores four-dimensional unsigned integer data.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UInteger4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

impl UInteger4 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1, 0, 0, 0);
    
    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0, 1, 0, 0);

    /// positive unit vector on z-axis.
    pub const Z: Self = Self::new(0, 0, 1, 0);

    /// positive unit vector on w-axis.
    pub const W: Self = Self::new(0, 0, 0, 1);

    /// All elements are [`u32::MIN`].
    pub const MIN: Self = Self::fill(u32::MIN);

    /// All elements are [`u32::MAX`].
    pub const MAX: Self = Self::fill(u32::MAX);
}

// Vector swizzle code implementation.
impl UInteger4 {
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
    pub const fn xw(self) -> UInteger2 {
        UInteger2 { x: self.x, y: self.w }
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
    pub const fn yw(self) -> UInteger2 {
        UInteger2 { x: self.y, y: self.w }
    }

    #[inline]
    pub const fn zx(self) -> UInteger2 {
        UInteger2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(self) -> UInteger2 {
        UInteger2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zz(self) -> UInteger2 {
        UInteger2 { x: self.z, y: self.z }
    }

    #[inline]
    pub const fn zw(self) -> UInteger2 {
        UInteger2 { x: self.z, y: self.w }
    }

    #[inline]
    pub const fn wx(self) -> UInteger2 {
        UInteger2 { x: self.w, y: self.x }
    }

    #[inline]
    pub const fn wy(self) -> UInteger2 {
        UInteger2 { x: self.w, y: self.y }
    }

    #[inline]
    pub const fn wz(self) -> UInteger2 {
        UInteger2 { x: self.w, y: self.z }
    }

    #[inline]
    pub const fn ww(self) -> UInteger2 {
        UInteger2 { x: self.w, y: self.w }
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
    pub const fn xxw(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.x, z: self.w }
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
    pub const fn xyw(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.y, z: self.w }
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
    pub const fn xzw(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn xwx(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn xwy(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn xwz(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn xww(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.w, z: self.w }
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
    pub const fn yxw(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.x, z: self.w }
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
    pub const fn yyw(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.y, z: self.w }
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
    pub const fn yzw(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn ywx(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn ywy(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn ywz(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn yww(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.w, z: self.w }
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
    pub const fn zxw(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.x, z: self.w }
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
    pub const fn zyw(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.y, z: self.w }
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
    pub const fn zzw(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn zwx(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn zwy(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn zwz(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn zww(self) -> UInteger3 {
        UInteger3 { x: self.z, y: self.w, z: self.w }
    }

    #[inline]
    pub const fn wxx(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn wxy(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn wxz(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn wxw(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.x, z: self.w }
    }

    #[inline]
    pub const fn wyx(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn wyy(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn wyz(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn wyw(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.y, z: self.w }
    }

    #[inline]
    pub const fn wzx(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn wzy(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn wzz(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn wzw(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn wwx(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn wwy(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn wwz(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn www(self) -> UInteger3 {
        UInteger3 { x: self.w, y: self.w, z: self.w }
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
    pub const fn xxxw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.x, w: self.w }
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
    pub const fn xxyw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.y, w: self.w }
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
    pub const fn xxzw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xxwx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xxwy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xxwz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xxww(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.w, w: self.w }
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
    pub const fn xyxw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.x, w: self.w }
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
    pub const fn xyyw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.y, w: self.w }
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
    pub const fn xyzw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xywx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xywy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xywz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xyww(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.w, w: self.w }
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
    pub const fn xzxw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.x, w: self.w }
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
    pub const fn xzyw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.y, w: self.w }
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
    pub const fn xzzw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xzwx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xzwy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xzwz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xzww(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn xwxx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xwxy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xwxz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xwxw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn xwyx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xwyy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xwyz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xwyw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn xwzx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xwzy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xwzz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xwzw(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xwwx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xwwy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xwwz(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xwww(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.w, z: self.w, w: self.w }
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
    pub const fn yxxw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.x, w: self.w }
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
    pub const fn yxyz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yxyw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.y, w: self.w }
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
    pub const fn yxzw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yxwx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yxwy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yxwz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yxww(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.w, w: self.w }
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
    pub const fn yyxw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.x, w: self.w }
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
    pub const fn yyyw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.y, w: self.w }
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
    pub const fn yyzw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yywx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yywy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yywz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yyww(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.w, w: self.w }
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
    pub const fn yzxw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.x, w: self.w }
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
    pub const fn yzyw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.y, w: self.w }
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
    pub const fn yzzw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yzwx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yzwy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yzwz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yzww(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn ywxx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn ywxy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn ywxz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn ywxw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn ywyx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn ywyy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn ywyz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn ywyw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn ywzx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn ywzy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn ywzz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn ywzw(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn ywwx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn ywwy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn ywwz(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn ywww(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.w, z: self.w, w: self.w }
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
    pub const fn zxxw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.x, w: self.w }
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
    pub const fn zxyw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.y, w: self.w }
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
    pub const fn zxzw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zxwx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zxwy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zxwz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zxww(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.x, z: self.w, w: self.w }
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
    pub const fn zyxw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.x, w: self.w }
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
    pub const fn zyyw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.y, w: self.w }
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
    pub const fn zyzw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zywx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zywy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zywz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zyww(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.y, z: self.w, w: self.w }
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
    pub const fn zzxw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.x, w: self.w }
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
    pub const fn zzyw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.y, w: self.w }
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

    #[inline]
    pub const fn zzzw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zzwx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zzwy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zzwz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zzww(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn zwxx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zwxy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zwxz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zwxw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn zwyx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zwyy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zwyz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zwyw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn zwzx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zwzy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zwzz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zwzw(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zwwx(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zwwy(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zwwz(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zwww(self) -> UInteger4 {
        UInteger4 { x: self.z, y: self.w, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wxxx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wxxy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wxxz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wxxw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wxyx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wxyy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wxyz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wxyw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wxzx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wxzy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wxzz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wxzw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wxwx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wxwy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wxwz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wxww(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.x, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wyxx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wyxy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wyxz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wyxw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wyyx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wyyy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wyyz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wyyw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wyzx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wyzy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wyzz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wyzw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wywx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wywy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wywz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wyww(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.y, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wzxx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wzxy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wzxz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wzxw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wzyx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wzyy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wzyz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wzyw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wzzx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wzzy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wzzz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wzzw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wzwx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wzwy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wzwz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wzww(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn wwxx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wwxy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wwxz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wwxw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wwyx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wwyy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wwyz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wwyw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wwzx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wwzy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wwzz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wwzw(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wwwx(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wwwy(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wwwz(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wwww(self) -> UInteger4 {
        UInteger4 { x: self.w, y: self.w, z: self.w, w: self.w }
    }
}

impl_element4!(u32, UInteger4);

impl_element4_op!(u32, UInteger4);

impl Default for UInteger4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Into<Boolean4> for UInteger4 {
    #[inline]
    fn into(self) -> Boolean4 {
        Boolean4 { 
            x: self.x == 0xFFFFFFFF, 
            y: self.y == 0xFFFFFFFF, 
            z: self.z == 0xFFFFFFFF, 
            w: self.w == 0xFFFFFFFF 
        }
    }
}

impl From<Boolean4> for UInteger4 {
    #[inline]
    fn from(value: Boolean4) -> Self {
        UInteger4 { 
            x: if value.x { 0xFFFFFFFF } else { 0 }, 
            y: if value.y { 0xFFFFFFFF } else { 0 }, 
            z: if value.z { 0xFFFFFFFF } else { 0 }, 
            w: if value.w { 0xFFFFFFFF } else { 0 } 
        }
    }
}

impl From<UInteger2> for UInteger4 {
    #[inline]
    fn from(value: UInteger2) -> Self {
        UInteger4 { x: value.x, y: value.y, z: 0, w: 0 }
    }
}

impl From<UInteger3> for UInteger4 {
    #[inline]
    fn from(value: UInteger3) -> Self {
        UInteger4 { x: value.x, y: value.y, z: value.z, w: 0 }
    }
}

impl core::ops::BitAnd<Self> for UInteger4 {
    type Output = Self;
    /// Element-wise bit `AND` operation of two vectors. 
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x & rhs.x, 
            y: self.y & rhs.y, 
            z: self.z & rhs.z, 
            w: self.w & rhs.w 
        }
    }
}

impl core::ops::BitAndAssign<Self> for UInteger4 {
    /// Element-wise bit `AND` operation of two vectors. (assign)
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl core::ops::BitOr<Self> for UInteger4 {
    type Output = Self;
    /// Element-wise bit `OR` operation of two vectors.
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x | rhs.x, 
            y: self.y | rhs.y, 
            z: self.z | rhs.z, 
            w: self.w | rhs.w 
        }
    }
}

impl core::ops::BitOrAssign<Self> for UInteger4 {
    /// Element-wise bit `OR` operation of two vectors. (assign)
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl core::ops::BitXor<Self> for UInteger4 {
    type Output = Self;
    /// Element-wise bit `XOR` operation of two vectors.
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x ^ rhs.x, 
            y: self.y ^ rhs.y, 
            z: self.z ^ rhs.z, 
            w: self.w ^ rhs.w 
        }
    }
}

impl core::ops::BitXorAssign<Self> for UInteger4 {
    /// Element-wise bit `XOR` operation of two vectors. (assign)
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl core::ops::Not for UInteger4 {
    type Output = Self;
    /// Element-wise bit `NOT` operation of two vectors. (assign)
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            x: !self.x, 
            y: !self.y, 
            z: !self.z, 
            w: !self.w 
        }
    }
}
