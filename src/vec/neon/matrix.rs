use core::fmt;
use core::ops;
use core::arch::aarch64::*;
use crate::{ Vector, Float3x3, Float4x4 };



/// This is a matrix data type that uses the `SIMD` instruction.
/// 
/// Using the `scalar-math` feature disables the use of `SIMD` instructions.
/// 
/// It is recommended not to use this data types as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Matrix(pub(crate) [float32x4_t; 4]);

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
            let tran_r0r1 = vtrnq_f32(self[0], self[1]);
            let tran_r2r3 = vtrnq_f32(self[2], self[3]);
            let m00_m10_m02_m12 = tran_r0r1.0;
            let m01_m11_m03_m13 = tran_r0r1.1;
            let m20_m30_m22_m32 = tran_r2r3.0;
            let m21_m31_m23_m33 = tran_r2r3.1;

            let m00_m10 = vget_low_f32(m00_m10_m02_m12);
            let m20_m30 = vget_low_f32(m20_m30_m22_m32);
            let m00_m10_m20_m30 = vcombine_f32(m00_m10, m20_m30);

            let m01_m11 = vget_low_f32(m01_m11_m03_m13);
            let m21_m31 = vget_low_f32(m21_m31_m23_m33);
            let m01_m11_m21_m31 = vcombine_f32(m01_m11, m21_m31);

            let m02_m12 = vget_high_f32(m00_m10_m02_m12);
            let m22_m32 = vget_high_f32(m20_m30_m22_m32);
            let m02_m12_m22_m32 = vcombine_f32(m02_m12, m22_m32);
            
            let m03_m13 = vget_high_f32(m01_m11_m03_m13);
            let m23_m33 = vget_high_f32(m21_m31_m23_m33);
            let m03_m13_m23_m33 = vcombine_f32(m03_m13, m23_m33);

            Matrix([
                m00_m10_m20_m30, 
                m01_m11_m21_m31, 
                m02_m12_m22_m32, 
                m03_m13_m23_m33 
            ])
        }
    }

    /// Determinant of a matrix.
    pub fn determinant(self) -> Vector {
        // Reference: glm/detail/func_matrix.inl
        const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
        const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];
        unsafe {
            let m00_m01 = vget_low_f32(self[0]);
            let m02_m03 = vget_high_f32(self[0]);
            let m01_m00 = vext_f32::<0b01>(m00_m01, m00_m01);
            let m03_m02 = vext_f32::<0b01>(m02_m03, m02_m03);
            let m00_m00 = vext_f32::<0b01>(m01_m00, m00_m01);
            let m01_m01 = vext_f32::<0b01>(m00_m01, m01_m00);
            let m02_m02 = vext_f32::<0b01>(m03_m02, m02_m03);
            let m03_m03 = vext_f32::<0b01>(m02_m03, m03_m02);

            let m10_m11 = vget_low_f32(self[1]);
            let m12_m13 = vget_high_f32(self[1]);
            let m11_m10 = vext_f32::<0b01>(m10_m11, m10_m11);
            let m13_m12 = vext_f32::<0b01>(m12_m13, m12_m13);
            let m10_m10 = vext_f32::<0b01>(m11_m10, m10_m11);
            let m11_m11 = vext_f32::<0b01>(m10_m11, m11_m10);
            let m12_m12 = vext_f32::<0b01>(m13_m12, m12_m13);
            let m13_m13 = vext_f32::<0b01>(m12_m13, m13_m12);

            let m20_m21 = vget_low_f32(self[2]);
            let m22_m23 = vget_high_f32(self[2]);
            let m21_m20 = vext_f32::<0b01>(m20_m21, m20_m21);
            let m23_m22 = vext_f32::<0b01>(m22_m23, m22_m23);
            let m20_m20 = vext_f32::<0b01>(m21_m20, m20_m21);
            let m21_m21 = vext_f32::<0b01>(m20_m21, m21_m20);
            let m22_m22 = vext_f32::<0b01>(m23_m22, m22_m23);
            let m23_m23 = vext_f32::<0b01>(m22_m23, m23_m22);

            let m30_m31 = vget_low_f32(self[3]);
            let m32_m33 = vget_high_f32(self[3]);
            let m31_m30 = vext_f32::<0b01>(m30_m31, m30_m31);
            let m33_m32 = vext_f32::<0b01>(m32_m33, m32_m33);
            let m30_m30 = vext_f32::<0b01>(m31_m30, m30_m31);
            let m31_m31 = vext_f32::<0b01>(m30_m31, m31_m30);
            let m32_m32 = vext_f32::<0b01>(m33_m32, m32_m33);
            let m33_m33 = vext_f32::<0b01>(m32_m33, m33_m32);

            let m10_m00 = vext_f32::<0b01>(m11_m10, m00_m01);
            let m11_m01 = vext_f32::<0b01>(m10_m11, m01_m00);
            let m12_m02 = vext_f32::<0b01>(m13_m12, m02_m03);
            let m13_m03 = vext_f32::<0b01>(m12_m13, m03_m02);
            let m30_m20 = vext_f32::<0b01>(m31_m30, m20_m21);
            let m31_m21 = vext_f32::<0b01>(m30_m31, m21_m20);
            let m32_m22 = vext_f32::<0b01>(m33_m32, m22_m23);
            let m33_m23 = vext_f32::<0b01>(m32_m33, m23_m22);

            let m20_m20_m10_m10 = vcombine_f32(m20_m20, m10_m10);
            let m21_m21_m11_m11 = vcombine_f32(m21_m21, m11_m11);
            let m22_m22_m12_m12 = vcombine_f32(m22_m22, m12_m12);
            let m23_m23_m13_m13 = vcombine_f32(m23_m23, m13_m13);
            let m30_m30_m30_m20 = vcombine_f32(m30_m30, m30_m20);
            let m31_m31_m31_m21 = vcombine_f32(m31_m31, m31_m21);
            let m32_m32_m32_m22 = vcombine_f32(m32_m32, m32_m22);
            let m33_m33_m33_m23 = vcombine_f32(m33_m33, m33_m23);



            let a = vmulq_f32(m22_m22_m12_m12, m33_m33_m33_m23);
            let b = vmulq_f32(m32_m32_m32_m22, m23_m23_m13_m13);
            let fac0 = vsubq_f32(a, b);

            let a = vmulq_f32(m21_m21_m11_m11, m33_m33_m33_m23);
            let b = vmulq_f32(m31_m31_m31_m21, m23_m23_m13_m13);
            let fac1 = vsubq_f32(a, b);

            let a = vmulq_f32(m21_m21_m11_m11, m32_m32_m32_m22);
            let b = vmulq_f32(m31_m31_m31_m21, m22_m22_m12_m12);
            let fac2 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m33_m33_m33_m23);
            let b = vmulq_f32(m30_m30_m30_m20, m23_m23_m13_m13);
            let fac3 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m32_m32_m32_m22);
            let b = vmulq_f32(m30_m30_m30_m20, m22_m22_m12_m12);
            let fac4 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m31_m31_m31_m21);
            let b = vmulq_f32(m30_m30_m30_m20, m21_m21_m11_m11);
            let fac5 = vsubq_f32(a, b);

            let vec0 = vcombine_f32(m10_m00, m00_m00);
            let vec1 = vcombine_f32(m11_m01, m01_m01);
            let vec2 = vcombine_f32(m12_m02, m02_m02);
            let vec3 = vcombine_f32(m13_m03, m03_m03);

            let inv0 = vaddq_f32(vsubq_f32(vmulq_f32(vec1, fac0), vmulq_f32(vec2, fac1)), vmulq_f32(vec3, fac2));
            let inv1 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac0), vmulq_f32(vec2, fac3)), vmulq_f32(vec3, fac4));
            let inv2 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac1), vmulq_f32(vec1, fac3)), vmulq_f32(vec3, fac5));
            let inv3 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac2), vmulq_f32(vec1, fac4)), vmulq_f32(vec2, fac5));

            let one_neg_one_neg = vld1q_f32(&ONE_NEG_ONE_NEG as *const f32);
            let neg_one_neg_one = vld1q_f32(&NEG_ONE_NEG_ONE as *const f32);
            let inverse = [
                vmulq_f32(inv0, one_neg_one_neg), 
                vmulq_f32(inv1, neg_one_neg_one), 
                vmulq_f32(inv2, one_neg_one_neg), 
                vmulq_f32(inv3, neg_one_neg_one), 
            ];

            let inv00_inv10 = vget_low_f32(vtrnq_f32(inverse[0], inverse[1]).0); 
            let inv20_inv30 = vget_low_f32(vtrnq_f32(inverse[2], inverse[3]).0);
            let row0 = vcombine_f32(inv00_inv10, inv20_inv30);
            let det = vmulq_f32(self[0], row0);
            let det = vpaddq_f32(det, det);
            let det = vpaddq_f32(det, det);
            
            return Vector(det);
        }
    }

    /// Inverse of a matrix.
    /// If the inverse of matrix cannot be calculated, returns `None`.
    pub fn inverse(self) -> Option<Self> {
        // Reference: glm/detail/func_matrix.inl
        const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
        const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];
        unsafe {
            let m00_m01 = vget_low_f32(self[0]);
            let m02_m03 = vget_high_f32(self[0]);
            let m01_m00 = vext_f32::<0b01>(m00_m01, m00_m01);
            let m03_m02 = vext_f32::<0b01>(m02_m03, m02_m03);
            let m00_m00 = vext_f32::<0b01>(m01_m00, m00_m01);
            let m01_m01 = vext_f32::<0b01>(m00_m01, m01_m00);
            let m02_m02 = vext_f32::<0b01>(m03_m02, m02_m03);
            let m03_m03 = vext_f32::<0b01>(m02_m03, m03_m02);

            let m10_m11 = vget_low_f32(self[1]);
            let m12_m13 = vget_high_f32(self[1]);
            let m11_m10 = vext_f32::<0b01>(m10_m11, m10_m11);
            let m13_m12 = vext_f32::<0b01>(m12_m13, m12_m13);
            let m10_m10 = vext_f32::<0b01>(m11_m10, m10_m11);
            let m11_m11 = vext_f32::<0b01>(m10_m11, m11_m10);
            let m12_m12 = vext_f32::<0b01>(m13_m12, m12_m13);
            let m13_m13 = vext_f32::<0b01>(m12_m13, m13_m12);

            let m20_m21 = vget_low_f32(self[2]);
            let m22_m23 = vget_high_f32(self[2]);
            let m21_m20 = vext_f32::<0b01>(m20_m21, m20_m21);
            let m23_m22 = vext_f32::<0b01>(m22_m23, m22_m23);
            let m20_m20 = vext_f32::<0b01>(m21_m20, m20_m21);
            let m21_m21 = vext_f32::<0b01>(m20_m21, m21_m20);
            let m22_m22 = vext_f32::<0b01>(m23_m22, m22_m23);
            let m23_m23 = vext_f32::<0b01>(m22_m23, m23_m22);

            let m30_m31 = vget_low_f32(self[3]);
            let m32_m33 = vget_high_f32(self[3]);
            let m31_m30 = vext_f32::<0b01>(m30_m31, m30_m31);
            let m33_m32 = vext_f32::<0b01>(m32_m33, m32_m33);
            let m30_m30 = vext_f32::<0b01>(m31_m30, m30_m31);
            let m31_m31 = vext_f32::<0b01>(m30_m31, m31_m30);
            let m32_m32 = vext_f32::<0b01>(m33_m32, m32_m33);
            let m33_m33 = vext_f32::<0b01>(m32_m33, m33_m32);

            let m10_m00 = vext_f32::<0b01>(m11_m10, m00_m01);
            let m11_m01 = vext_f32::<0b01>(m10_m11, m01_m00);
            let m12_m02 = vext_f32::<0b01>(m13_m12, m02_m03);
            let m13_m03 = vext_f32::<0b01>(m12_m13, m03_m02);
            let m30_m20 = vext_f32::<0b01>(m31_m30, m20_m21);
            let m31_m21 = vext_f32::<0b01>(m30_m31, m21_m20);
            let m32_m22 = vext_f32::<0b01>(m33_m32, m22_m23);
            let m33_m23 = vext_f32::<0b01>(m32_m33, m23_m22);

            let m20_m20_m10_m10 = vcombine_f32(m20_m20, m10_m10);
            let m21_m21_m11_m11 = vcombine_f32(m21_m21, m11_m11);
            let m22_m22_m12_m12 = vcombine_f32(m22_m22, m12_m12);
            let m23_m23_m13_m13 = vcombine_f32(m23_m23, m13_m13);
            let m30_m30_m30_m20 = vcombine_f32(m30_m30, m30_m20);
            let m31_m31_m31_m21 = vcombine_f32(m31_m31, m31_m21);
            let m32_m32_m32_m22 = vcombine_f32(m32_m32, m32_m22);
            let m33_m33_m33_m23 = vcombine_f32(m33_m33, m33_m23);



            let a = vmulq_f32(m22_m22_m12_m12, m33_m33_m33_m23);
            let b = vmulq_f32(m32_m32_m32_m22, m23_m23_m13_m13);
            let fac0 = vsubq_f32(a, b);

            let a = vmulq_f32(m21_m21_m11_m11, m33_m33_m33_m23);
            let b = vmulq_f32(m31_m31_m31_m21, m23_m23_m13_m13);
            let fac1 = vsubq_f32(a, b);

            let a = vmulq_f32(m21_m21_m11_m11, m32_m32_m32_m22);
            let b = vmulq_f32(m31_m31_m31_m21, m22_m22_m12_m12);
            let fac2 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m33_m33_m33_m23);
            let b = vmulq_f32(m30_m30_m30_m20, m23_m23_m13_m13);
            let fac3 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m32_m32_m32_m22);
            let b = vmulq_f32(m30_m30_m30_m20, m22_m22_m12_m12);
            let fac4 = vsubq_f32(a, b);

            let a = vmulq_f32(m20_m20_m10_m10, m31_m31_m31_m21);
            let b = vmulq_f32(m30_m30_m30_m20, m21_m21_m11_m11);
            let fac5 = vsubq_f32(a, b);

            let vec0 = vcombine_f32(m10_m00, m00_m00);
            let vec1 = vcombine_f32(m11_m01, m01_m01);
            let vec2 = vcombine_f32(m12_m02, m02_m02);
            let vec3 = vcombine_f32(m13_m03, m03_m03);

            let inv0 = vaddq_f32(vsubq_f32(vmulq_f32(vec1, fac0), vmulq_f32(vec2, fac1)), vmulq_f32(vec3, fac2));
            let inv1 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac0), vmulq_f32(vec2, fac3)), vmulq_f32(vec3, fac4));
            let inv2 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac1), vmulq_f32(vec1, fac3)), vmulq_f32(vec3, fac5));
            let inv3 = vaddq_f32(vsubq_f32(vmulq_f32(vec0, fac2), vmulq_f32(vec1, fac4)), vmulq_f32(vec2, fac5));

            let one_neg_one_neg = vld1q_f32(&ONE_NEG_ONE_NEG as *const f32);
            let neg_one_neg_one = vld1q_f32(&NEG_ONE_NEG_ONE as *const f32);
            let inverse = [
                vmulq_f32(inv0, one_neg_one_neg), 
                vmulq_f32(inv1, neg_one_neg_one), 
                vmulq_f32(inv2, one_neg_one_neg), 
                vmulq_f32(inv3, neg_one_neg_one), 
            ];

            let inv00_inv10 = vget_low_f32(vtrnq_f32(inverse[0], inverse[1]).0); 
            let inv20_inv30 = vget_low_f32(vtrnq_f32(inverse[2], inverse[3]).0);
            let row0 = vcombine_f32(inv00_inv10, inv20_inv30);
            let det = vmulq_f32(self[0], row0);
            let det = vpaddq_f32(det, det);
            let det = vpaddq_f32(det, det);
            let det = vgetq_lane_f32::<0b00>(det);
            if det.abs() <= f32::EPSILON {
                return None;
            }

            let recip_det = det.recip();
            Some(Matrix([
                vmulq_n_f32(inverse[0], recip_det), 
                vmulq_n_f32(inverse[1], recip_det), 
                vmulq_n_f32(inverse[2], recip_det), 
                vmulq_n_f32(inverse[3], recip_det)
            ]))
        }
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
        unsafe { 
            Matrix([
                vld1q_f32(&value[0] as *const _ as *const f32), 
                vld1q_f32(&value[1] as *const _ as *const f32), 
                vld1q_f32(&value[2] as *const _ as *const f32), 
                vld1q_f32(&value[3] as *const _ as *const f32) 
            ])
        }
    }
}

impl Into<Float4x4> for Matrix {
    #[inline]
    fn into(self) -> Float4x4 {
        let mut value = Float4x4::default();
        unsafe {
            vst1q_f32(&mut value[0] as *mut _ as *mut f32, self[0]);
            vst1q_f32(&mut value[1] as *mut _ as *mut f32, self[1]);
            vst1q_f32(&mut value[2] as *mut _ as *mut f32, self[2]);
            vst1q_f32(&mut value[3] as *mut _ as *mut f32, self[3]);
        }
        return value;
    }
}

impl ops::Deref for Matrix {
    type Target = [float32x4_t; 4];
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
                vaddq_f32(self[0], rhs[0]), 
                vaddq_f32(self[1], rhs[1]), 
                vaddq_f32(self[2], rhs[2]), 
                vaddq_f32(self[3], rhs[3])
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
                vsubq_f32(self[0], rhs[0]), 
                vsubq_f32(self[1], rhs[1]), 
                vsubq_f32(self[2], rhs[2]), 
                vsubq_f32(self[3], rhs[3]) 
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
                vnegq_f32(self[0]), 
                vnegq_f32(self[1]), 
                vnegq_f32(self[2]), 
                vnegq_f32(self[3])
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

            let e0 = vmulq_f32(rows[0], *rhs);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows[1], *rhs);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows[2], *rhs);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows[3], *rhs);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col0 = vextq_f32::<0b10>(tran0, tran1);

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

            let e0 = vmulq_f32(rows[0], rhs[0]);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows[1], rhs[0]);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows[2], rhs[0]);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows[3], rhs[0]);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col0 = vextq_f32::<0b10>(tran0, tran1);


            let e0 = vmulq_f32(rows[0], rhs[1]);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows[1], rhs[1]);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows[2], rhs[1]);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows[3], rhs[1]);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col1 = vextq_f32::<0b10>(tran0, tran1);


            let e0 = vmulq_f32(rows[0], rhs[2]);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows[1], rhs[2]);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows[2], rhs[2]);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows[3], rhs[2]);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col2 = vextq_f32::<0b10>(tran0, tran1);


            let e0 = vmulq_f32(rows[0], rhs[3]);
            let e0 = vpaddq_f32(e0, e0);
            let e0 = vpaddq_f32(e0, e0);

            let e1 = vmulq_f32(rows[1], rhs[3]);
            let e1 = vpaddq_f32(e1, e1);
            let e1 = vpaddq_f32(e1, e1);

            let e2 = vmulq_f32(rows[2], rhs[3]);
            let e2 = vpaddq_f32(e2, e2);
            let e2 = vpaddq_f32(e2, e2);

            let e3 = vmulq_f32(rows[3], rhs[3]);
            let e3 = vpaddq_f32(e3, e3);
            let e3 = vpaddq_f32(e3, e3);

            let tran0 = vtrn1q_f32(e0, e1);
            let tran1 = vtrn1q_f32(e2, e3);
            let col3 = vextq_f32::<0b10>(tran0, tran1);
            
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
