use core::fmt;
use core::ops;

use super::int2::Integer2;
use super::int3::Integer3;

#[repr(C)]
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

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: i32) -> Self {
        Self { x: val, y: val, z: val, w: val }
    }
}

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

impl AsRef<[i32; 4]> for Integer4 {
    #[inline]
    fn as_ref(&self) -> &[i32; 4] {
        unsafe { &*(self as *const Integer4 as *const [i32; 4]) }
    }
}

impl AsMut<[i32; 4]> for Integer4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [i32; 4] {
        unsafe { &mut *(self as *mut Integer4 as *mut [i32; 4]) }
    }
}

impl ops::Index<usize> for Integer4 {
    type Output = i32;
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

impl ops::IndexMut<usize> for Integer4 {
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

impl fmt::Debug for Integer4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Integer4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl fmt::Display for Integer4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {} }}", &self.x, &self.y, &self.z, &self.w)
    }
}
