use core::fmt;

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
