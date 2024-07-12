use crate::{
    Boolean2, Boolean3, Boolean4, 
    UInteger2, UInteger3, UInteger4, 
    Float2, Float3, Float3x3, Float4, Float4x4, 
};
use core::ops::*;



/// This is the data type used in vector operations.
pub type Vector = Float4;

/// This is the data type used in vector operations.
pub type VectorU32 = UInteger4;

/// This is the data type used in matrix operations.
pub type Matrix = Float4x4;


/// Load vector data from [`Boolean2`].
#[must_use]
#[inline(always)]
pub fn load_boolean2(src: Boolean2) -> VectorU32 {
    load_boolean4(Boolean4::from(src))
}

/// Load vector data from [`Boolean3`].
#[must_use]
#[inline(always)]
pub fn load_boolean3(src: Boolean3) -> VectorU32 {
    load_boolean4(Boolean4::from(src))
}

/// Load vector data from [`Boolean4`].
#[must_use]
#[inline(always)]
pub fn load_boolean4(src: Boolean4) -> VectorU32 {
    UInteger4::from(src)
}

/// Load vector data from [`Float2`].
#[must_use]
#[inline(always)]
pub fn load_float2(src: Float2 )-> Vector {
    Float4::from(src)
}

/// Load vector data from [`Float3`].
#[must_use]
#[inline(always)]
pub fn load_float3(src: Float3) -> Vector {
    Float4::from(src)
}

/// Load vector data from [`Float4`].
#[must_use]
#[inline(always)]
pub fn load_float4(src: Float4) -> Vector {
    Float4::from(src)
}

/// Load matrix data from [`Float3x3`].
#[must_use]
#[inline(always)]
pub fn load_float3x3(src: Float3x3) -> Matrix {
    Float4x4::from(src)
}

/// Load matrix data from [`Float4x4`].
#[must_use]
#[inline(always)]
pub fn load_float4x4(src: Float4x4) -> Matrix {
    Float4x4::from(src)
}

/// Load vector data from [`UInteger2`].
#[must_use]
#[inline(always)]
pub fn load_uinteger2(src: UInteger2) -> VectorU32 {
    UInteger4::from(src)
}

/// Load vector data from [`UInteger3`].
#[must_use]
#[inline(always)]
pub fn load_uinteger3(src: UInteger3) -> VectorU32 {
    UInteger4::from(src)
}

/// Load vector data from [`UInteger4`].
#[must_use]
#[inline(always)]
pub fn load_uinteger4(src: UInteger4) -> VectorU32 {
    UInteger4::from(src)
}

/// Store vector data in [`Boolean2`].
#[must_use]
#[inline(always)]
pub fn store_boolean2(src: VectorU32) -> Boolean2 {
    store_uinteger2(src).into()
}

/// Store vector data in [`Boolean3`].
#[must_use]
#[inline(always)]
pub fn store_boolean3(src: VectorU32) -> Boolean3 {
    store_uinteger3(src).into()
}

/// Store vector data in [`Boolean4`].
#[must_use]
#[inline(always)]
pub fn store_boolean4(src: VectorU32) -> Boolean4 {
    store_uinteger4(src).into()
}

/// Store vector data in [`Float2`].
#[must_use]
#[inline(always)]
pub fn store_float2(src: Vector) -> Float2 {
    Float2::from(store_float4(src))
}

/// Store vector data in [`Float3`].
#[must_use]
#[inline(always)]
pub fn store_float3(src: Vector) -> Float3 {
    Float3::from(store_float4(src))
}

/// Store vector data in [`Float4`].
#[must_use]
#[inline(always)]
pub fn store_float4(src: Vector) -> Float4 {
    Float4::from(src)
}

/// Store matrix data in [`Float3x3`].
#[must_use]
#[inline(always)]
pub fn store_float3x3(src: Matrix) -> Float3x3 {
    Float3x3::from(store_float4x4(src))
}

/// Store matrix data in [`Float4x4`].
#[must_use]
#[inline(always)]
pub fn store_float4x4(src: Matrix) -> Float4x4 {
    Float4x4::from(src)
}

/// Store vector data in [`UInteger2`].
#[must_use]
#[inline(always)]
pub fn store_uinteger2(src: VectorU32) -> UInteger2 {
    UInteger2::from(store_uinteger4(src))
}

/// Store vector data in [`UInteger3`].
#[must_use]
#[inline(always)]
pub fn store_uinteger3(src: VectorU32) -> UInteger3 {
    UInteger3::from(store_uinteger4(src))
}

/// Store vector data in [`UInteger4`].
#[must_use]
#[inline(always)]
pub fn store_uinteger4(src: VectorU32) -> UInteger4 {
    UInteger4::from(src)
}



/// Adds two vectors.
#[must_use]
#[inline(always)]
pub fn vector_add(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.x.add(b.x), 
        a.y.add(b.y), 
        a.z.add(b.z), 
        a.w.add(b.w)
    )
}

/// Subtracts two vectors.
#[must_use]
#[inline(always)]
pub fn vector_sub(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.x.sub(b.x), 
        a.y.sub(b.y), 
        a.z.sub(b.z), 
        a.w.sub(b.w)
    )
}

/// Multiplies two vectors.
#[must_use]
#[inline(always)]
pub fn vector_mul(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.x.mul(b.x), 
        a.y.mul(b.y), 
        a.z.mul(b.z), 
        a.w.mul(b.w)
    )
}

/// Divides two vectors.
#[must_use]
#[inline(always)]
pub fn vector_div(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.x.div(b.x), 
        a.y.div(b.y), 
        a.z.div(b.z), 
        a.w.div(b.w)
    )
}

/// Absolute value on vector elements.
#[must_use]
#[inline(always)]
pub fn vector_abs(v: Vector) -> Vector {
    Vector::new(
        v.x.abs(), 
        v.y.abs(), 
        v.z.abs(), 
        v.w.abs()
    )
}

/// Negativizes the given vector.
#[must_use]
#[inline(always)]
pub fn vector_neg(v: Vector) -> Vector {
    Vector::new(
        v.x.neg(), 
        v.y.neg(), 
        v.z.neg(), 
        v.w.neg()
    )
}

/// Takes the smaller of the elements of the two vectors.
#[must_use]
#[inline(always)]
pub fn vector_min(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.x.min(b.x), 
        a.y.min(b.y), 
        a.z.min(b.z), 
        a.w.min(b.w)
    )
}

/// Takes the larger of the elements of the two vectors.
#[must_use]
#[inline(always)]
pub fn vector_max(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.x.max(b.x), 
        a.y.max(b.y), 
        a.z.max(b.z), 
        a.w.max(b.w)
    )
}

/// Checks if the elements of two given vectors are less.
#[must_use]
#[inline(always)]
pub fn vector_lt(a: Vector, b: Vector) -> Boolean4 {
    Boolean4::new(
        a.x.lt(&b.x), 
        a.y.lt(&b.y), 
        a.z.lt(&b.z), 
        a.w.lt(&b.w)
    )
}

/// Checks if the elemets of two given vectors are less than or equal.
#[must_use]
#[inline(always)]
pub fn vector_le(a: Vector, b: Vector) -> Boolean4 {
    Boolean4::new(
        a.x.le(&b.x), 
        a.y.le(&b.y), 
        a.z.le(&b.z), 
        b.w.le(&b.w)
    )
}

/// Checks if the elements of two given vectors are greater.
#[must_use]
#[inline(always)]
pub fn vector_gt(a: Vector, b: Vector) -> Boolean4 {
    Boolean4::new(
        a.x.gt(&b.x), 
        a.y.gt(&b.y), 
        a.z.gt(&b.z), 
        a.w.gt(&b.w)
    )
}

/// Checks if the elements of two given vectors are greater than or equal.
#[must_use]
#[inline(always)]
pub fn vector_ge(a: Vector, b: Vector) -> Boolean4 {
    Boolean4::new(
        a.x.ge(&b.x), 
        a.y.ge(&b.y), 
        a.z.ge(&b.z), 
        a.w.ge(&b.w)
    )
}

/// Checks if the elements of two given vectors are equal.
/// This function does not use [`f32::EPSILON`].
/// 
#[must_use]
#[inline(always)]
pub fn vector_eq(a: Vector, b: Vector) -> Boolean4 {
    Boolean4::new(
        a.x.eq(&b.x), 
        a.y.eq(&b.y), 
        a.z.eq(&b.z), 
        a.w.eq(&b.w)
    )
}

/// Checks if the elements of two given vectors are not equal.
/// This function does not use [`f32::EPSILON`].
/// 
#[must_use]
#[inline(always)]
pub fn vector_ne(a: Vector, b: Vector) -> Boolean4 {
    Boolean4::new(
        a.x.ne(&b.x), 
        a.y.ne(&b.y), 
        a.z.ne(&b.z), 
        a.w.ne(&b.w)
    )
}

/// Returns a vector filled by adding all the elements of the vector.
#[must_use]
#[inline(always)]
pub fn vector_sum(v: Vector) -> Vector {
    Vector::new(
        v.x + v.y + v.z + v.w, 
        v.x + v.y + v.z + v.w, 
        v.x + v.y + v.z + v.w, 
        v.x + v.y + v.z + v.w
    )
}

/// Transformation of the given vector.
#[inline]
#[must_use]
pub fn vector_transform(m: Matrix, v: Vector) -> Vector {
    let tm = matrix_transpose(m);
    let e0 = vector_mul(tm.x_axis, v);
    let e0 = vector_sum(e0).x;

    let e1 = vector_mul(tm.y_axis, v);
    let e1 = vector_sum(e1).x;

    let e2 = vector_mul(tm.z_axis, v);
    let e2 = vector_sum(e2).x;

    let e3 = vector_mul(tm.w_axis, v);
    let e3 = vector_sum(e3).x;

    Float4::new(e0, e1, e2, e3)
}


/// Length sqared of a two-element vector.
#[must_use]
#[inline(always)]
pub fn vector2_length_sq(v: Vector) -> f32 {
    vector2_dot(v, v)
}

/// Length sqared of a three-element vector.
#[must_use]
#[inline(always)]
pub fn vector3_length_sq(v: Vector) -> f32 {
    vector3_dot(v, v)
}

/// Length sqared of a four-element vector.
#[must_use]
#[inline(always)]
pub fn vector4_length_sq(v: Vector) -> f32 {
    vector4_dot(v, v)
}

/// Length of a two-element vector.
#[must_use]
#[inline(always)]
pub fn vector2_length(v: Vector) -> f32 {
    vector2_length_sq(v).sqrt()
}

/// Length of a three-element vector.
#[must_use]
#[inline(always)]
pub fn vector3_length(v: Vector) -> f32 {
    vector3_length_sq(v).sqrt()
}

/// Length of a four-element vector.
#[must_use]
#[inline(always)]
pub fn vector4_length(v: Vector) -> f32 {
    vector4_length_sq(v).sqrt()
}

/// Normalizes a given two-element vector.
/// If normalization fails, `None` is returned.
#[inline]
#[must_use]
pub fn vector2_normalize(v: Vector) -> Option<Vector> {
    let len = vector2_length(v);
    if len > 0.0 {
        let len = load_float4(Float4::fill(len));
        Some(vector_div(v, len))
    } else {
        None
    }
}

/// Normalizes a given three-element vector.
/// If normalization fails, `None` is returned.
#[inline]
#[must_use]
pub fn vector3_normalize(v: Vector) -> Option<Vector> {
    let len = vector3_length(v);
    if len > 0.0 {
        let len = load_float4(Float4::fill(len));
        Some(vector_div(v, len))
    } else {
        None
    }
}

/// Normalizes a given four-element vector.
/// If normalization fails, `None` is returned.
#[inline]
#[must_use]
pub fn vector4_normalize(v: Vector) -> Option<Vector> {
    let len = vector4_length(v);
    if len > 0.0 {
        let len = load_float4(Float4::fill(len));
        Some(vector_div(v, len))
    } else {
        None
    }
}

/// Dot product of a two-element vector
#[must_use]
#[inline(always)]
pub fn vector2_dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y
}

/// Dot product of a three-element vector
#[must_use]
#[inline(always)]
pub fn vector3_dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// Dot product of a four-element vector
#[must_use]
#[inline(always)]
pub fn vector4_dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

/// Cross product of a three-element vector
#[must_use]
#[inline(always)]
pub fn vector3_cross(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.y * b.z - a.z * b.y, 
        a.z * b.x - a.x * b.z, 
        a.x * b.y - a.y * b.x, 
        1.0
    )
}

/// Checks if two elements of two given vectors are equal.
/// This function compares using [`f32::EPSILON`].
#[inline]
#[must_use]
pub fn vector2_eq(a: Vector, b: Vector) -> bool {
    let diff = vector_sub(a, b);
    let len = vector2_length_sq(diff);
    return len <= f32::EPSILON;
}

/// Checks if three elements of two given vectors are equal.
/// This function compares using [`f32::EPSILON`].
#[inline]
#[must_use]
pub fn vector3_eq(a: Vector, b: Vector) -> bool {
    let diff = vector_sub(a, b);
    let len = vector3_length_sq(diff);
    return len <= f32::EPSILON;
}

/// Checks if four elements of two given vectors are equal.
/// This function compares using [`f32::EPSILON`].
#[inline]
#[must_use]
pub fn vector4_eq(a: Vector, b: Vector) -> bool {
    let diff = vector_sub(a, b);
    let len = vector4_length_sq(diff);
    return len <= f32::EPSILON;
}



/// Dot product of a two quaternions.
#[must_use]
#[inline(always)]
pub fn quaternion_dot(a: Vector, b: Vector) -> f32 {
    vector4_dot(a, b)
}

/// Multiplies two quaternions.
#[must_use]
#[inline(always)]
pub fn quaternion_mul(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y, 
        a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x, 
        a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w, 
        a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z
    )
}

/// Length sqared of a quaternion.
#[must_use]
#[inline(always)]
pub fn quaternion_length_sq(q: Vector) -> f32 {
    vector4_length_sq(q)
}

/// Length of a quaternion.
#[must_use]
#[inline(always)]
pub fn quaternion_length(q: Vector) -> f32 {
    vector4_length(q)
}

/// Normalizes a given quaternion. 
/// If normalization fails, `None` is returned.
#[must_use]
#[inline(always)]
pub fn quaternion_normalize(q: Vector) -> Option<Vector> {
    vector4_normalize(q)
}

/// Returns the conjugate of the quaternion.
#[inline(always)]
pub fn quaternion_conjugate(q: Vector) -> Vector {
    Vector::new(
        q.x.neg(), 
        q.y.neg(), 
        q.z.neg(), 
        q.w
    )
}

/// Returns the inverse of the quaternion. 
/// If normalization fails, `None` is returned.
#[inline]
#[must_use]
pub fn quaternion_inverse(q: Vector) -> Option<Vector> {
    quaternion_normalize(q)
        .map(|q| quaternion_conjugate(q))
}



/// Adds two matrices.
#[must_use]
#[inline(always)]
pub fn matrix_add(a: Matrix, b: Matrix) -> Matrix {
    Float4x4::from_columns(
        vector_add(a.x_axis, b.x_axis), 
        vector_add(a.y_axis, b.y_axis), 
        vector_add(a.z_axis, b.z_axis), 
        vector_add(a.w_axis, b.w_axis)
    )
}

/// Subtracts two matrices.
#[must_use]
#[inline(always)]
pub fn matrix_sub(a: Matrix, b: Matrix) -> Matrix {
    Float4x4::from_columns(
        vector_sub(a.x_axis, b.x_axis), 
        vector_sub(a.y_axis, b.y_axis), 
        vector_sub(a.z_axis, b.z_axis), 
        vector_sub(a.w_axis, b.w_axis)
    )
}

/// Negativizes the given matrix.
#[must_use]
#[inline(always)]
pub fn matrix_neg(m: Matrix) -> Matrix {
    Float4x4::from_columns(
        vector_neg(m.x_axis), 
        vector_neg(m.y_axis), 
        vector_neg(m.z_axis), 
        vector_neg(m.w_axis)
    )
}

/// Multiplies two matrices.
#[must_use]
pub fn matrix_mul(a: Matrix, b: Matrix) -> Matrix {
    let ta = matrix_transpose(a);
    
    let m00 = vector_mul(ta.x_axis, b.x_axis);
    let m00 = vector_sum(m00).x;
    let m01 = vector_mul(ta.y_axis, b.x_axis);
    let m01 = vector_sum(m01).x;
    let m02 = vector_mul(ta.z_axis, b.x_axis);
    let m02 = vector_sum(m02).x;
    let m03 = vector_mul(ta.w_axis, b.x_axis);
    let m03 = vector_sum(m03).x;
    let v0 = Float4::new(m00, m01, m02, m03);

    let m10 = vector_mul(ta.x_axis, b.y_axis);
    let m10 = vector_sum(m10).x;
    let m11 = vector_mul(ta.y_axis, b.y_axis);
    let m11 = vector_sum(m11).x;
    let m12 = vector_mul(ta.z_axis, b.y_axis);
    let m12 = vector_sum(m12).x;
    let m13 = vector_mul(ta.w_axis, b.y_axis);
    let m13 = vector_sum(m13).x;
    let v1 = Float4::new(m10, m11, m12, m13);

    let m20 = vector_mul(ta.x_axis, b.z_axis);
    let m20 = vector_sum(m20).x;
    let m21 = vector_mul(ta.y_axis, b.z_axis);
    let m21 = vector_sum(m21).x;
    let m22 = vector_mul(ta.z_axis, b.z_axis);
    let m22 = vector_sum(m22).x;
    let m23 = vector_mul(ta.w_axis, b.z_axis);
    let m23 = vector_sum(m23).x;
    let v2 = Float4::new(m20, m21, m22, m23);

    let m30 = vector_mul(ta.x_axis, b.w_axis);
    let m30 = vector_sum(m30).x;
    let m31 = vector_mul(ta.y_axis, b.w_axis);
    let m31 = vector_sum(m31).x;
    let m32 = vector_mul(ta.z_axis, b.w_axis);
    let m32 = vector_sum(m32).x;
    let m33 = vector_mul(ta.w_axis, b.w_axis);
    let m33 = vector_sum(m33).x;
    let v3 = Float4::new(m30, m31, m32, m33);

    Float4x4::from_columns(v0, v1, v2, v3)
}

/// Transpose of a matrix.
#[inline]
#[must_use]
pub fn matrix_transpose(m: Matrix) -> Matrix {
    Float4x4::from_columns(
        Float4::new(m.x_axis.x, m.y_axis.x, m.z_axis.x, m.w_axis.x), 
        Float4::new(m.x_axis.y, m.y_axis.y, m.z_axis.y, m.w_axis.y), 
        Float4::new(m.x_axis.z, m.y_axis.z, m.z_axis.z, m.w_axis.z), 
        Float4::new(m.x_axis.w, m.y_axis.w, m.z_axis.w, m.w_axis.w)
    )
}

/// determinant of a matrix.
#[must_use]
pub fn matrix_determinant(m: Matrix) -> f32 {
    // det = m00 * (m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m13*m22*m31 - m12*m21*m33 - m11*m23*m32)
    //      - m10 * (m01*m22*m33 + m02*m23*m31 + m03*m21*m32 - m03*m22*m31 - m02*m21*m33 - m01*m23*m32)
    //      + m20 * (m01*m12*m33 + m02*m13*m31 + m03*m11*m32 - m03*m12*m31 - m02*m11*m33 - m01*m13*m32)
    //      - m30 * (m01*m12*m23 + m02*m13*m21 + m03*m11*m22 - m03*m12*m21 - m02*m11*m23 - m01*m13*m22)
    //
    
    let e0 = m.y_axis.y * m.z_axis.z * m.w_axis.w;
    let e1 = m.y_axis.z * m.z_axis.w * m.w_axis.y;
    let e2 = m.y_axis.w * m.z_axis.y * m.w_axis.z;
    let e3 = m.y_axis.w * m.z_axis.z * m.w_axis.y;
    let e4 = m.y_axis.z * m.z_axis.y * m.w_axis.w;
    let e5 = m.y_axis.y * m.z_axis.w * m.w_axis.z;
    let v0 = m.x_axis.x  * (e0 + e1 + e2 - e3 - e4 - e5);

    let e0 = m.x_axis.y * m.z_axis.z * m.w_axis.w;
    let e1 = m.x_axis.z * m.z_axis.w * m.w_axis.y;
    let e2 = m.x_axis.w * m.z_axis.y * m.w_axis.z;
    let e3 = m.x_axis.w * m.z_axis.z * m.w_axis.y;
    let e4 = m.x_axis.z * m.z_axis.y * m.w_axis.w;
    let e5 = m.x_axis.y * m.z_axis.w * m.w_axis.z;
    let v1 = m.y_axis.x * (e0 + e1 + e2 - e3 - e4 - e5);

    let e0 = m.x_axis.y * m.y_axis.z * m.w_axis.w;
    let e1 = m.x_axis.z * m.y_axis.w * m.w_axis.y;
    let e2 = m.x_axis.w * m.y_axis.y * m.w_axis.z;
    let e3 = m.x_axis.w * m.y_axis.z * m.w_axis.y;
    let e4 = m.x_axis.z * m.y_axis.y * m.w_axis.w;
    let e5 = m.x_axis.y * m.y_axis.w * m.w_axis.z;
    let v2 = m.z_axis.x * (e0 + e1 + e2 - e3 - e4 - e5);

    let e0 = m.x_axis.y * m.y_axis.z * m.z_axis.w;
    let e1 = m.x_axis.z * m.y_axis.w * m.z_axis.y;
    let e2 = m.x_axis.w * m.y_axis.y * m.z_axis.z;
    let e3 = m.x_axis.w * m.y_axis.z * m.z_axis.y;
    let e4 = m.x_axis.z * m.y_axis.y * m.z_axis.w;
    let e5 = m.x_axis.y * m.y_axis.w * m.z_axis.z;
    let v3 = m.w_axis.x * (e0 + e1 + e2 - e3 - e4 - e5);

    v0 - v1 + v2 - v3
}

/// inverse of a matrix.
/// 
/// If the inverse of a matrix cannot be calculated, returns `None`.
/// 
#[must_use]
pub fn matrix_inverse(m: Matrix) -> Option<Matrix> {
    // Reference: glm/detail/func_matrix.inl
    //
    let coef00 = m[2][2] * m[3][3] - m[3][2] * m[2][3];
    let coef02 = m[1][2] * m[3][3] - m[3][2] * m[1][3];
    let coef03 = m[1][2] * m[2][3] - m[2][2] * m[1][3];

    let coef04 = m[2][1] * m[3][3] - m[3][1] * m[2][3];
    let coef06 = m[1][1] * m[3][3] - m[3][1] * m[1][3];
    let coef07 = m[1][1] * m[2][3] - m[2][1] * m[1][3];

    let coef08 = m[2][1] * m[3][2] - m[3][1] * m[2][2];
    let coef10 = m[1][1] * m[3][2] - m[3][1] * m[1][2];
    let coef11 = m[1][1] * m[2][2] - m[2][1] * m[1][2];

    let coef12 = m[2][0] * m[3][3] - m[3][0] * m[2][3];
    let coef14 = m[1][0] * m[3][3] - m[3][0] * m[1][3];
    let coef15 = m[1][0] * m[2][3] - m[2][0] * m[1][3];

    let coef16 = m[2][0] * m[3][2] - m[3][0] * m[2][2];
    let coef18 = m[1][0] * m[3][2] - m[3][0] * m[1][2];
    let coef19 = m[1][0] * m[2][2] - m[2][0] * m[1][2];

    let coef20 = m[2][0] * m[3][1] - m[3][0] * m[2][1];
    let coef22 = m[1][0] * m[3][1] - m[3][0] * m[1][1];
    let coef23 = m[1][0] * m[2][1] - m[2][0] * m[1][1];


    let fac0 = Float4::new(coef00, coef00, coef02, coef03);
    let fac1 = Float4::new(coef04, coef04, coef06, coef07);
    let fac2 = Float4::new(coef08, coef08, coef10, coef11);
    let fac3 = Float4::new(coef12, coef12, coef14, coef15);
    let fac4 = Float4::new(coef16, coef16, coef18, coef19);
    let fac5 = Float4::new(coef20, coef20, coef22, coef23);


    let vec0 = Float4::new(m[1][0], m[0][0], m[0][0], m[0][0]);
    let vec1 = Float4::new(m[1][1], m[0][1], m[0][1], m[0][1]);
    let vec2 = Float4::new(m[1][2], m[0][2], m[0][2], m[0][2]);
    let vec3 = Float4::new(m[1][3], m[0][3], m[0][3], m[0][3]);


	let inv0 = vector_add(vector_sub(vector_mul(vec1, fac0), vector_mul(vec2, fac1)), vector_mul(vec3, fac2));
	let inv1 = vector_add(vector_sub(vector_mul(vec0, fac0), vector_mul(vec2, fac3)), vector_mul(vec3, fac4));
	let inv2 = vector_add(vector_sub(vector_mul(vec0, fac1), vector_mul(vec1, fac3)), vector_mul(vec3, fac5));
	let inv3 = vector_add(vector_sub(vector_mul(vec0, fac2), vector_mul(vec1, fac4)), vector_mul(vec2, fac5));


	let one_neg_one_neg = Float4::new(1.0, -1.0, 1.0, -1.0);
	let neg_one_neg_one = Float4::new(-1.0, 1.0, -1.0, 1.0);
    let inverse = Float4x4::from_columns(
        vector_mul(inv0, one_neg_one_neg), 
        vector_mul(inv1, neg_one_neg_one), 
        vector_mul(inv2, one_neg_one_neg), 
        vector_mul(inv3, neg_one_neg_one)
    );

    let row0 = Float4::new(inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);
    let det = vector_mul(m[0], row0);
    let det = det.x + det.y + det.z + det.w;

    if det.abs() <= f32::EPSILON {
        return None;
    }

    let recip_det = Float4::fill(det.recip());
    Some(Float4x4::from_columns(
        vector_mul(inverse[0], recip_det),
        vector_mul(inverse[1], recip_det),
        vector_mul(inverse[2], recip_det),
        vector_mul(inverse[3], recip_det),
    ))
}
