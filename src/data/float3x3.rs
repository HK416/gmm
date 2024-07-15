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
    #[inline]
    #[must_use]
    pub const fn from_columns(x_axis: Float3, y_axis: Float3, z_axis: Float3) -> Self {
        Self { x_axis, y_axis, z_axis }
    }

    /// Creates with given array.
    #[inline]
    #[must_use]
    pub fn from_column_array(arr: [f32; 9]) -> Self {
        Self { 
            x_axis: Float3::from_slice(&arr[0..3]), 
            y_axis: Float3::from_slice(&arr[3..6]), 
            z_axis: Float3::from_slice(&arr[6..9]) 
        }
    }

    /// Convert to array.
    #[inline]
    #[must_use]
    pub const fn to_column_array(self) -> [f32; 9] {
        [
            self.x_axis.x, self.x_axis.y, self.x_axis.z, 
            self.y_axis.x, self.y_axis.y, self.y_axis.z, 
            self.z_axis.x, self.z_axis.y, self.z_axis.z 
        ]
    }

    /// Creates with given slice.
    /// 
    /// # Panics
    /// If the length of the given array is less than the number of elements in the matrix,
    /// an index out of range error occurs.
    /// 
    #[inline]
    #[must_use]
    pub fn from_column_slice(slice: &[f32]) -> Self {
        Self { 
            x_axis: Float3::from_slice(&slice[0..3]), 
            y_axis: Float3::from_slice(&slice[3..6]), 
            z_axis: Float3::from_slice(&slice[6..9]), 
        }
    }

    /// Creates with given tuple.
    #[inline]
    #[must_use]
    pub const fn from_column_tuple(tuple: (Float3, Float3, Float3)) -> Self {
        Self { 
            x_axis: tuple.0, 
            y_axis: tuple.1, 
            z_axis: tuple.2 
        }
    }

    /// Convert to tuple.
    #[inline]
    #[must_use]
    pub const fn to_column_tuple(self) -> (Float3, Float3, Float3) {
        (self.x_axis, self.y_axis, self.z_axis)
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

impl From<[f32; 9]> for Float3x3 {
    #[inline]
    fn from(value: [f32; 9]) -> Self {
        Self::from_column_array(value)
    }
}

impl Into<[f32; 9]> for Float3x3 {
    #[inline]
    fn into(self) -> [f32; 9] {
        self.to_column_array()
    }
}

impl From<(Float3, Float3, Float3)> for Float3x3 {
    #[inline]
    fn from(value: (Float3, Float3, Float3)) -> Self {
        Self::from_column_tuple(value)
    }
}

impl Into<(Float3, Float3, Float3)> for Float3x3 {
    #[inline]
    fn into(self) -> (Float3, Float3, Float3) {
        self.to_column_tuple()
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

impl ops::Add<Float3x3> for f32 {
    type Output = Float3x3;
    /// Adds a matrix elements to scalar value.
    #[inline]
    fn add(self, rhs: Float3x3) -> Self::Output {
        Float3x3 {
            x_axis: self + rhs.x_axis, 
            y_axis: self + rhs.y_axis, 
            z_axis: self + rhs.z_axis 
        }
    }
}

impl ops::Add<f32> for Float3x3 {
    type Output = Self;
    /// Adds a scalar value to matrix elements.
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Float3x3 {
            x_axis: self.x_axis + rhs, 
            y_axis: self.y_axis + rhs, 
            z_axis: self.z_axis + rhs 
        }
    }
}

impl ops::AddAssign<f32> for Float3x3 {
    /// Adds a scalar value to matrix elements. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs
    }
}

impl ops::Add<Self> for Float3x3 {
    type Output = Self;
    /// Adds two matrices.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Float3x3 {
            x_axis: self.x_axis + rhs.x_axis, 
            y_axis: self.y_axis + rhs.y_axis, 
            z_axis: self.z_axis + rhs.z_axis 
        }
    }
}

impl ops::AddAssign<Self> for Float3x3 {
    /// Adds two matrices. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub<Float3x3> for f32 {
    type Output = Float3x3;
    /// Subtracts a matrix elements to scalar value.
    #[inline]
    fn sub(self, rhs: Float3x3) -> Self::Output {
        Float3x3 {
            x_axis: self - rhs.x_axis, 
            y_axis: self - rhs.y_axis, 
            z_axis: self - rhs.z_axis 
        }
    }
}

impl ops::Sub<f32> for Float3x3 {
    type Output = Self;
    /// Subtracts a scalar value to matrix elements.
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Float3x3 {
            x_axis: self.x_axis - rhs, 
            y_axis: self.y_axis - rhs, 
            z_axis: self.z_axis - rhs 
        }
    }
}

impl ops::SubAssign<f32> for Float3x3 {
    /// Subtracts a scalar value to matrix elements. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs
    }
}

impl ops::Sub<Self> for Float3x3 {
    type Output = Self;
    /// Subtracts two matrices.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Float3x3 {
            x_axis: self.x_axis - rhs.x_axis, 
            y_axis: self.y_axis - rhs.y_axis, 
            z_axis: self.z_axis - rhs.z_axis 
        }
    }
}

impl ops::SubAssign<Self> for Float3x3 {
    /// Subtracts two matrices. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl ops::Neg for Float3x3 {
    type Output = Self;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        Float3x3 {
            x_axis: -self.x_axis, 
            y_axis: -self.y_axis, 
            z_axis: -self.z_axis 
        }
    }
}

impl ops::Mul<Float3x3> for f32 {
    type Output = Float3x3;
    /// Multiplies a matrix elements to scalar value.
    #[inline]
    fn mul(self, rhs: Float3x3) -> Self::Output {
        Float3x3 {
            x_axis: self * rhs.x_axis, 
            y_axis: self * rhs.y_axis, 
            z_axis: self * rhs.z_axis 
        }
    }
}

impl ops::Mul<f32> for Float3x3 {
    type Output = Self;
    /// Multiplies a scalar value to matrix elements.
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Float3x3 {
            x_axis: self.x_axis * rhs, 
            y_axis: self.y_axis * rhs, 
            z_axis: self.z_axis * rhs 
        }
    }
}

impl ops::MulAssign<f32> for Float3x3 {
    /// Multiplies a scalar value to matrix elements. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl ops::Mul<Float3> for Float3x3 {
    type Output = Float3;
    /// Transformation of the vector.
    fn mul(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self[0][0] * rhs[0] + self[0][1] * rhs[1] + self[0][2] * rhs[2], 
            y: self[1][0] * rhs[0] + self[1][1] * rhs[1] + self[1][2] * rhs[2], 
            z: self[2][0] * rhs[0] + self[2][1] * rhs[1] + self[2][2] * rhs[2] 
        }
    }
}

impl ops::Mul<Self> for Float3x3 {
    type Output = Self;
    /// Multiply two matrices.
    fn mul(self, rhs: Self) -> Self::Output {
        let mut value = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += self[k][j] * rhs[i][k];
                }
                value[i][j] = sum;
            }
        }
        return value;
    }
}

impl ops::MulAssign<Self> for Float3x3 {
    /// Multiply two matrices. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
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
