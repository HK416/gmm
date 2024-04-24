use core::fmt;

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
