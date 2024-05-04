use core::fmt;
use core::ops;

use super::bool4::Boolean4;
use super::uint2::UInteger2;
use super::uint3::UInteger3;

#[repr(C)]
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

    #[must_use]
    #[inline(always)]
    pub const fn new(x: u32, y: u32, z: u32, w: u32) -> Self {
        Self { x, y, z, w }
    }

    #[must_use]
    #[inline(always)]
    pub const fn fill(val: u32) -> Self {
        Self { x: val, y: val, z: val, w: val }
    }
}

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

impl AsRef<[u32; 4]> for UInteger4 {
    #[inline]
    fn as_ref(&self) -> &[u32; 4] {
        unsafe { &*(self as *const UInteger4 as *const [u32; 4]) }
    }
}

impl AsMut<[u32; 4]> for UInteger4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u32; 4] {
        unsafe { &mut *(self as *mut UInteger4 as *mut [u32; 4]) }
    }
}

impl ops::Index<usize> for UInteger4 {
    type Output = u32;
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

impl ops::IndexMut<usize> for UInteger4 {
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

impl fmt::Debug for UInteger4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(UInteger4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl fmt::Display for UInteger4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {} }}", &self.x, &self.y, &self.z, &self.w)
    }
}
