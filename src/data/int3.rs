use core::fmt;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Integer3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Integer3 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1);
    
    /// All elements are negative one.
    pub const NEG_ONE: Self = Self::fill(-1);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1, 0, 0);

    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0, 1, 0);

    /// positive unit vector on z-axis.
    pub const Z: Self = Self::new(0, 0, 1);

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self::new(-1, 0, 0);

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self::new(0, -1, 0);

    /// negative unit vector on z-axis.
    pub const NEG_Z: Self = Self::new(0, 0, -1);

    /// All elements are [`i32::MIN`].
    pub const MIN: Self = Self::fill(i32::MIN);

    /// All elements are [`i32::MAX`].
    pub const MAX: Self = Self::fill(i32::MAX);

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: i32) -> Self {
        Self { x: val, y: val, z: val }
    }
}

impl fmt::Debug for Integer3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Integer3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl fmt::Display for Integer3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}", &self.x, &self.y, &self.z)
    }
}
