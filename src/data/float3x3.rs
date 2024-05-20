use core::fmt;

use super::bool3::Boolean3;
use super::float3::Float3;

/// A structure that stores 3x3 column major matrix data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Float3x3 {
    pub x_axis: Float3,
    pub y_axis: Float3,
    pub z_axis: Float3
}

impl Float3x3 {
    /// A 3x3 matrix where all elements of the matrix are zero.
    pub const ZERO: Self = Self::from_columns(Float3::ZERO, Float3::ZERO, Float3::ZERO);

    /// 3x3 identity matrix.
    pub const IDENTITY: Self = Self::from_columns(Float3::X, Float3::Y, Float3::Z);

    /// Creates a 3x3 matrix with given column vectors.
    #[must_use]
    #[inline(always)]
    pub const fn from_columns(x_axis: Float3, y_axis: Float3, z_axis: Float3) -> Self {
        Self { x_axis, y_axis, z_axis }
    }

    /// Returns `true` if at least one element of the matrix is [`f32::NAN`].
    #[inline]
    pub fn is_nan(&self) -> bool {
        Boolean3 {
            x: self.x_axis.is_nan(),
            y: self.y_axis.is_nan(),
            z: self.z_axis.is_nan()
        }.any()
    }

    /// Returns `true` if at least one element of the matrix is [`f32::INFINITY`].
    #[inline]
    pub fn is_infinite(&self) -> bool {
        Boolean3 {
            x: self.x_axis.is_infinite(),
            y: self.y_axis.is_infinite(),
            z: self.z_axis.is_infinite()
        }.any()
    }
}

impl fmt::Debug for Float3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Float3x3))
            .field(&self.x_axis)
            .field(&self.y_axis)
            .field(&self.z_axis)
            .finish()
    }
}

impl fmt::Display for Float3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", &self.x_axis, &self.y_axis, &self.z_axis)
    }
}
