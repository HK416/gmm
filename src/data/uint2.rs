use core::fmt;
use core::ops;

use super::bool2::Boolean2;
use super::uint3::UInteger3;
use super::uint4::UInteger4;

/// A structure that stores two-dimensional unsigned integer data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UInteger2 {
    pub x: u32,
    pub y: u32,
}

impl UInteger2 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1, 0);
    
    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0, 1);

    /// All elements are [`u32::MIN`].
    pub const MIN: Self = Self::fill(u32::MIN);

    /// All elements are [`u32::MAX`].
    pub const MAX: Self = Self::fill(u32::MAX);

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: u32) -> Self {
        Self { x: val, y: val }
    }

    /// Creates with given array.
    /// 
    /// # Panics
    /// If the length of the given array is less than the number of elements in the vector,
    /// an index out of range error occurs.
    /// 
    #[must_use]
    #[inline(always)]
    pub fn from_array(arr: &[u32]) -> Self {
        Self { x: arr[0], y: arr[1] }
    }
}

// Vector swizzle code implementation.
impl UInteger2 {
    #[inline]
    pub const fn xx(self) -> UInteger2 {
        UInteger2 { x: self.x, y: self.x }
    }

    #[inline]
    pub const fn xy(self) -> UInteger2 {
        UInteger2 { x: self.x, y: self.y }
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
    pub const fn xxx(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn xxy(self) -> UInteger3 {
        UInteger3 { x: self.x, y: self.x, z: self.y }
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
    pub const fn yxx(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn yxy(self) -> UInteger3 {
        UInteger3 { x: self.y, y: self.x, z: self.y }
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
    pub const fn xxxx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xxxy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.x, z: self.x, w: self.y }
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
    pub const fn xyxx(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xyxy(self) -> UInteger4 {
        UInteger4 { x: self.x, y: self.y, z: self.x, w: self.y }
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
    pub const fn yxxx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yxxy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.x, z: self.x, w: self.y }
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
    pub const fn yyxx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yyxy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yyyx(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yyyy(self) -> UInteger4 {
        UInteger4 { x: self.y, y: self.y, z: self.y, w: self.y }
    }
}

impl Default for UInteger2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Into<Boolean2> for UInteger2 {
    #[inline]
    fn into(self) -> Boolean2 {
        Boolean2 { 
            x: self.x == 0xFFFFFFFF, 
            y: self.y == 0xFFFFFFFF 
        }
    }
}

impl From<Boolean2> for UInteger2 {
    #[inline]
    fn from(value: Boolean2) -> Self {
        UInteger2 { 
            x: if value.x { 0xFFFFFFFF } else { 0 }, 
            y: if value.y { 0xFFFFFFFF } else { 0 } 
        }
    }
}

impl From<UInteger3> for UInteger2 {
    #[inline]
    fn from(value: UInteger3) -> Self {
        UInteger2 { x: value.x, y: value.y }
    }
}

impl From<UInteger4> for UInteger2 {
    #[inline]
    fn from(value: UInteger4) -> Self {
        UInteger2 { x: value.x, y: value.y }
    }
}

impl AsRef<[u32; 2]> for UInteger2 {
    #[inline]
    fn as_ref(&self) -> &[u32; 2] {
        unsafe { &*(self as *const UInteger2 as *const [u32; 2]) }
    }
}

impl AsMut<[u32; 2]> for UInteger2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u32; 2] {
        unsafe { &mut *(self as *mut UInteger2 as *mut [u32; 2]) }
    }
}

impl ops::Index<usize> for UInteger2 {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for UInteger2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of range!"),
        }
    }
}

impl fmt::Debug for UInteger2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(UInteger2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl fmt::Display for UInteger2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {} }}", &self.x, &self.y)
    }
}
