use core::fmt;

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
