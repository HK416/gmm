use crate::macros::impl_element4;
use crate::macros::impl_element4_op;
use super::int2::Integer2;
use super::int3::Integer3;



/// A structure that stores four-dimensional integer data.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Integer4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Integer4 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1);
    
    /// All elements are negative one.
    pub const NEG_ONE: Self = Self::fill(-1);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1, 0, 0, 0);

    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0, 1, 0, 0);

    /// positive unit vector on z-axis.
    pub const Z: Self = Self::new(0, 0, 1, 0);

    /// positive unit vector on w-axis.
    pub const W: Self = Self::new(0, 0, 0, 1);

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self::new(-1, 0, 0, 0);

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self::new(0, -1, 0, 0);

    /// negative unit vector on z-axis.
    pub const NEG_Z: Self = Self::new(0, 0, -1, 0);

    /// negative unit vector on w-axis.
    pub const NEG_W: Self = Self::new(0, 0, 0, -1);

    /// All elements are [`i32::MIN`].
    pub const MIN: Self = Self::fill(i32::MIN);

    /// All elements are [`i32::MAX`].
    pub const MAX: Self = Self::fill(i32::MAX);
}

// Vector swizzle code implementation.
impl Integer4 {
    #[inline]
    pub const fn xx(self) -> Integer2 {
        Integer2 { x: self.x, y: self.x }
    }

    #[inline]
    pub const fn xy(self) -> Integer2 {
        Integer2 { x: self.x, y: self.y }
    }

    #[inline]
    pub const fn xz(self) -> Integer2 {
        Integer2 { x: self.x, y: self.z }
    }

    #[inline]
    pub const fn xw(self) -> Integer2 {
        Integer2 { x: self.x, y: self.w }
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
    pub const fn yz(self) -> Integer2 {
        Integer2 { x: self.y, y: self.z }
    }

    #[inline]
    pub const fn yw(self) -> Integer2 {
        Integer2 { x: self.y, y: self.w }
    }

    #[inline]
    pub const fn zx(self) -> Integer2 {
        Integer2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(self) -> Integer2 {
        Integer2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zz(self) -> Integer2 {
        Integer2 { x: self.z, y: self.z }
    }

    #[inline]
    pub const fn zw(self) -> Integer2 {
        Integer2 { x: self.z, y: self.w }
    }

    #[inline]
    pub const fn wx(self) -> Integer2 {
        Integer2 { x: self.w, y: self.x }
    }

    #[inline]
    pub const fn wy(self) -> Integer2 {
        Integer2 { x: self.w, y: self.y }
    }

    #[inline]
    pub const fn wz(self) -> Integer2 {
        Integer2 { x: self.w, y: self.z }
    }

    #[inline]
    pub const fn ww(self) -> Integer2 {
        Integer2 { x: self.w, y: self.w }
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
    pub const fn xxz(self) -> Integer3 {
        Integer3 { x: self.x, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn xxw(self) -> Integer3 {
        Integer3 { x: self.x, y: self.x, z: self.w }
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
    pub const fn xyz(self) -> Integer3 {
        Integer3 { x: self.x, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn xyw(self) -> Integer3 {
        Integer3 { x: self.x, y: self.y, z: self.w }
    }

    #[inline]
    pub const fn xzx(self) -> Integer3 {
        Integer3 { x: self.x, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn xzy(self) -> Integer3 {
        Integer3 { x: self.x, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn xzz(self) -> Integer3 {
        Integer3 { x: self.x, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn xzw(self) -> Integer3 {
        Integer3 { x: self.x, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn xwx(self) -> Integer3 {
        Integer3 { x: self.x, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn xwy(self) -> Integer3 {
        Integer3 { x: self.x, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn xwz(self) -> Integer3 {
        Integer3 { x: self.x, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn xww(self) -> Integer3 {
        Integer3 { x: self.x, y: self.w, z: self.w }
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
    pub const fn yxz(self) -> Integer3 {
        Integer3 { x: self.y, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn yxw(self) -> Integer3 {
        Integer3 { x: self.y, y: self.x, z: self.w }
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
    pub const fn yyz(self) -> Integer3 {
        Integer3 { x: self.y, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn yyw(self) -> Integer3 {
        Integer3 { x: self.y, y: self.y, z: self.w }
    }

    #[inline]
    pub const fn yzx(self) -> Integer3 {
        Integer3 { x: self.y, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn yzy(self) -> Integer3 {
        Integer3 { x: self.y, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn yzz(self) -> Integer3 {
        Integer3 { x: self.y, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn yzw(self) -> Integer3 {
        Integer3 { x: self.y, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn ywx(self) -> Integer3 {
        Integer3 { x: self.y, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn ywy(self) -> Integer3 {
        Integer3 { x: self.y, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn ywz(self) -> Integer3 {
        Integer3 { x: self.y, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn yww(self) -> Integer3 {
        Integer3 { x: self.y, y: self.w, z: self.w }
    }

    #[inline]
    pub const fn zxx(self) -> Integer3 {
        Integer3 { x: self.z, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn zxy(self) -> Integer3 {
        Integer3 { x: self.z, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn zxz(self) -> Integer3 {
        Integer3 { x: self.z, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn zxw(self) -> Integer3 {
        Integer3 { x: self.z, y: self.x, z: self.w }
    }

    #[inline]
    pub const fn zyx(self) -> Integer3 {
        Integer3 { x: self.z, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn zyy(self) -> Integer3 {
        Integer3 { x: self.z, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn zyz(self) -> Integer3 {
        Integer3 { x: self.z, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn zyw(self) -> Integer3 {
        Integer3 { x: self.z, y: self.y, z: self.w }
    }

    #[inline]
    pub const fn zzx(self) -> Integer3 {
        Integer3 { x: self.z, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn zzy(self) -> Integer3 {
        Integer3 { x: self.z, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn zzz(self) -> Integer3 {
        Integer3 { x: self.z, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn zzw(self) -> Integer3 {
        Integer3 { x: self.z, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn zwx(self) -> Integer3 {
        Integer3 { x: self.z, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn zwy(self) -> Integer3 {
        Integer3 { x: self.z, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn zwz(self) -> Integer3 {
        Integer3 { x: self.z, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn zww(self) -> Integer3 {
        Integer3 { x: self.z, y: self.w, z: self.w }
    }

    #[inline]
    pub const fn wxx(self) -> Integer3 {
        Integer3 { x: self.w, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn wxy(self) -> Integer3 {
        Integer3 { x: self.w, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn wxz(self) -> Integer3 {
        Integer3 { x: self.w, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn wxw(self) -> Integer3 {
        Integer3 { x: self.w, y: self.x, z: self.w }
    }

    #[inline]
    pub const fn wyx(self) -> Integer3 {
        Integer3 { x: self.w, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn wyy(self) -> Integer3 {
        Integer3 { x: self.w, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn wyz(self) -> Integer3 {
        Integer3 { x: self.w, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn wyw(self) -> Integer3 {
        Integer3 { x: self.w, y: self.y, z: self.w }
    }

    #[inline]
    pub const fn wzx(self) -> Integer3 {
        Integer3 { x: self.w, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn wzy(self) -> Integer3 {
        Integer3 { x: self.w, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn wzz(self) -> Integer3 {
        Integer3 { x: self.w, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn wzw(self) -> Integer3 {
        Integer3 { x: self.w, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn wwx(self) -> Integer3 {
        Integer3 { x: self.w, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn wwy(self) -> Integer3 {
        Integer3 { x: self.w, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn wwz(self) -> Integer3 {
        Integer3 { x: self.w, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn www(self) -> Integer3 {
        Integer3 { x: self.w, y: self.w, z: self.w }
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
    pub const fn xxxz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xxxw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.x, w: self.w }
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
    pub const fn xxyz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xxyw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn xxzx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xxzy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xxzz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xxzw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xxwx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xxwy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xxwz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xxww(self) -> Integer4 {
        Integer4 { x: self.x, y: self.x, z: self.w, w: self.w }
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
    pub const fn xyxz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xyxw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.x, w: self.w }
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
    pub const fn xyyz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xyyw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn xyzx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xyzy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xyzz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xyzw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xywx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xywy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xywz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xyww(self) -> Integer4 {
        Integer4 { x: self.x, y: self.y, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn xzxx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xzxy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xzxz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xzxw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn xzyx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xzyy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xzyz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xzyw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn xzzx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xzzy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xzzz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xzzw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xzwx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xzwy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xzwz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xzww(self) -> Integer4 {
        Integer4 { x: self.x, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn xwxx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xwxy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xwxz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xwxw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn xwyx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xwyy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xwyz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xwyw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn xwzx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xwzy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xwzz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xwzw(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xwwx(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xwwy(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xwwz(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xwww(self) -> Integer4 {
        Integer4 { x: self.x, y: self.w, z: self.w, w: self.w }
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
    pub const fn yxxz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yxxw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.x, w: self.w }
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
    pub const fn yxyz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yxyw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn yxzx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yxzy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yxzz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yxzw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yxwx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yxwy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yxwz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yxww(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.w, w: self.w }
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
    pub const fn yyxz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yyxw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn yyyx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yyyy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yyyz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yyyw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn yyzx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yyzy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yyzz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yyzw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yywx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yywy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yywz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yyww(self) -> Integer4 {
        Integer4 { x: self.y, y: self.y, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn yzxx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yzxy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yzxz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yzxw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn yzyx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yzyy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yzyz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yzyw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn yzzx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yzzy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yzzz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yzzw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yzwx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yzwy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yzwz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yzww(self) -> Integer4 {
        Integer4 { x: self.y, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn ywxx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn ywxy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn ywxz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn ywxw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn ywyx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn ywyy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn ywyz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn ywyw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn ywzx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn ywzy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn ywzz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn ywzw(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn ywwx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn ywwy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn ywwz(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn ywww(self) -> Integer4 {
        Integer4 { x: self.y, y: self.w, z: self.w, w: self.w }
    }
    

    #[inline]
    pub const fn zxxx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zxxy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zxxz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zxxw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn zxyx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zxyy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zxyz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zxyw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn zxzx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zxzy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zxzz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zxzw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zxwx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zxwy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zxwz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zxww(self) -> Integer4 {
        Integer4 { x: self.z, y: self.x, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn zyxx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zyxy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zyxz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zyxw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn zyyx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zyyy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zyyz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zyyw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn zyzx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zyzy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zyzz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zyzw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zywx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zywy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zywz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zyww(self) -> Integer4 {
        Integer4 { x: self.z, y: self.y, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn zzxx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zzxy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zzxz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zzxw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn zzyx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zzyy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zzyz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zzyw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn zzzx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zzzy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zzzz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zzzw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zzwx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zzwy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zzwz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zzww(self) -> Integer4 {
        Integer4 { x: self.z, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn zwxx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zwxy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zwxz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zwxw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn zwyx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zwyy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zwyz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zwyw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn zwzx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zwzy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zwzz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zwzw(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zwwx(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zwwy(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zwwz(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zwww(self) -> Integer4 {
        Integer4 { x: self.z, y: self.w, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wxxx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wxxy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wxxz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wxxw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wxyx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wxyy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wxyz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wxyw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wxzx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wxzy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wxzz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wxzw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wxwx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wxwy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wxwz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wxww(self) -> Integer4 {
        Integer4 { x: self.w, y: self.x, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wyxx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wyxy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wyxz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wyxw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wyyx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wyyy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wyyz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wyyw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wyzx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wyzy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wyzz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wyzw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wywx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wywy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wywz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wyww(self) -> Integer4 {
        Integer4 { x: self.w, y: self.y, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wzxx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wzxy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wzxz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wzxw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wzyx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wzyy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wzyz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wzyw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wzzx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wzzy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wzzz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wzzw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wzwx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wzwy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wzwz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wzww(self) -> Integer4 {
        Integer4 { x: self.w, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn wwxx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wwxy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wwxz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wwxw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wwyx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wwyy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wwyz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wwyw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wwzx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wwzy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wwzz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wwzw(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wwwx(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wwwy(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wwwz(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wwww(self) -> Integer4 {
        Integer4 { x: self.w, y: self.w, z: self.w, w: self.w }
    }
}

impl_element4!(i32, Integer4);

impl_element4_op!(i32, Integer4);

impl Default for Integer4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<Integer2> for Integer4 {
    #[inline]
    fn from(value: Integer2) -> Self {
        Integer4 { x: value.x, y: value.y, z: 0, w: 0 }
    }
} 

impl From<Integer3> for Integer4 {
    #[inline]
    fn from(value: Integer3) -> Self {
        Integer4 { x: value.x, y: value.y, z: value.z, w: 0 }
    }
}

impl core::ops::Neg for Integer4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x, 
            y: -self.y, 
            z: -self.z, 
            w: -self.w, 
        }
    }
}

impl core::ops::BitAnd<Self> for Integer4 {
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

impl core::ops::BitAndAssign<Self> for Integer4 {
    /// Element-wise bit `AND` operation of two vectors. (assign)
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl core::ops::BitOr<Self> for Integer4 {
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

impl core::ops::BitOrAssign<Self> for Integer4 {
    /// Element-wise bit `OR` operation of two vectors. (assign)
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl core::ops::BitXor<Self> for Integer4 {
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

impl core::ops::BitXorAssign<Self> for Integer4 {
    /// Element-wise bit `XOR` operation of two vectors. (assign)
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl core::ops::Not for Integer4 {
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
