use core::fmt;
use core::ops;

use super::bool2::Boolean2;
use super::bool3::Boolean3;

/// A structure that stores four-dimensional boolean data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean4 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}

impl Boolean4 {
    /// All elements are `true`.
    pub const TRUE: Self = Self::fill(true);

    /// All elements are `false`.
    pub const FALSE: Self = Self::fill(false);

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self { x, y, z, w }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: bool) -> Self {
        Self { x: val, y: val, z: val, w: val }
    }

    /// Creates with given array.
    /// 
    /// # Panics
    /// If the length of the given array is less than the number of elements in the vector,
    /// an index out of range error occurs.
    /// 
    #[must_use]
    #[inline(always)]
    pub fn from_array(arr: &[bool]) -> Self {
        Self { x: arr[0], y: arr[1], z: arr[2], w: arr[3] }
    }

    /// Returns `true` if any of the elements are `true`.
    #[inline]
    pub const fn any(self) -> bool {
        self.x | self.y | self.z | self.w
    }

    /// Returns `true` if all the elements are `true`.
    #[inline]
    pub const fn all(self) -> bool {
        self.x & self.y & self.z & self.w
    }
}

// Vector swizzle code implementation.
impl Boolean4 {
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
    pub const fn xw(self) -> Boolean2 {
        Boolean2 { x: self.x, y: self.w }
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
    pub const fn yw(self) -> Boolean2 {
        Boolean2 { x: self.y, y: self.w }
    }

    #[inline]
    pub const fn zx(self) -> Boolean2 {
        Boolean2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(self) -> Boolean2 {
        Boolean2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zz(self) -> Boolean2 {
        Boolean2 { x: self.z, y: self.z }
    }

    #[inline]
    pub const fn zw(self) -> Boolean2 {
        Boolean2 { x: self.z, y: self.w }
    }

    #[inline]
    pub const fn wx(self) -> Boolean2 {
        Boolean2 { x: self.w, y: self.x }
    }

    #[inline]
    pub const fn wy(self) -> Boolean2 {
        Boolean2 { x: self.w, y: self.y }
    }

    #[inline]
    pub const fn wz(self) -> Boolean2 {
        Boolean2 { x: self.w, y: self.z }
    }

    #[inline]
    pub const fn ww(self) -> Boolean2 {
        Boolean2 { x: self.w, y: self.w }
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
    pub const fn xxw(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.x, z: self.w }
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
    pub const fn xyw(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.y, z: self.w }
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
    pub const fn xzw(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn xwx(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn xwy(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn xwz(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn xww(self) -> Boolean3 {
        Boolean3 { x: self.x, y: self.w, z: self.w }
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
    pub const fn yxw(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.x, z: self.w }
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
    pub const fn yyw(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.y, z: self.w }
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
    pub const fn yzw(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn ywx(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn ywy(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn ywz(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn yww(self) -> Boolean3 {
        Boolean3 { x: self.y, y: self.w, z: self.w }
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
    pub const fn zxw(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.x, z: self.w }
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
    pub const fn zyw(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.y, z: self.w }
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
    pub const fn zzw(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn zwx(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn zwy(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn zwz(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn zww(self) -> Boolean3 {
        Boolean3 { x: self.z, y: self.w, z: self.w }
    }

    #[inline]
    pub const fn wxx(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn wxy(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn wxz(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn wxw(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.x, z: self.w }
    }

    #[inline]
    pub const fn wyx(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn wyy(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn wyz(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn wyw(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.y, z: self.w }
    }

    #[inline]
    pub const fn wzx(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn wzy(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn wzz(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn wzw(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn wwx(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn wwy(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn wwz(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn www(self) -> Boolean3 {
        Boolean3 { x: self.w, y: self.w, z: self.w }
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
    pub const fn xxxw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.x, w: self.w }
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
    pub const fn xxyw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.y, w: self.w }
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
    pub const fn xxzw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xxwx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xxwy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xxwz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xxww(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.x, z: self.w, w: self.w }
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
    pub const fn xyxw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.x, w: self.w }
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
    pub const fn xyyw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.y, w: self.w }
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
    pub const fn xyzw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xywx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xywy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xywz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xyww(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.y, z: self.w, w: self.w }
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
    pub const fn xzxw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.x, w: self.w }
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
    pub const fn xzyw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.y, w: self.w }
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
    pub const fn xzzw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xzwx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xzwy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xzwz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xzww(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn xwxx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xwxy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xwxz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xwxw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn xwyx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xwyy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xwyz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xwyw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn xwzx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xwzy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xwzz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xwzw(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xwwx(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xwwy(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xwwz(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xwww(self) -> Boolean4 {
        Boolean4 { x: self.x, y: self.w, z: self.w, w: self.w }
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
    pub const fn yxxw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.x, w: self.w }
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
    pub const fn yxyz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yxyw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.y, w: self.w }
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
    pub const fn yxzw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yxwx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yxwy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yxwz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yxww(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.x, z: self.w, w: self.w }
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
    pub const fn yyxw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.x, w: self.w }
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
    pub const fn yyyw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.y, w: self.w }
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
    pub const fn yyzw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yywx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yywy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yywz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yyww(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.y, z: self.w, w: self.w }
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
    pub const fn yzxw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.x, w: self.w }
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
    pub const fn yzyw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.y, w: self.w }
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
    pub const fn yzzw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yzwx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yzwy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yzwz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yzww(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn ywxx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn ywxy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn ywxz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn ywxw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn ywyx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn ywyy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn ywyz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn ywyw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn ywzx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn ywzy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn ywzz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn ywzw(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn ywwx(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn ywwy(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn ywwz(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn ywww(self) -> Boolean4 {
        Boolean4 { x: self.y, y: self.w, z: self.w, w: self.w }
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
    pub const fn zxxw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.x, w: self.w }
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
    pub const fn zxyw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.y, w: self.w }
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
    pub const fn zxzw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zxwx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zxwy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zxwz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zxww(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.x, z: self.w, w: self.w }
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
    pub const fn zyxw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.x, w: self.w }
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
    pub const fn zyyw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.y, w: self.w }
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
    pub const fn zyzw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zywx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zywy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zywz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zyww(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.y, z: self.w, w: self.w }
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
    pub const fn zzxw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.x, w: self.w }
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
    pub const fn zzyw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.y, w: self.w }
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

    #[inline]
    pub const fn zzzw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zzwx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zzwy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zzwz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zzww(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn zwxx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zwxy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zwxz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zwxw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn zwyx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zwyy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zwyz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zwyw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn zwzx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zwzy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zwzz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zwzw(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zwwx(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zwwy(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zwwz(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zwww(self) -> Boolean4 {
        Boolean4 { x: self.z, y: self.w, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wxxx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wxxy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wxxz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wxxw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wxyx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wxyy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wxyz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wxyw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wxzx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wxzy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wxzz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wxzw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wxwx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wxwy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wxwz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wxww(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.x, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wyxx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wyxy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wyxz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wyxw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wyyx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wyyy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wyyz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wyyw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wyzx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wyzy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wyzz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wyzw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wywx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wywy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wywz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wyww(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.y, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wzxx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wzxy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wzxz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wzxw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wzyx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wzyy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wzyz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wzyw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wzzx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wzzy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wzzz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wzzw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wzwx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wzwy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wzwz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wzww(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn wwxx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wwxy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wwxz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wwxw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wwyx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wwyy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wwyz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wwyw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wwzx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wwzy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wwzz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wwzw(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wwwx(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wwwy(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wwwz(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wwww(self) -> Boolean4 {
        Boolean4 { x: self.w, y: self.w, z: self.w, w: self.w }
    }
}

impl Default for Boolean4 {
    #[inline(always)]
    fn default() -> Self {
        Self::FALSE
    }
}

impl From<Boolean2> for Boolean4 {
    #[inline]
    fn from(value: Boolean2) -> Self {
        Boolean4 { x: value.x, y: value.y, z: false, w: false }
    }
}

impl From<Boolean3> for Boolean4 {
    #[inline]
    fn from(value: Boolean3) -> Self {
        Boolean4 { x: value.x, y: value.y, z: value.z, w: false }
    }
}

impl AsRef<[bool; 4]> for Boolean4 {
    #[inline]
    fn as_ref(&self) -> &[bool; 4] {
        unsafe { &*(self as *const Boolean4 as *const [bool; 4]) }
    }
}

impl AsMut<[bool; 4]> for Boolean4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [bool; 4] {
        unsafe { &mut *(self as *mut Boolean4 as *mut [bool; 4]) }
    }
}

impl ops::Index<usize> for Boolean4 {
    type Output = bool;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for Boolean4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of range!"),
        }
    }
}

impl fmt::Debug for Boolean4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Boolean4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl fmt::Display for Boolean4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {} }}", &self.x, &self.y, &self.z, &self.w)
    }
}
