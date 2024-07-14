use core::fmt;
use core::ops;

#[cfg(target_pointer_width = "32")]
use core::arch::x86::*;

#[cfg(target_pointer_width = "64")]
use core::arch::x86_64::*;

use crate::Float3x3;
use crate::Float4x4;

use super::Vector;



/// This is a matrix data type that uses the `SIMD` instruction.
/// 
/// It is recommended not to use this data types as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Matrix([__m128; 4]);

impl Matrix {
    /// Transpose of a matrix.
    pub fn transpose(self) -> Self {
        // Origin:
        // m00 m01 m02 m03 
        // m10 m11 m12 m13
        // m20 m21 m22 m23
        // m30 m31 m32 m33
        // 
        unsafe {
            let m00_m01_m10_m11 = _mm_shuffle_ps::<0b_01_00_01_00>(self[0], self[1]);
            let m20_m21_m30_m31 = _mm_shuffle_ps::<0b_01_00_01_00>(self[2], self[3]);
            let m02_m03_m12_m13 = _mm_shuffle_ps::<0b_11_10_11_10>(self[0], self[1]);
            let m22_m23_m32_m33 = _mm_shuffle_ps::<0b_11_10_11_10>(self[2], self[3]);

            let col0 = _mm_shuffle_ps::<0b_10_00_10_00>(m00_m01_m10_m11, m20_m21_m30_m31); 
            let col1 = _mm_shuffle_ps::<0b_11_01_11_01>(m00_m01_m10_m11, m20_m21_m30_m31); 
            let col2 = _mm_shuffle_ps::<0b_10_00_10_00>(m02_m03_m12_m13, m22_m23_m32_m33); 
            let col3 = _mm_shuffle_ps::<0b_11_01_11_01>(m02_m03_m12_m13, m22_m23_m32_m33); 

            Matrix([col0, col1, col2, col3])
        }
    }

    /// Determinant of a matrix.
    pub fn determinant(self) -> Vector {
        // Reference: glm/detail/func_matrix.inl
        const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
        const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];
        unsafe {
            let m20_m20_m10_m10 = _mm_shuffle_ps::<0b_00_00_00_00>(self[2], self[1]);
            let m21_m21_m11_m11 = _mm_shuffle_ps::<0b_01_01_01_01>(self[2], self[1]);
            let m22_m22_m12_m12 = _mm_shuffle_ps::<0b_10_10_10_10>(self[2], self[1]);
            let m23_m23_m13_m13 = _mm_shuffle_ps::<0b_11_11_11_11>(self[2], self[1]);
            let m30_m30_m20_m20 = _mm_shuffle_ps::<0b_00_00_00_00>(self[3], self[2]);
            let m31_m31_m21_m21 = _mm_shuffle_ps::<0b_01_01_01_01>(self[3], self[2]);
            let m32_m32_m22_m22 = _mm_shuffle_ps::<0b_10_10_10_10>(self[3], self[2]);
            let m33_m33_m23_m23 = _mm_shuffle_ps::<0b_11_11_11_11>(self[3], self[2]);
    
            let m30_m30_m30_m20 = _mm_shuffle_ps::<0b_10_00_00_00>(m30_m30_m20_m20, m30_m30_m20_m20);
            let m31_m31_m31_m21 = _mm_shuffle_ps::<0b_10_00_00_00>(m31_m31_m21_m21, m31_m31_m21_m21);
            let m32_m32_m32_m22 = _mm_shuffle_ps::<0b_10_00_00_00>(m32_m32_m22_m22, m32_m32_m22_m22);
            let m33_m33_m33_m23 = _mm_shuffle_ps::<0b_10_00_00_00>(m33_m33_m23_m23, m33_m33_m23_m23);
    
    
            let a = _mm_mul_ps(m22_m22_m12_m12, m33_m33_m33_m23);
            let b = _mm_mul_ps(m32_m32_m32_m22, m23_m23_m13_m13);
            let fac0 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m21_m21_m11_m11, m33_m33_m33_m23);
            let b = _mm_mul_ps(m31_m31_m31_m21, m23_m23_m13_m13);
            let fac1 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m21_m21_m11_m11, m32_m32_m32_m22);
            let b = _mm_mul_ps(m31_m31_m31_m21, m22_m22_m12_m12);
            let fac2 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m20_m20_m10_m10, m33_m33_m33_m23);
            let b = _mm_mul_ps(m30_m30_m30_m20, m23_m23_m13_m13);
            let fac3 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m20_m20_m10_m10, m32_m32_m32_m22);
            let b = _mm_mul_ps(m30_m30_m30_m20, m22_m22_m12_m12);
            let fac4 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m20_m20_m10_m10, m31_m31_m31_m21);
            let b = _mm_mul_ps(m30_m30_m30_m20, m21_m21_m11_m11);
            let fac5 = _mm_sub_ps(a, b);
    
    
            let m00_m01_m10_m11 = _mm_shuffle_ps::<0b_01_00_01_00>(self[0], self[1]);
            let m02_m03_m12_m13 = _mm_shuffle_ps::<0b_11_10_11_10>(self[0], self[1]);
            let vec0 = _mm_shuffle_ps::<0b_00_00_00_10>(m00_m01_m10_m11, m00_m01_m10_m11);
            let vec1 = _mm_shuffle_ps::<0b_01_01_01_11>(m00_m01_m10_m11, m00_m01_m10_m11);
            let vec2 = _mm_shuffle_ps::<0b_00_00_00_10>(m02_m03_m12_m13, m02_m03_m12_m13);
            let vec3 = _mm_shuffle_ps::<0b_01_01_01_11>(m02_m03_m12_m13, m02_m03_m12_m13);
    
    
            let inv0 = _mm_add_ps(_mm_sub_ps(_mm_mul_ps(vec1, fac0), _mm_mul_ps(vec2, fac1)), _mm_mul_ps(vec3, fac2));
            let inv1 = _mm_add_ps(_mm_sub_ps(_mm_mul_ps(vec0, fac0), _mm_mul_ps(vec2, fac3)), _mm_mul_ps(vec3, fac4));
            let inv2 = _mm_add_ps(_mm_sub_ps(_mm_mul_ps(vec0, fac1), _mm_mul_ps(vec1, fac3)), _mm_mul_ps(vec3, fac5));
            let inv3 = _mm_add_ps(_mm_sub_ps(_mm_mul_ps(vec0, fac2), _mm_mul_ps(vec1, fac4)), _mm_mul_ps(vec2, fac5));
    
    
            let one_neg_one_neg = _mm_load_ps(&ONE_NEG_ONE_NEG as *const f32);
            let neg_one_neg_one = _mm_load_ps(&NEG_ONE_NEG_ONE as *const f32);
            let inverse = [
                _mm_mul_ps(inv0, one_neg_one_neg), 
                _mm_mul_ps(inv1, neg_one_neg_one), 
                _mm_mul_ps(inv2, one_neg_one_neg), 
                _mm_mul_ps(inv3, neg_one_neg_one) 
            ];
    
            let m00_m00_m10_m10 = _mm_shuffle_ps::<0b_00_00_00_00>(inverse[0], inverse[1]);
            let m20_m20_m30_m30 = _mm_shuffle_ps::<0b_00_00_00_00>(inverse[2], inverse[3]);
            let row0 = _mm_shuffle_ps::<0b_10_00_10_00>(m00_m00_m10_m10, m20_m20_m30_m30);
            let det = _mm_mul_ps(self[0], row0);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(det, det);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(det, det);
            let sum = _mm_add_ps(low, high);
            let mix = _mm_shuffle_ps::<0b_10_11_00_01>(sum, sum);
            let sum = _mm_add_ps(sum, mix);

            return Vector(sum);
        }
    }

    /// Inverse of a matrix.
    /// If the inverse of matrix cannot be calculated, returns `None`.
    pub fn inverse(self) -> Option<Self> {
        // Reference: glm/detail/func_matrix.inl
        const ONE_ONE_ONE_ONE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
        const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];
        unsafe {
            let m20_m20_m10_m10 = _mm_shuffle_ps::<0b_00_00_00_00>(self[2], self[1]);
            let m21_m21_m11_m11 = _mm_shuffle_ps::<0b_01_01_01_01>(self[2], self[1]);
            let m22_m22_m12_m12 = _mm_shuffle_ps::<0b_10_10_10_10>(self[2], self[1]);
            let m23_m23_m13_m13 = _mm_shuffle_ps::<0b_11_11_11_11>(self[2], self[1]);
            let m30_m30_m20_m20 = _mm_shuffle_ps::<0b_00_00_00_00>(self[3], self[2]);
            let m31_m31_m21_m21 = _mm_shuffle_ps::<0b_01_01_01_01>(self[3], self[2]);
            let m32_m32_m22_m22 = _mm_shuffle_ps::<0b_10_10_10_10>(self[3], self[2]);
            let m33_m33_m23_m23 = _mm_shuffle_ps::<0b_11_11_11_11>(self[3], self[2]);
    
            let m30_m30_m30_m20 = _mm_shuffle_ps::<0b_10_00_00_00>(m30_m30_m20_m20, m30_m30_m20_m20);
            let m31_m31_m31_m21 = _mm_shuffle_ps::<0b_10_00_00_00>(m31_m31_m21_m21, m31_m31_m21_m21);
            let m32_m32_m32_m22 = _mm_shuffle_ps::<0b_10_00_00_00>(m32_m32_m22_m22, m32_m32_m22_m22);
            let m33_m33_m33_m23 = _mm_shuffle_ps::<0b_10_00_00_00>(m33_m33_m23_m23, m33_m33_m23_m23);
    
    
            let a = _mm_mul_ps(m22_m22_m12_m12, m33_m33_m33_m23);
            let b = _mm_mul_ps(m32_m32_m32_m22, m23_m23_m13_m13);
            let fac0 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m21_m21_m11_m11, m33_m33_m33_m23);
            let b = _mm_mul_ps(m31_m31_m31_m21, m23_m23_m13_m13);
            let fac1 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m21_m21_m11_m11, m32_m32_m32_m22);
            let b = _mm_mul_ps(m31_m31_m31_m21, m22_m22_m12_m12);
            let fac2 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m20_m20_m10_m10, m33_m33_m33_m23);
            let b = _mm_mul_ps(m30_m30_m30_m20, m23_m23_m13_m13);
            let fac3 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m20_m20_m10_m10, m32_m32_m32_m22);
            let b = _mm_mul_ps(m30_m30_m30_m20, m22_m22_m12_m12);
            let fac4 = _mm_sub_ps(a, b);
    
            let a = _mm_mul_ps(m20_m20_m10_m10, m31_m31_m31_m21);
            let b = _mm_mul_ps(m30_m30_m30_m20, m21_m21_m11_m11);
            let fac5 = _mm_sub_ps(a, b);
    
    
            let m00_m01_m10_m11 = _mm_shuffle_ps::<0b_01_00_01_00>(self[0], self[1]);
            let m02_m03_m12_m13 = _mm_shuffle_ps::<0b_11_10_11_10>(self[0], self[1]);
            let vec0 = _mm_shuffle_ps::<0b_00_00_00_10>(m00_m01_m10_m11, m00_m01_m10_m11);
            let vec1 = _mm_shuffle_ps::<0b_01_01_01_11>(m00_m01_m10_m11, m00_m01_m10_m11);
            let vec2 = _mm_shuffle_ps::<0b_00_00_00_10>(m02_m03_m12_m13, m02_m03_m12_m13);
            let vec3 = _mm_shuffle_ps::<0b_01_01_01_11>(m02_m03_m12_m13, m02_m03_m12_m13);
    
    
            let inv0 = _mm_add_ps(_mm_sub_ps(_mm_mul_ps(vec1, fac0), _mm_mul_ps(vec2, fac1)), _mm_mul_ps(vec3, fac2));
            let inv1 = _mm_add_ps(_mm_sub_ps(_mm_mul_ps(vec0, fac0), _mm_mul_ps(vec2, fac3)), _mm_mul_ps(vec3, fac4));
            let inv2 = _mm_add_ps(_mm_sub_ps(_mm_mul_ps(vec0, fac1), _mm_mul_ps(vec1, fac3)), _mm_mul_ps(vec3, fac5));
            let inv3 = _mm_add_ps(_mm_sub_ps(_mm_mul_ps(vec0, fac2), _mm_mul_ps(vec1, fac4)), _mm_mul_ps(vec2, fac5));
    
    
            let one_neg_one_neg = _mm_load_ps(&ONE_NEG_ONE_NEG as *const f32);
            let neg_one_neg_one = _mm_load_ps(&NEG_ONE_NEG_ONE as *const f32);
            let inverse = [
                _mm_mul_ps(inv0, one_neg_one_neg), 
                _mm_mul_ps(inv1, neg_one_neg_one), 
                _mm_mul_ps(inv2, one_neg_one_neg), 
                _mm_mul_ps(inv3, neg_one_neg_one) 
            ];
    
            let m00_m00_m10_m10 = _mm_shuffle_ps::<0b_00_00_00_00>(inverse[0], inverse[1]);
            let m20_m20_m30_m30 = _mm_shuffle_ps::<0b_00_00_00_00>(inverse[2], inverse[3]);
            let row0 = _mm_shuffle_ps::<0b_10_00_10_00>(m00_m00_m10_m10, m20_m20_m30_m30);
            let det = _mm_mul_ps(self[0], row0);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(det, det);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(det, det);
            let a = _mm_add_ps(low, high);
            let b = _mm_shuffle_ps::<0b_00_01_00_01>(a, a);
            let det = _mm_add_ps(a, b);
            let val = _mm_cvtss_f32(det);
    
            if val.abs() <= f32::EPSILON {
                return None;
            }
    
            let one_one_one_one = _mm_loadu_ps(&ONE_ONE_ONE_ONE as *const f32);
            let recip_det = _mm_div_ps(one_one_one_one, det);
            Some(Matrix([
                _mm_mul_ps(inverse[0], recip_det), 
                _mm_mul_ps(inverse[1], recip_det), 
                _mm_mul_ps(inverse[2], recip_det), 
                _mm_mul_ps(inverse[3], recip_det) 
            ]))
        }
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
        unsafe { 
            Matrix([
                _mm_loadu_ps(&value[0] as *const _ as *const f32), 
                _mm_loadu_ps(&value[1] as *const _ as *const f32), 
                _mm_loadu_ps(&value[2] as *const _ as *const f32), 
                _mm_loadu_ps(&value[3] as *const _ as *const f32) 
            ])
        }
    }
}

impl Into<Float4x4> for Matrix {
    #[inline]
    fn into(self) -> Float4x4 {
        let mut value = Float4x4::default();
        unsafe {
            _mm_storeu_ps(&mut value[0] as *mut _ as *mut f32, self[0]);
            _mm_storeu_ps(&mut value[1] as *mut _ as *mut f32, self[1]);
            _mm_storeu_ps(&mut value[2] as *mut _ as *mut f32, self[2]);
            _mm_storeu_ps(&mut value[3] as *mut _ as *mut f32, self[3]);
        }
        return value;
    }
}

impl ops::Deref for Matrix {
    type Target = [__m128; 4];
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
        unsafe { 
            Matrix([
                _mm_add_ps(self[0], rhs[0]), 
                _mm_add_ps(self[1], rhs[1]), 
                _mm_add_ps(self[2], rhs[2]), 
                _mm_add_ps(self[3], rhs[3])
            ])
        }
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
        unsafe {
            Matrix([
                _mm_sub_ps(self[0], rhs[0]), 
                _mm_sub_ps(self[1], rhs[1]), 
                _mm_sub_ps(self[2], rhs[2]), 
                _mm_sub_ps(self[3], rhs[3]) 
            ])
        }
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
        unsafe {
            Matrix([
                _mm_sub_ps(_mm_setzero_ps(), self[0]), 
                _mm_sub_ps(_mm_setzero_ps(), self[1]), 
                _mm_sub_ps(_mm_setzero_ps(), self[2]), 
                _mm_sub_ps(_mm_setzero_ps(), self[3]) 
            ])
        }
    }
}

impl ops::Mul<Vector> for Matrix {
    type Output = Vector;
    /// Transformation of the vector.
    fn mul(self, rhs: Vector) -> Self::Output {
        unsafe {
            let rows = self.transpose();

            let e0 = _mm_mul_ps(rows[0], *rhs);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e0, e0);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e0, e0);
            let sum = _mm_add_ps(low, high);
            let mix = _mm_shuffle_ps::<0b_10_11_00_01>(sum, sum);
            let e0 = _mm_add_ps(sum, mix);

            let e1 = _mm_mul_ps(rows[1], *rhs);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e1, e1);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e1, e1);
            let sum = _mm_add_ps(low, high);
            let mix = _mm_shuffle_ps::<0b_10_11_00_01>(sum, sum);
            let e1: __m128 = _mm_add_ps(sum, mix);

            let e2 = _mm_mul_ps(rows[2], *rhs);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e2, e2);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e2, e2);
            let sum = _mm_add_ps(low, high);
            let mix = _mm_shuffle_ps::<0b_10_11_00_01>(sum, sum);
            let e2 = _mm_add_ps(sum, mix);

            let e3 = _mm_mul_ps(rows[3], *rhs);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e3, e3);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e3, e3);
            let sum = _mm_add_ps(low, high);
            let mix = _mm_shuffle_ps::<0b_10_11_00_01>(sum, sum);
            let e3 = _mm_add_ps(sum, mix);

            let tran0 = _mm_shuffle_ps::<0b_01_00_01_00>(e0, e1);
            let tran1 = _mm_shuffle_ps::<0b_01_00_01_00>(e2, e3);
            let col0 = _mm_shuffle_ps::<0b_01_00_01_00>(tran0, tran1);

            return Vector(col0);
        }
    }
}

impl ops::Mul<Self> for Matrix {
    type Output = Self;
    /// Multiplies two matrices.
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            let rows = self.transpose();

            let e0 = _mm_mul_ps(rows[0], rhs[0]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e0, e0);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e0, e0);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e0 = _mm_add_ps(low, high);

            let e1 = _mm_mul_ps(rows[1], rhs[0]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e1, e1);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e1, e1);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e1 = _mm_add_ps(low, high);

            let e2 = _mm_mul_ps(rows[2], rhs[0]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e2, e2);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e2, e2);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e2 = _mm_add_ps(low, high);

            let e3 = _mm_mul_ps(rows[3], rhs[0]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e3, e3);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e3, e3);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e3 = _mm_add_ps(low, high);

            let tran0 = _mm_shuffle_ps::<0b_00_00_00_00>(e0, e1);
            let tran1 = _mm_shuffle_ps::<0b_00_00_00_00>(e2, e3);
            let col0 = _mm_shuffle_ps::<0b_10_00_10_00>(tran0, tran1);


            let e0 = _mm_mul_ps(rows[0], rhs[1]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e0, e0);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e0, e0);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e0 = _mm_add_ps(low, high);

            let e1 = _mm_mul_ps(rows[1], rhs[1]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e1, e1);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e1, e1);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e1 = _mm_add_ps(low, high);

            let e2 = _mm_mul_ps(rows[2], rhs[1]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e2, e2);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e2, e2);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e2 = _mm_add_ps(low, high);

            let e3 = _mm_mul_ps(rows[3], rhs[1]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e3, e3);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e3, e3);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e3 = _mm_add_ps(low, high);

            let tran0 = _mm_shuffle_ps::<0b_00_00_00_00>(e0, e1);
            let tran1 = _mm_shuffle_ps::<0b_00_00_00_00>(e2, e3);
            let col1 = _mm_shuffle_ps::<0b_10_00_10_00>(tran0, tran1);


            let e0 = _mm_mul_ps(rows[0], rhs[2]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e0, e0);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e0, e0);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e0 = _mm_add_ps(low, high);

            let e1 = _mm_mul_ps(rows[1], rhs[2]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e1, e1);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e1, e1);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e1 = _mm_add_ps(low, high);

            let e2 = _mm_mul_ps(rows[2], rhs[2]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e2, e2);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e2, e2);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e2 = _mm_add_ps(low, high);

            let e3 = _mm_mul_ps(rows[3], rhs[2]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e3, e3);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e3, e3);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e3 = _mm_add_ps(low, high);

            let tran0 = _mm_shuffle_ps::<0b_00_00_00_00>(e0, e1);
            let tran1 = _mm_shuffle_ps::<0b_00_00_00_00>(e2, e3);
            let col2 = _mm_shuffle_ps::<0b_10_00_10_00>(tran0, tran1);


            let e0 = _mm_mul_ps(rows[0], rhs[3]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e0, e0);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e0, e0);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e0 = _mm_add_ps(low, high);

            let e1 = _mm_mul_ps(rows[1], rhs[3]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e1, e1);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e1, e1);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e1 = _mm_add_ps(low, high);

            let e2 = _mm_mul_ps(rows[2], rhs[3]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e2, e2);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e2, e2);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e2 = _mm_add_ps(low, high);

            let e3 = _mm_mul_ps(rows[3], rhs[3]);
            let low = _mm_shuffle_ps::<0b_01_00_01_00>(e3, e3);
            let high = _mm_shuffle_ps::<0b_11_10_11_10>(e3, e3);
            let sum = _mm_add_ps(low, high);
            let low = _mm_shuffle_ps::<0b_10_00_10_00>(sum, sum);
            let high = _mm_shuffle_ps::<0b_11_01_11_01>(sum, sum);
            let e3 = _mm_add_ps(low, high);

            let tran0 = _mm_shuffle_ps::<0b_00_00_00_00>(e0, e1);
            let tran1 = _mm_shuffle_ps::<0b_00_00_00_00>(e2, e3);
            let col3 = _mm_shuffle_ps::<0b_10_00_10_00>(tran0, tran1);
            
            return Matrix([col0, col1, col2, col3]);
        }
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
