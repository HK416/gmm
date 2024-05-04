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
