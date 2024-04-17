use core::fmt;
use core::f32;

/// A structure that stores three-dimensional vector data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0.0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1.0);

    /// All elements are negative one.
    pub const NEG_ONE: Self = Self::fill(-1.0);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);

    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);

    /// positive unit vector on z-axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);

    /// negative unit vector on z-axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    /// All elements are [`f32::MIN`].
    pub const MIN: Self = Self::fill(f32::MIN);

    /// All elements are [`f32::MAX`].
    pub const MAX: Self = Self::fill(f32::MAX);

    /// All elements are [`f32::NAN`].
    pub const NAN: Self = Self::fill(f32::NAN);

    /// All elements are [`f32::INFINITY`].
    pub const INFINITY: Self = Self::fill(f32::INFINITY);

    /// All elements are [`f32::NEG_INFINITY`].
    pub const NEG_INFINITY: Self = Self::fill(f32::NEG_INFINITY);

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: f32) -> Self {
        Self { x: val, y: val, z: val }
    }
}

impl fmt::Debug for Float3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Float3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl fmt::Display for Float3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}", &self.x, &self.y, &self.z)
    }
}
