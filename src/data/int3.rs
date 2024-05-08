use core::fmt;
use core::ops;

use super::int2::Integer2;
use super::int4::Integer4;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Integer3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Integer3 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1);
    
    /// All elements are negative one.
    pub const NEG_ONE: Self = Self::fill(-1);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1, 0, 0);

    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0, 1, 0);

    /// positive unit vector on z-axis.
    pub const Z: Self = Self::new(0, 0, 1);

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self::new(-1, 0, 0);

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self::new(0, -1, 0);

    /// negative unit vector on z-axis.
    pub const NEG_Z: Self = Self::new(0, 0, -1);

    /// All elements are [`i32::MIN`].
    pub const MIN: Self = Self::fill(i32::MIN);

    /// All elements are [`i32::MAX`].
    pub const MAX: Self = Self::fill(i32::MAX);

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: i32) -> Self {
        Self { x: val, y: val, z: val }
    }
}

// Vector swizzle code implementation.
impl Integer3 {
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
    pub const fn zx(self) -> Integer2 {
        Integer2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(self) -> Integer2 {
        Integer2 { x: self.z, y: self.y }
    }

    #[inline]
    pub const fn zz(self) -> Integer2 {
        Integer2 { x: self.z, y: self.z }
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
    pub const fn yxyx(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yxyy(self) -> Integer4 {
        Integer4 { x: self.y, y: self.x, z: self.y, w: self.y }
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
}

impl Default for Integer3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<Integer2> for Integer3 {
    #[inline]
    fn from(value: Integer2) -> Self {
        Integer3 { x: value.x, y: value.y, z: 0 }
    }
}

impl From<Integer4> for Integer3 {
    #[inline]
    fn from(value: Integer4) -> Self {
        Integer3 { x: value.x, y: value.y, z: value.z }
    }
}

impl AsRef<[i32; 3]> for Integer3 {
    #[inline]
    fn as_ref(&self) -> &[i32; 3] {
        unsafe { &*(self as *const Integer3 as *const [i32; 3]) }
    }
}

impl AsMut<[i32; 3]> for Integer3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [i32; 3] {
        unsafe { &mut *(self as *mut Integer3 as *mut [i32; 3]) }
    }
}

impl ops::Index<usize> for Integer3 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for Integer3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range!"),
        }
    }
}

impl fmt::Debug for Integer3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Integer3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl fmt::Display for Integer3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}", &self.x, &self.y, &self.z)
    }
}
