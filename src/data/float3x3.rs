use core::fmt;
use core::ops;

use super::bool3::Boolean3;
use super::float3::Float3;
use super::float4x4::Float4x4;

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

    /// Creates with given array.
    /// 
    /// # Panics
    /// If the length of the given array is less than the number of elements in the matrix,
    /// an index out of range error occurs.
    /// 
    #[must_use]
    #[inline(always)]
    pub fn from_column_array(arr: &[f32]) -> Self {
        Self { 
            x_axis: Float3::from_array(&arr[0..3]), 
            y_axis: Float3::from_array(&arr[3..6]), 
            z_axis: Float3::from_array(&arr[6..9]) 
        }
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

impl Default for Float3x3 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Float4x4> for Float3x3 {
    #[inline]
    fn from(value: Float4x4) -> Self {
        Float3x3 { 
            x_axis: Float3::from(value.x_axis), 
            y_axis: Float3::from(value.y_axis),
            z_axis: Float3::from(value.z_axis) 
        }
    }
}

impl AsRef<[f32; 9]> for Float3x3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 9] {
        unsafe { &*(self as *const Self as *const [f32; 9]) }
    }
}

impl AsMut<[f32; 9]> for Float3x3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 9] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 9]) }
    }
}

impl ops::Index<usize> for Float3x3 {
    type Output = Float3;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x_axis,
            1 => &self.y_axis,
            2 => &self.z_axis,
            _ => panic!("index out of range!")
        }
    }
}

impl ops::IndexMut<usize> for Float3x3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            2 => &mut self.z_axis,
            _ => panic!("index out of range!")
        }
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
