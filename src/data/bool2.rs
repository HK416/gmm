use core::fmt;

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
