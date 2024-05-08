use core::fmt;
use core::ops;

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

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: i32) -> Self {
        Self { x: val, y: val }
    }
}

impl Default for Integer2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

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

impl AsRef<[i32; 2]> for Integer2 {
    #[inline]
    fn as_ref(&self) -> &[i32; 2] {
        unsafe { &*(self as *const Integer2 as *const [i32; 2]) }
    }
}

impl AsMut<[i32; 2]> for Integer2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [i32; 2] {
        unsafe { &mut *(self as *mut Integer2 as *mut [i32; 2]) }
    }
}

impl ops::Index<usize> for Integer2 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for Integer2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of range!"),
        }
    }
}

impl fmt::Debug for Integer2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Integer2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl fmt::Display for Integer2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {} }}", &self.x, &self.y)
    }
}
