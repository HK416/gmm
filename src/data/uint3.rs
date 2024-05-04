use core::fmt;
use core::ops;

use super::bool3::Boolean3;
use super::uint2::UInteger2;
use super::uint4::UInteger4;

/// A structure that stores three-dimensional unsigned integer data.
#[repr(C)]
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

    #[must_use]
    #[inline(always)]
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    #[inline(always)]
    pub const fn fill(val: u32) -> Self {
        Self { x: val, y: val, z: val }
    }
}

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

impl AsRef<[u32; 3]> for UInteger3 {
    #[inline]
    fn as_ref(&self) -> &[u32; 3] {
        unsafe { &*(self as *const UInteger3 as *const [u32; 3]) }
    }
}

impl AsMut<[u32; 3]> for UInteger3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u32; 3] {
        unsafe { &mut *(self as *mut UInteger3 as *mut [u32; 3]) }
    }
}

impl ops::Index<usize> for UInteger3 {
    type Output = u32;
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

impl ops::IndexMut<usize> for UInteger3 {
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

impl fmt::Debug for UInteger3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(UInteger3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl fmt::Display for UInteger3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}", &self.x, &self.y, &self.z)
    }
}
