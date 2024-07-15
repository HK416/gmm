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
    #[inline]
    #[must_use]
    pub const fn from_columns(x_axis: Float4, y_axis: Float4, z_axis: Float4, w_axis: Float4) -> Self {
        Self { x_axis, y_axis, z_axis, w_axis }
    }

    /// Creates with given array.
    #[inline]
    #[must_use]
    pub fn from_column_array(arr: [f32; 16]) -> Self {
        Self { 
            x_axis: Float4::from_slice(&arr[0..4]), 
            y_axis: Float4::from_slice(&arr[4..8]), 
            z_axis: Float4::from_slice(&arr[8..12]), 
            w_axis: Float4::from_slice(&arr[12..16]) 
        }
    }

    /// Convert to array.
    #[inline]
    #[must_use]
    pub const fn to_column_array(self) -> [f32; 16] {
        [
            self.x_axis.x, self.x_axis.y, self.x_axis.z, self.x_axis.w, 
            self.y_axis.x, self.y_axis.y, self.y_axis.z, self.y_axis.w, 
            self.z_axis.x, self.z_axis.y, self.z_axis.z, self.z_axis.w, 
            self.w_axis.x, self.w_axis.y, self.w_axis.z, self.w_axis.w 
        ]
    }

    /// Creates with given array.
    /// 
    /// # Panics
    /// If the length of the given array is less than the number of elements in the matrix,
    /// an index out of range error occurs.
    /// 
    #[inline]
    #[must_use]
    pub fn from_column_slice(slice: &[f32]) -> Self {
        Self { 
            x_axis: Float4::from_slice(&slice[0..4]), 
            y_axis: Float4::from_slice(&slice[4..8]), 
            z_axis: Float4::from_slice(&slice[8..12]), 
            w_axis: Float4::from_slice(&slice[12..16]) 
        }
    }

    /// Creates with given tuple.
    #[inline]
    #[must_use]
    pub const fn from_column_tuple(tuple: (Float4, Float4, Float4, Float4)) -> Self {
        Self { 
            x_axis: tuple.0, 
            y_axis: tuple.1, 
            z_axis: tuple.2, 
            w_axis: tuple.3 
        }
    }

    /// Convert to tuple.
    #[inline]
    #[must_use]
    pub const fn to_column_tuple(self) -> (Float4, Float4, Float4, Float4) {
        (self.x_axis, self.y_axis, self.z_axis, self.w_axis)
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

impl From<[f32; 16]> for Float4x4 {
    #[inline]
    fn from(value: [f32; 16]) -> Self {
        Self::from_column_array(value)
    }
}

impl Into<[f32; 16]> for Float4x4 {
    #[inline]
    fn into(self) -> [f32; 16] {
        self.to_column_array()
    }
}

impl From<(Float4, Float4, Float4, Float4)> for Float4x4 {
    #[inline]
    fn from(value: (Float4, Float4, Float4, Float4)) -> Self {
        Self::from_column_tuple(value)
    }
}

impl Into<(Float4, Float4, Float4, Float4)> for Float4x4 {
    #[inline]
    fn into(self) -> (Float4, Float4, Float4, Float4) {
        self.to_column_tuple()
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

impl ops::Add<Float4x4> for f32 {
    type Output = Float4x4;
    /// Adds a matrix elements to scalar value.
    #[inline]
    fn add(self, rhs: Float4x4) -> Self::Output {
        Float4x4 {
            x_axis: self + rhs.x_axis, 
            y_axis: self + rhs.y_axis, 
            z_axis: self + rhs.z_axis, 
            w_axis: self + rhs.w_axis 
        }
    }
}

impl ops::Add<f32> for Float4x4 {
    type Output = Self;
    /// Adds a scalar value to matrix elements.
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Float4x4 {
            x_axis: self.x_axis + rhs, 
            y_axis: self.y_axis + rhs, 
            z_axis: self.z_axis + rhs, 
            w_axis: self.w_axis + rhs 
        }
    }
}

impl ops::AddAssign<f32> for Float4x4 {
    /// Adds a scalar value to matrix elements. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs
    }
}

impl ops::Add<Self> for Float4x4 {
    type Output = Self;
    /// Adds two matrices.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Float4x4 {
            x_axis: self.x_axis + rhs.x_axis, 
            y_axis: self.y_axis + rhs.y_axis, 
            z_axis: self.z_axis + rhs.z_axis, 
            w_axis: self.w_axis + rhs.w_axis 
        }
    }
}

impl ops::AddAssign<Self> for Float4x4 {
    /// Adds two matrices. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub<Float4x4> for f32 {
    type Output = Float4x4;
    /// Subtracts a matrix elements to scalar value.
    #[inline]
    fn sub(self, rhs: Float4x4) -> Self::Output {
        Float4x4 {
            x_axis: self - rhs.x_axis, 
            y_axis: self - rhs.y_axis, 
            z_axis: self - rhs.z_axis, 
            w_axis: self - rhs.w_axis
        }
    }
}

impl ops::Sub<f32> for Float4x4 {
    type Output = Self;
    /// Subtracts a scalar value to matrix elements.
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Float4x4 {
            x_axis: self.x_axis - rhs, 
            y_axis: self.y_axis - rhs, 
            z_axis: self.z_axis - rhs, 
            w_axis: self.w_axis - rhs 
        }
    }
}

impl ops::SubAssign<f32> for Float4x4 {
    /// Subtracts a scalar value to matrix elements. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs
    }
}

impl ops::Sub<Self> for Float4x4 {
    type Output = Self;
    /// Subtracts two matrices.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Float4x4 {
            x_axis: self.x_axis - rhs.x_axis, 
            y_axis: self.y_axis - rhs.y_axis, 
            z_axis: self.z_axis - rhs.z_axis, 
            w_axis: self.w_axis - rhs.w_axis 
        }
    }
}

impl ops::SubAssign<Self> for Float4x4 {
    /// Subtracts two matrices. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl ops::Neg for Float4x4 {
    type Output = Self;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        Float4x4 {
            x_axis: -self.x_axis, 
            y_axis: -self.y_axis, 
            z_axis: -self.z_axis, 
            w_axis: -self.w_axis 
        }
    }
}

impl ops::Mul<Float4x4> for f32 {
    type Output = Float4x4;
    /// Multiplies a matrix elements to scalar value.
    #[inline]
    fn mul(self, rhs: Float4x4) -> Self::Output {
        Float4x4 {
            x_axis: self * rhs.x_axis, 
            y_axis: self * rhs.y_axis, 
            z_axis: self * rhs.z_axis, 
            w_axis: self * rhs.w_axis 
        }
    }
}

impl ops::Mul<f32> for Float4x4 {
    type Output = Self;
    /// Multiplies a scalar value to matrix elements.
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Float4x4 {
            x_axis: self.x_axis * rhs, 
            y_axis: self.y_axis * rhs, 
            z_axis: self.z_axis * rhs, 
            w_axis: self.w_axis * rhs 
        }
    }
}

impl ops::MulAssign<f32> for Float4x4 {
    /// Multiplies a scalar value to matrix elements. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl ops::Mul<Float4> for Float4x4 {
    type Output = Float4;
    /// Transformation of the vector.
    fn mul(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self[0][0] * rhs[0] + self[0][1] * rhs[1] + self[0][2] * rhs[2] + self[0][3] * rhs[3], 
            y: self[1][0] * rhs[0] + self[1][1] * rhs[1] + self[1][2] * rhs[2] + self[1][3] * rhs[3], 
            z: self[2][0] * rhs[0] + self[2][1] * rhs[1] + self[2][2] * rhs[2] + self[2][3] * rhs[3], 
            w: self[3][0] * rhs[0] + self[3][1] * rhs[1] + self[3][2] * rhs[2] + self[3][3] * rhs[3]  
        }
    }
}

impl ops::Mul<Self> for Float4x4 {
    type Output = Self;
    /// Multiply two matrices.
    fn mul(self, rhs: Self) -> Self::Output {
        let mut value = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self[k][j] * rhs[i][k];
                }
                value[i][j] = sum;
            }
        }
        return value;
    }
}

impl ops::MulAssign<Self> for Float4x4 {
    /// Multiply two matrices. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
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
