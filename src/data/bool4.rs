use core::fmt;
use core::ops;

use super::bool2::Boolean2;
use super::bool3::Boolean3;

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
