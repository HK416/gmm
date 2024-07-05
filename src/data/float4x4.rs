use core::fmt;
use core::ops;

use super::bool4::Boolean4;
use super::float3x3::Float3x3;
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
            x_axis: Float4::from_array(&arr[0..4]), 
            y_axis: Float4::from_array(&arr[4..8]), 
            z_axis: Float4::from_array(&arr[8..12]), 
            w_axis: Float4::from_array(&arr[12..16]) 
        }
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

impl Default for Float4x4 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Float3x3> for Float4x4 {
    #[inline]
    fn from(value: Float3x3) -> Self {
        Self { 
            x_axis: Float4::from(value.x_axis), 
            y_axis: Float4::from(value.y_axis), 
            z_axis: Float4::from(value.z_axis), 
            w_axis: Float4::W 
        }
    }
}

impl AsRef<[f32; 16]> for Float4x4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 16] {
        unsafe { &*(self as *const Self as *const [f32; 16]) }
    }
}

impl AsMut<[f32; 16]> for Float4x4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 16] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 16]) }
    }
}

impl ops::Index<usize> for Float4x4 {
    type Output = Float4;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x_axis, 
            1 => &self.y_axis,
            2 => &self.z_axis,
            3 => &self.w_axis,
            _ => panic!("index out of range!")
        }
    }
}

impl ops::IndexMut<usize> for Float4x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            2 => &mut self.z_axis,
            3 => &mut self.w_axis,
            _ => panic!("index out of range!")
        }
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
