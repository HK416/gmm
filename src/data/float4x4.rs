use core::fmt;

use super::bool4::Boolean4;
use super::float4::Float4;

/// A structure that stores 4x4 column major matrix data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Float4x4 {
    pub x_axis: Float4,
    pub y_axis: Float4,
    pub z_axis: Float4,
    pub w_axis: Float4,
}

impl Float4x4 {
    /// A 4x4 matrix where all elements of the matrix are zero.
    pub const ZERO: Self = Self::from_columns(Float4::ZERO, Float4::ZERO, Float4::ZERO, Float4::ZERO);

    /// 4x4 identity matrix.
    pub const IDENTITY: Self = Self::from_columns(Float4::X, Float4::Y, Float4::Z, Float4::W);

    /// Creates a 4x4 matrix with given column vectors.
    #[must_use]
    #[inline(always)]
    pub const fn from_columns(x_axis: Float4, y_axis: Float4, z_axis: Float4, w_axis: Float4) -> Self {
        Self { x_axis, y_axis, z_axis, w_axis }
    }

    /// Returns `true` if at least one element of the matrix is [`f32::NAN`].
    #[inline]
    pub fn is_nan(&self) -> bool {
        Boolean4 {
            x: self.x_axis.is_nan(),
            y: self.y_axis.is_nan(),
            z: self.z_axis.is_nan(),
            w: self.w_axis.is_nan()
        }.any()
    }

    /// Returns `true` if at least one element of the matrix is [`f32::INFINITY`].
    #[inline]
    pub fn is_infinite(&self) -> bool {
        Boolean4 {
            x: self.x_axis.is_infinite(),
            y: self.y_axis.is_infinite(),
            z: self.z_axis.is_infinite(),
            w: self.w_axis.is_infinite()
        }.any()
    }
}

impl fmt::Debug for Float4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Float4x4))
            .field(&self.x_axis)
            .field(&self.y_axis)
            .field(&self.z_axis)
            .field(&self.w_axis)
            .finish()
    }
}

impl fmt::Display for Float4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", &self.x_axis, &self.y_axis, &self.z_axis, &self.w_axis)
    }
}
