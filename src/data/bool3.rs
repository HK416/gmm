use core::fmt;
use core::ops;

use super::bool2::Boolean2;
use super::bool4::Boolean4;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl Boolean3 {
    /// All elements are `true`.
    pub const TRUE: Self = Self::fill(true);

    /// All elements are `false`.
    pub const FALSE: Self = Self::fill(false);

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: bool) -> Self {
        Self { x: val, y: val, z: val }
    }

    /// Returns `true` if any of the elements are `true`.
    #[inline]
    pub const fn any(self) -> bool {
        self.x | self.y | self.z
    }

    /// Returns `true` if all elements are `true`.
    #[inline]
    pub const fn all(self) -> bool {
        self.x & self.y & self.z
    }
}

impl Default for Boolean3 {
    #[inline(always)]
    fn default() -> Self {
        Self::FALSE
    }
}

impl From<Boolean2> for Boolean3 {
    #[inline]
    fn from(value: Boolean2) -> Self {
        Boolean3 { x: value.x, y: value.y, z: false }
    }
}

impl From<Boolean4> for Boolean3 {
    #[inline]
    fn from(value: Boolean4) -> Self {
        Boolean3 { x: value.x, y: value.y, z: value.z }
    }
}

impl AsRef<[bool; 3]> for Boolean3 {
    #[inline]
    fn as_ref(&self) -> &[bool; 3] {
        unsafe { &*(self as *const Boolean3 as *const [bool; 3]) }
    }
}

impl AsMut<[bool; 3]> for Boolean3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [bool; 3] {
        unsafe { &mut *(self as *mut Boolean3 as *mut [bool; 3]) }
    }
}

impl ops::Index<usize> for Boolean3 {
    type Output = bool;
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

impl ops::IndexMut<usize> for Boolean3 {
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

impl fmt::Debug for Boolean3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Boolean3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl fmt::Display for Boolean3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}", &self.x, &self.y, &self.z)
    }
}
