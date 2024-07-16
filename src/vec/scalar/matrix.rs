use core::fmt;
use core::ops;
use crate::{ Vector, Float3x3, Float4, Float4x4 };



/// This is a matrix data type that uses the `SIMD` instruction.
/// 
/// Using the `scalar-math` feature disables the use of `SIMD` instructions.
/// 
/// It is recommended not to use this data types as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Matrix(pub(crate) Float4x4);

impl Matrix {
    /// Transpose of a matrix.
    pub fn transpose(self) -> Self {
        // Origin:
        // m00 m01 m02 m03 
        // m10 m11 m12 m13
        // m20 m21 m22 m23
        // m30 m31 m32 m33
        // 
        Float4x4 {
            x_axis: Float4 { x: self[0][0], y: self[1][0], z: self[2][0], w: self[3][0] }, 
            y_axis: Float4 { x: self[0][1], y: self[1][1], z: self[2][1], w: self[3][1] }, 
            z_axis: Float4 { x: self[0][2], y: self[1][2], z: self[2][2], w: self[3][2] }, 
            w_axis: Float4 { x: self[0][3], y: self[1][3], z: self[2][3], w: self[3][3] } 
        }.into()
    }

    /// Determinant of a matrix.
    pub fn determinant(self) -> Vector {
        // Reference: glm/detail/func_matrix.inl
        let fac0 = Float4::new(
            self[2][2]*self[3][3] - self[3][2]*self[2][3], 
            self[2][2]*self[3][3] - self[3][2]*self[2][3], 
            self[1][2]*self[3][3] - self[3][2]*self[1][3], 
            self[1][2]*self[2][3] - self[2][2]*self[1][3]
        );
        let fac1 = Float4::new(
            self[2][1]*self[3][3] - self[3][1]*self[2][3], 
            self[2][1]*self[3][3] - self[3][1]*self[2][3], 
            self[1][1]*self[3][3] - self[3][1]*self[1][3], 
            self[1][1]*self[2][3] - self[2][1]*self[1][3]
        );
        let fac2 = Float4::new(
            self[2][1]*self[3][2] - self[3][1]*self[2][2], 
            self[2][1]*self[3][2] - self[3][1]*self[2][2], 
            self[1][1]*self[3][2] - self[3][1]*self[1][2], 
            self[1][1]*self[2][2] - self[2][1]*self[1][2]
        );
        let fac3 = Float4::new(
            self[2][0]*self[3][3] - self[3][0]*self[2][3], 
            self[2][0]*self[3][3] - self[3][0]*self[2][3], 
            self[1][0]*self[3][3] - self[3][0]*self[1][3], 
            self[1][0]*self[2][3] - self[2][0]*self[1][3], 
        );
        let fac4 = Float4::new(
            self[2][0]*self[3][2] - self[3][0]*self[2][2], 
            self[2][0]*self[3][2] - self[3][0]*self[2][2], 
            self[1][0]*self[3][2] - self[3][0]*self[1][2], 
            self[1][0]*self[2][2] - self[2][0]*self[1][2]
        );
        let fac5 = Float4::new(
            self[2][0]*self[3][1] - self[3][0]*self[2][1], 
            self[2][0]*self[3][1] - self[3][0]*self[2][1], 
            self[1][0]*self[3][1] - self[3][0]*self[1][1], 
            self[1][0]*self[2][1] - self[2][0]*self[1][1]
        );


        let vec0 = Float4::new(self[1][0], self[0][0], self[0][0], self[0][0]);
        let vec1 = Float4::new(self[1][1], self[0][1], self[0][1], self[0][1]);
        let vec2 = Float4::new(self[1][2], self[0][2], self[0][2], self[0][2]);
        let vec3 = Float4::new(self[1][3], self[0][3], self[0][3], self[0][3]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let one_neg_one_neg = Float4::new(1.0, -1.0, 1.0, -1.0);
        let neg_one_neg_one = Float4::new(-1.0, 1.0, -1.0, 1.0);
        let inverse = Float4x4 {
            x_axis: inv0 * one_neg_one_neg, 
            y_axis: inv1 * neg_one_neg_one, 
            z_axis: inv2 * one_neg_one_neg, 
            w_axis: inv3 * neg_one_neg_one
        };

        let row0 = Float4::new(inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);
        let det = self[0] * row0;
        let det = det[0] + det[1] + det[2] + det[3];
        return Float4::fill(det).into();
    }

    /// Inverse of a matrix.
    /// If the inverse of matrix cannot be calculated, returns `None`.
    pub fn inverse(self) -> Option<Self> {
        // Reference: glm/detail/func_matrix.inl
        let fac0 = Float4::new(
            self[2][2]*self[3][3] - self[3][2]*self[2][3], 
            self[2][2]*self[3][3] - self[3][2]*self[2][3], 
            self[1][2]*self[3][3] - self[3][2]*self[1][3], 
            self[1][2]*self[2][3] - self[2][2]*self[1][3]
        );
        let fac1 = Float4::new(
            self[2][1]*self[3][3] - self[3][1]*self[2][3], 
            self[2][1]*self[3][3] - self[3][1]*self[2][3], 
            self[1][1]*self[3][3] - self[3][1]*self[1][3], 
            self[1][1]*self[2][3] - self[2][1]*self[1][3]
        );
        let fac2 = Float4::new(
            self[2][1]*self[3][2] - self[3][1]*self[2][2], 
            self[2][1]*self[3][2] - self[3][1]*self[2][2], 
            self[1][1]*self[3][2] - self[3][1]*self[1][2], 
            self[1][1]*self[2][2] - self[2][1]*self[1][2]
        );
        let fac3 = Float4::new(
            self[2][0]*self[3][3] - self[3][0]*self[2][3], 
            self[2][0]*self[3][3] - self[3][0]*self[2][3], 
            self[1][0]*self[3][3] - self[3][0]*self[1][3], 
            self[1][0]*self[2][3] - self[2][0]*self[1][3], 
        );
        let fac4 = Float4::new(
            self[2][0]*self[3][2] - self[3][0]*self[2][2], 
            self[2][0]*self[3][2] - self[3][0]*self[2][2], 
            self[1][0]*self[3][2] - self[3][0]*self[1][2], 
            self[1][0]*self[2][2] - self[2][0]*self[1][2]
        );
        let fac5 = Float4::new(
            self[2][0]*self[3][1] - self[3][0]*self[2][1], 
            self[2][0]*self[3][1] - self[3][0]*self[2][1], 
            self[1][0]*self[3][1] - self[3][0]*self[1][1], 
            self[1][0]*self[2][1] - self[2][0]*self[1][1]
        );


        let vec0 = Float4::new(self[1][0], self[0][0], self[0][0], self[0][0]);
        let vec1 = Float4::new(self[1][1], self[0][1], self[0][1], self[0][1]);
        let vec2 = Float4::new(self[1][2], self[0][2], self[0][2], self[0][2]);
        let vec3 = Float4::new(self[1][3], self[0][3], self[0][3], self[0][3]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let one_neg_one_neg = Float4::new(1.0, -1.0, 1.0, -1.0);
        let neg_one_neg_one = Float4::new(-1.0, 1.0, -1.0, 1.0);
        let inverse = Float4x4 {
            x_axis: inv0 * one_neg_one_neg, 
            y_axis: inv1 * neg_one_neg_one, 
            z_axis: inv2 * one_neg_one_neg, 
            w_axis: inv3 * neg_one_neg_one
        };

        let row0 = Float4::new(inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);
        let det = self[0] * row0;
        let det = det[0] + det[1] + det[2] + det[3];
        if det.abs() <= f32::EPSILON {
            return None;
        }

        let recip_det = det.recip();
        Some(Float4x4 {
            x_axis: inverse[0] * recip_det, 
            y_axis: inverse[1] * recip_det, 
            z_axis: inverse[2] * recip_det, 
            w_axis: inverse[3] * recip_det
        }.into())
    }
}

impl From<[f32; 16]> for Matrix {
    #[inline]
    fn from(value: [f32; 16]) -> Self {
        Self::from(Float4x4::from(value))
    }
}

impl Into<[f32; 16]> for Matrix {
    #[inline]
    fn into(self) -> [f32; 16] {
        let value: Float4x4 = self.into();
        value.into()
    }
}

impl From<Float3x3> for Matrix {
    #[inline]
    fn from(value: Float3x3) -> Self {
        Self::from(Float4x4::from(value))
    }
}

impl Into<Float3x3> for Matrix {
    #[inline]
    fn into(self) -> Float3x3 {
        let value: Float4x4 = self.into();
        return value.into();
    }
}

impl From<Float4x4> for Matrix {
    #[inline]
    fn from(value: Float4x4) -> Self {
        Self(value)
    }
}

impl Into<Float4x4> for Matrix {
    #[inline]
    fn into(self) -> Float4x4 {
        *self
    }
}

impl ops::Deref for Matrix {
    type Target = Float4x4;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Matrix {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ops::Add<Self> for Matrix {
    type Output = Self;
    /// Adds two matrices.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Matrix(*self + *rhs)
    }
}

impl ops::AddAssign<Self> for Matrix {
    /// Adds two matrices. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub<Self> for Matrix {
    type Output = Self;
    /// Subtracts two matrices.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix(*self - *rhs)
    }
}

impl ops::SubAssign<Self> for Matrix {
    /// Subtracts two matrices. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl ops::Neg for Matrix {
    type Output = Self;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        Matrix(-*self)
    }
}

impl ops::Mul<Vector> for Matrix {
    type Output = Vector;
    /// Transformation of the vector.
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(*self * *rhs)
    }
}

impl ops::Mul<Self> for Matrix {
    type Output = Self;
    /// Multiplies two matrices.
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix(*self * *rhs)
    }
}

impl ops::MulAssign<Self> for Matrix {
    /// Multiplies two matrices. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl fmt::Debug for Matrix {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Matrix))
            .field(&self[0])
            .field(&self[1])
            .field(&self[2])
            .field(&self[3])
            .finish()
    }
}
