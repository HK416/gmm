use core::fmt;
use core::ops;

use super::bool3::Boolean3;
use super::bool4::Boolean4;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean2 {
    pub x: bool,
    pub y: bool,
}

impl Boolean2 {
    /// All elements are `true`.
    pub const TRUE: Self = Self::fill(true);

    /// All elements are `false`.
    pub const FALSE: Self = Self::fill(false);

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: bool) -> Self {
        Self { x: val, y: val }
    }

    /// Returns `true` if any of the elements are `true`.
    #[inline]
    pub const fn any(self) -> bool {
        self.x | self.y
    }

    /// Returns `true` if all the elements are `true`.
    #[inline]
    pub const fn all(self) -> bool {
        self.x & self.y
    }
}

impl Default for Boolean2 {
    #[inline(always)]
    fn default() -> Self {
        Self::FALSE
    }
}

impl From<Boolean3> for Boolean2 {
    #[inline]
    fn from(value: Boolean3) -> Self {
        Boolean2 { x: value.x, y: value.y }
    }
}

impl From<Boolean4> for Boolean2 {
    #[inline]
    fn from(value: Boolean4) -> Self {
        Boolean2 { x: value.x, y: value.y }
    }
}

impl AsRef<[bool; 2]> for Boolean2 {
    #[inline]
    fn as_ref(&self) -> &[bool; 2] {
        unsafe { &*(self as *const Boolean2 as *const [bool; 2]) }
    }
}

impl AsMut<[bool; 2]> for Boolean2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [bool; 2] {
        unsafe { &mut *(self as *mut Boolean2 as *mut [bool; 2]) }
    }
}

impl ops::Index<usize> for Boolean2 {
    type Output = bool;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for Boolean2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of range!"),
        }
    }
}

impl fmt::Debug for Boolean2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Boolean2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl fmt::Display for Boolean2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {} }}", &self.x, &self.y)
    }
}
