#[cfg(target_pointer_width = "32")]
use core::arch::x86::*;

#[cfg(target_pointer_width = "64")]
use core::arch::x86_64::*;

use crate::data::*;


/// This is the data type used in vector operations.
pub type Vector = __m128;

/// This is the data type used in vector operations.
pub type VectorU32 = __m128i;

/// This is the data type used in matrix operation.
pub type Matrix = [__m128; 4];



/// Load vector data from [`Boolean2`].
#[inline(always)]
pub fn load_boolean2(src: Boolean2) -> VectorU32 {
    load_uinteger2(UInteger2::from(src))
}

/// Load vector data from [`Boolean3`].
#[inline(always)]
pub fn load_boolean3(src: Boolean3) -> VectorU32 {
    load_uinteger3(UInteger3::from(src))
}

/// Load vector data from [`Boolean4`].
#[inline(always)]
pub fn load_boolean4(src: Boolean4) -> VectorU32 {
    load_uinteger4(UInteger4::from(src))
}

/// Load vector data from [`Float2`].
#[inline(always)]
pub fn load_float2(src: Float2) -> Vector {
    load_float4(Float4::from(src))
}

/// Load vector data from [`Float3`].
#[inline(always)]
pub fn load_float3(src: Float3) -> Vector {
    load_float4(Float4::from(src))
}

/// Load vector data from [`Float4`].
#[inline]
pub fn load_float4(src: Float4) -> Vector {
    unsafe { _mm_loadu_ps(&src as *const Float4 as *const f32) }
}

/// Load matrix data from [`Float3x3`].
#[inline(always)]
pub fn load_float3x3(src: Float3x3) -> Matrix {
    load_float4x4(Float4x4::from(src))
}

/// Load matrix data from [`Float4x4`].
#[inline(always)]
pub fn load_float4x4(src: Float4x4) -> Matrix {
    [
        load_float4(src.x_axis), 
        load_float4(src.y_axis), 
        load_float4(src.z_axis), 
        load_float4(src.w_axis)
    ]
}

/// Load vector data from [`UInteger2`].
#[inline(always)]
pub fn load_uinteger2(src: UInteger2) -> VectorU32 {
    load_uinteger4(UInteger4::from(src))
}

/// Load vector data from [`UInteger3`].
#[inline(always)]
pub fn load_uinteger3(src: UInteger3) -> VectorU32 {
    load_uinteger4(UInteger4::from(src))
}

/// Load vector data from [`UInteger4`].
#[inline]
pub fn load_uinteger4(src: UInteger4) -> VectorU32 {
    unsafe { _mm_loadu_si128(&src as *const UInteger4 as *const __m128i) }
}

/// Store vector data in [`Boolean2`].
#[inline(always)]
pub fn store_boolean2(src: VectorU32) -> Boolean2 {
    store_uinteger2(src).into()
}

/// Store vector data in [`Boolean3`].
#[inline(always)]
pub fn store_boolean3(src: VectorU32) -> Boolean3 {
    store_uinteger3(src).into()
}

/// Store vector data in [`Boolean4`].
#[inline(always)]
pub fn store_boolean4(src: VectorU32) -> Boolean4 {
    store_uinteger4(src).into()
}

/// Store vector data in [`Float2`].
#[inline(always)]
pub fn store_float2(src: Vector) -> Float2 {
    Float2::from(store_float4(src))
}

/// Store vector data in [`Float3`].
#[inline(always)]
pub fn store_float3(src: Vector) -> Float3 {
    Float3::from(store_float4(src))
}

/// Store vector data in [`Float4`].
#[inline]
pub fn store_float4(src: Vector) -> Float4 {
    let mut dst = Float4::default();
    unsafe { _mm_storeu_ps(&mut dst as *mut Float4 as *mut f32, src) };
    dst
}

/// Store matrix data in [`Float3x3`].
#[inline(always)]
pub fn store_float3x3(src: Matrix) -> Float3x3 {
    Float3x3::from(store_float4x4(src))
}

/// Store matrix data in [`Float4x4`].
#[inline(always)]
pub fn store_float4x4(src: Matrix) -> Float4x4 {
    Float4x4 { 
        x_axis: store_float4(src[0]), 
        y_axis: store_float4(src[1]), 
        z_axis: store_float4(src[2]), 
        w_axis: store_float4(src[3]) 
    }
}

/// Store vector data in [`UInteger2`].
#[inline(always)]
pub fn store_uinteger2(src: VectorU32) -> UInteger2 {
    UInteger2::from(store_uinteger4(src))
}

/// Store vector data in [`UInteger3`].
#[inline(always)]
pub fn store_uinteger3(src: VectorU32) -> UInteger3 {
    UInteger3::from(store_uinteger4(src))
}

/// Store vector data in [`UInteger4`].
#[inline]
pub fn store_uinteger4(src: VectorU32) -> UInteger4 {
    let mut dst = UInteger4::default();
    unsafe { _mm_storeu_si128(&mut dst as *mut UInteger4 as *mut __m128i, src) };
    dst
}



/// Adds two vectors.
#[inline]
pub fn vector_add(a: Vector, b: Vector) -> Vector {
    unsafe { _mm_add_ps(a, b) }
}

/// Subtracts two vectors.
#[inline]
pub fn vector_sub(a: Vector, b: Vector) -> Vector {
    unsafe { _mm_sub_ps(a, b) }
}

/// Multiplies two vectors.
#[inline]
pub fn vector_mul(a: Vector, b: Vector) -> Vector {
    unsafe { _mm_mul_ps(a, b) }
}

/// Divides two vectors.
#[inline]
pub fn vector_div(a: Vector, b: Vector) -> Vector {
    unsafe { _mm_div_ps(a, b) }
}

/// Absolute value on vector elements.
#[inline]
pub fn vector_abs(v: Vector) -> Vector {
    unsafe { 
        let zero = _mm_setzero_ps();
        let negative = _mm_sub_ps(zero, v);
        _mm_max_ps(negative, v)
    }
}

/// Negativizes the given vector.
#[inline]
pub fn vector_neg(v: Vector) -> Vector {
    unsafe { _mm_sub_ps(_mm_setzero_ps(), v) }
}

/// Takes the smaller of the elements of the two vectors.
#[inline]
pub fn vector_min(a: Vector, b: Vector) -> Vector {
    unsafe { _mm_min_ps(a, b) }
}

/// Takes the larger of the elements of the two vectors.
#[inline]
pub fn vector_max(a: Vector, b: Vector) -> Vector {
    unsafe { _mm_max_ps(a, b) }
}

/// Checks if the elements of two given vectors are less.
#[inline]
pub fn vector_lt(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(_mm_castps_si128(_mm_cmplt_ps(a, b))) }
}

/// Checks if the elements of two given vectors are less than or equal.
#[inline]
pub fn vector_le(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(_mm_castps_si128(_mm_cmple_ps(a, b))) }
}

/// Checks if the elements of two given vectors are greater.
#[inline]
pub fn vector_gt(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(_mm_castps_si128(_mm_cmpgt_ps(a, b))) }
}

/// Checks if the elements of two given vectors are greater than or equal.
#[inline]
pub fn vector_ge(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(_mm_castps_si128(_mm_cmpge_ps(a, b))) }
}

/// Checks if the elements of two given vectors are equal.
/// This function does not use [`f32::EPSILON`].
/// 
#[inline]
pub fn vector_eq(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(_mm_castps_si128(_mm_cmpeq_ps(a, b))) }
}

/// Checks if the elements of two given vectors are not equal.
/// This function does not use [`f32::EPSILON`].
/// 
#[inline]
pub fn vector_ne(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(_mm_castps_si128(_mm_cmpneq_ps(a, b))) }
}

/// Returns a vector filled by adding all the elements of the vector.
#[inline]
pub fn vector_sum(v: Vector) -> Vector {
    unsafe {
        let a = _mm_shuffle_ps::<0b_01_00_01_00>(v, v); // [x, y, x, y]
        let b = _mm_shuffle_ps::<0b_11_10_11_10>(v, v); // [z, w, z, w]
        let v = _mm_add_ps(a, b); // [x+z, y+w, x+z, y+w]
        let b = _mm_shuffle_ps::<0b_10_11_00_01>(v, v); // [y+w, x+z, y+w, x+z]
        _mm_add_ps(v, b) // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]
    }
}

/// Transformation of the given vector.
#[inline]
pub fn vector_transform(m: Matrix, v: Vector) -> Vector {
    unsafe {
        let tm = matrix_transpose(m);
        let e0 = vector_mul(tm[0], v);
        let e0 = vector_sum(e0);

        let e1 = vector_mul(tm[1], v);
        let e1 = vector_sum(e1);

        let e2 = vector_mul(tm[2], v);
        let e2 = vector_sum(e2);

        let e3 = vector_mul(tm[3], v);
        let e3 = vector_sum(e3);

        let a = _mm_shuffle_ps::<0b_00_00_00_00>(e0, e1); // [e0, e0, e1, e1]
        let b = _mm_shuffle_ps::<0b_00_00_00_00>(e2, e3); // [e2, e2, e3, e3]
        _mm_shuffle_ps::<0b_10_00_10_00>(a, b) // [e0, e1, e2, e3]
    }
}


/// Length squared of a two-element vector.
#[inline(always)]
pub fn vector2_length_sq(v: Vector) -> f32 {
    vector2_dot(v, v)
}

/// Length squared of a three-element vector.
#[inline(always)]
pub fn vector3_length_sq(v: Vector) -> f32 {
    vector3_dot(v, v)
}

/// Length squared of a four-element vector.
#[inline(always)]
pub fn vector4_length_sq(v: Vector) -> f32 {
    vector4_dot(v, v)
}

/// Length of a two-element vector.
#[inline(always)]
pub fn vector2_length(v: Vector) -> f32 {
    vector2_length_sq(v).sqrt()
}

/// Length of a three-element vector.
#[inline(always)]
pub fn vector3_length(v: Vector) -> f32 {
    vector3_length_sq(v).sqrt()
}

/// Length of a four-element vector.
#[inline(always)]
pub fn vector4_length(v: Vector) -> f32 {
    vector4_length_sq(v).sqrt()
}

/// Normalizes a given two-element vector.
/// If normalization fails, `None` is returned.
#[inline]
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
pub fn vector4_normalize(v: Vector) -> Option<Vector> {
    let len = vector4_length(v);
    if len > 0.0 {
        let len = load_float4(Float4::fill(len));
        Some(vector_div(v, len))
    } else {
        None
    }
}

/// Dot product of a two-element vector.
#[inline]
pub fn vector2_dot(a: Vector, b: Vector) -> f32 {
    unsafe {
        let mul = _mm_mul_ps(a, b); // x2, y2, --, --
        let y2 = _mm_shuffle_ps::<0b00_00_00_01>(mul, mul); // y2, --, --, --
        let sum = _mm_add_ss(mul, y2); // x2+y2, --, --, --
        _mm_cvtss_f32(sum)
    }
}

/// Dot product of a three-element vector.
#[inline]
pub fn vector3_dot(a: Vector, b: Vector) -> f32 {
    unsafe {
        let mul = _mm_mul_ps(a, b); // x2, y2, z2, --
        let y2 = _mm_shuffle_ps::<0b00_00_00_01>(mul, mul); // y2, --, --, --
        let z2 = _mm_shuffle_ps::<0b00_00_00_10>(mul, mul); // z2, --, --, --
        let sum = _mm_add_ss(mul, y2); // x2+y2, --, --, --
        let sum = _mm_add_ss(sum, z2); // x2+y2+z2, --, --, --
        _mm_cvtss_f32(sum)
    }
}

/// Dot product of a four-element vector.
#[inline]
pub fn vector4_dot(a: Vector, b: Vector) -> f32 {
    unsafe {
        let mul = _mm_mul_ps(a, b); // x2, y2, z2, w2
        let high = _mm_shuffle_ps::<0b00_00_11_10>(mul, mul); // z2, w2, --, --
        let sum = _mm_add_ps(mul, high); // x2+z2, y2+w2, --, --
        let high = _mm_shuffle_ps::<0b00_00_00_01>(sum, sum); // y2+w2, --, --, --
        let sum = _mm_add_ps(sum, high); // x2+y2+z2+w2, --, --, --
        _mm_cvtss_f32(sum)
    }
}

/// Cross product of a three-element vector
pub fn vector3_cross(a: Vector, b: Vector) -> Vector {
    const BIT_MASK: [u32; 4] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000];
    unsafe {
        // a x b = [ay*bz-az*by, az*bx-ax*bz, ax*by-ay*bx]
        // v0 = { ay, az, ax, -- }
        // v1 = { bz, bx, by, -- }
        // v2 = { az, ax, ay, -- }
        // v3 = { by, bz, bx, -- }
        // v0*v1-v2*v3
        //

        let v0 = _mm_shuffle_ps::<0b00_00_10_01>(a, a); // ay, az, ax, --
        let v1 = _mm_shuffle_ps::<0b00_01_00_10>(b, b); // bz, bx, by, --
        let v2 = _mm_shuffle_ps::<0b00_01_00_10>(a, a); // az, ax, ay, --
        let v3 = _mm_shuffle_ps::<0b00_00_10_01>(b, b); // by, bz, bx, --
        let res = _mm_sub_ps(_mm_mul_ps(v0, v1), _mm_mul_ps(v2, v3));

        let mask = _mm_loadu_ps(&BIT_MASK as *const u32 as *const f32);
        _mm_and_ps(mask, res)
    }
}

/// Checks if two elements of two given vectors are equal.
/// This function compares using [`f32::EPSILON`].
#[inline]
pub fn vector2_eq(a: Vector, b: Vector) -> bool {
    let diff = vector_sub(a, b);
    let len = vector2_length_sq(diff);
    return len <= f32::EPSILON;
}

/// Checks if three elements of two given vectors are equal.
/// This function compares using [`f32::EPSILON`].
#[inline]
pub fn vector3_eq(a: Vector, b: Vector) -> bool {
    let diff = vector_sub(a, b);
    let len = vector3_length_sq(diff);
    return len <= f32::EPSILON;
}

/// Checks if four elements of two given vectors are equal.
/// This function compares using [`f32::EPSILON`].
#[inline]
pub fn vector4_eq(a: Vector, b: Vector) -> bool {
    let diff = vector_sub(a, b);
    let len = vector4_length_sq(diff);
    return len <= f32::EPSILON;
}



/// Dot product of a two quaternions.
#[inline(always)]
pub fn quaternion_dot(a: Vector, b: Vector) -> f32 {
    vector4_dot(a, b)
}

/// Multiplies two quaternions.
#[inline]
pub fn quaternion_mul(a: Vector, b: Vector) -> Vector {
    unsafe {
        // x: aw*bx + ax*bw + ay*bz - az*by
        // y: aw*by - ax*bz + ay*bw + az*bx
        // z: aw*bz + ax*by - ay*bx + az*bw
        // w: aw*bw - ax*bx - ay*by - az*bz
        //
        let b_neg = vector_neg(b); // [-bx, -by, -bz, -bw]
        
        let temp = _mm_shuffle_ps::<0b_00_01_00_01>(b_neg, b); // [-by, -bx, by, bx]
        let temp = _mm_shuffle_ps::<0b_11_00_10_11>(b, temp); // [bw, bz, -by, bx]
        let i_res = _mm_mul_ps(a, temp); // [ax*bw, ay*bz, -az*by, aw*bx]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(i_res, i_res); // [ax*bw, ay*bz, ax*bw, ay*bz]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(i_res, i_res); // [-az*by, aw*bx, -az*by, aw*bx]
        let i_res = _mm_add_ps(low, high); // [ax*bw-az*by, ay*bz+aw*bx, ax*bw-az*by, ay*bz+aw*bx]
        let low = _mm_shuffle_ps::<0b_00_00_00_00>(i_res, i_res); // [ax*bw-az*by, ...]
        let high = _mm_shuffle_ps::<0b_01_01_01_01>(i_res, i_res); // [ay*bz+aw*bx, ...]
        let i = _mm_add_ps(low, high); // [ax*bw+ay*bz-az*by+aw*bx, ...]

        let temp = _mm_shuffle_ps::<0b_11_10_11_10>(b_neg, b); // [-bz, -bw, bz, bw]
        let temp = _mm_shuffle_ps::<0b_01_00_11_00>(temp, b); // [-bz, bw, bx, by]
        let j_res = _mm_mul_ps(a, temp); // [-ax*bz, ay*bw, az*bx, aw*by]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(j_res, j_res); // [-ax*bz, ay*bw, -ax*bz, ay*bw]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(j_res, j_res); // [az*bx, aw*by, az*bx, aw*by]
        let j_res = _mm_add_ps(low, high); // [az*bx-ax*bz, ay*bw+aw*by, az*bx-ax*bz, ay*bw+aw*by]
        let low = _mm_shuffle_ps::<0b_00_00_00_00>(j_res, j_res); // [az*bx-ax*bz, ...]
        let high = _mm_shuffle_ps::<0b_01_01_01_01>(j_res, j_res); // [ay*bw+aw*by, ...]
        let j = _mm_add_ps(low, high); // [-ax*bz+ay*bw+az*bx+aw*by, ...]

        let temp = _mm_shuffle_ps::<0b_00_01_00_10>(b_neg, b); // [-by, -bx, by, bx]
        let temp = _mm_shuffle_ps::<0b_10_11_01_10>(temp, b); // [by, -bx, bw, bz]
        let k_res = _mm_mul_ps(a, temp); // [ax*by, -ay*bx, az*bw, aw*bz]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(k_res, k_res); // [ax*by, -ay*bx, ax*by, -ay*bx]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(k_res, k_res); // [az*bw, aw*bz, az*bw, aw*bz]
        let k_res = _mm_add_ps(low, high); // [ax*by+az*bw, -ay*bx+aw*bz, ax*by+az*bw, -ay*bx+aw*bz]
        let low = _mm_shuffle_ps::<0b_00_00_00_00>(k_res, k_res); // [ax*by+az*bw, ...]
        let high = _mm_shuffle_ps::<0b_01_01_01_01>(k_res, k_res); // [-ay*bx+aw*bz, ...]
        let k = _mm_add_ps(low, high); // ax*by-ay*bx+az*bw+aw*bz, ...]


        let temp = _mm_shuffle_ps::<0b_11_10_11_10>(b_neg, b); // [-bz, -bw, bz, bw]
        let temp = _mm_shuffle_ps::<0b_11_00_01_00>(b_neg, temp); // [-bx, -by, -bz, bw]
        let w_res = _mm_mul_ps(a, temp); // [-ax*bx, -ay*by, -az*bz, aw*bw]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(w_res, w_res); // [-ax*bx, -ay*by, -ax*bx, -ay*by]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(w_res, w_res); // [-az*bz, aw*bw, -az*bz, aw*bw]
        let w_res = _mm_add_ps(low, high); // [-ax*bx-az*bz, -ay*by+aw*bw, -ax*bx-az*bz, -ay*by+aw*bw]
        let low = _mm_shuffle_ps::<0b_00_00_00_00>(w_res, w_res); // [-ax*bx-az*bz, ...]
        let high = _mm_shuffle_ps::<0b_01_01_01_01>(w_res, w_res); // [-ay*by+aw*bw, ...]
        let w = _mm_add_ps(low, high); // aw*bw-ax*bx-ay*by-az*bz, ...]

        let low = _mm_shuffle_ps::<0b_00_00_00_00>(i, j); // [i, i, j, j]
        let high = _mm_shuffle_ps::<0b_00_00_00_00>(k, w); // [k, k, w, w]
        _mm_shuffle_ps::<0b_10_00_10_00>(low, high) // [i, j, k, w]
    }
}

/// Length sqared of a quaternion.
#[inline(always)]
pub fn quaternion_length_sq(q: Vector) -> f32 {
    vector4_length_sq(q)
}

/// Length of a quaternion.
#[inline(always)]
pub fn quaternion_length(q: Vector) -> f32 {
    vector4_length(q)
}

/// Normalizes a given quaternion. 
/// If normalization fails, `None` is returned.
#[inline(always)]
pub fn quaternion_normalize(q: Vector) -> Option<Vector> {
    vector4_normalize(q)
}

/// Returns the conjugate of the quaternion.
#[inline]
pub fn quaternion_conjugate(q: Vector) -> Vector {
    const NEG: [f32; 4] = [-1.0, -1.0, -1.0, 1.0];
    unsafe {
        let neg = _mm_load_ps(&NEG as *const f32);
        _mm_mul_ps(neg, q)
    }
}

/// Returns the inverse of the quaternion. 
/// If normalization fails, `None` is returned.
#[inline]
pub fn quaternion_inverse(q: Vector) -> Option<Vector> {
    quaternion_normalize(q)
        .map(|q| quaternion_conjugate(q))
}



/// Adds two matrices.
#[inline(always)]
pub fn matrix_add(a: Matrix, b: Matrix) -> Matrix {
    [
        vector_add(a[0], b[0]), 
        vector_add(a[1], b[1]), 
        vector_add(a[2], b[2]), 
        vector_add(a[3], b[3])
    ]
}

/// Subtracts two matrices.
#[inline(always)]
pub fn matrix_sub(a: Matrix, b: Matrix) -> Matrix {
    [
        vector_sub(a[0], b[0]), 
        vector_sub(a[1], b[1]), 
        vector_sub(a[2], b[2]), 
        vector_sub(a[3], b[3])
    ]
}

/// Negativizes the given matrix.
#[inline(always)]
pub fn matrix_neg(m: Matrix) -> Matrix {
    [
        vector_neg(m[0]), 
        vector_neg(m[1]), 
        vector_neg(m[2]), 
        vector_neg(m[3])
    ]
}

/// Multiplies two matrices.
#[inline]
pub fn matrix_mul(a: Matrix, b: Matrix) -> Matrix {
    unsafe {
        let ta = matrix_transpose(a);

        let m00 = vector_mul(ta[0], b[0]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m00, m00); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m00, m00); // [z, w, z, w]
        let m00 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m00, m00); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m00, m00); // [y+w, ...]
        let m00 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m01 = vector_mul(ta[1], b[0]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m01, m01); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m01, m01); // [z, w, z, w]
        let m01 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m01, m01); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m01, m01); // [y+w, ...]
        let m01 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m02 = vector_mul(ta[2], b[0]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m02, m02); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m02, m02); // [z, w, z, w]
        let m02 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m02, m02); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m02, m02); // [y+w, ...]
        let m02 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m03 = vector_mul(ta[3], b[0]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m03, m03); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m03, m03); // [z, w, z, w]
        let m03 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m03, m03); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m03, m03); // [y+w, ...]
        let m03 = _mm_add_ps(low, high); // [x+y+z+w, ...]

        let low = _mm_shuffle_ps::<0b_00_00_00_00>(m00, m01); // [m00, m00, m01, m01]
        let high = _mm_shuffle_ps::<0b_00_00_00_00>(m02, m03); // [m02, m02, m03, m03]
        let v0 = _mm_shuffle_ps::<0b_10_00_10_00>(low, high); // [m00, m01, m02, m03]
        
        let m10 = vector_mul(ta[0], b[1]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m10, m10); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m10, m10); // [z, w, z, w]
        let m10 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m10, m10); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m10, m10); // [y+w, ...]
        let m10 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m11 = vector_mul(ta[1], b[1]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m11, m11); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m11, m11); // [z, w, z, w]
        let m11 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m11, m11); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m11, m11); // [y+w, ...]
        let m11 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m12 = vector_mul(ta[2], b[1]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m12, m12); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m12, m12); // [z, w, z, w]
        let m12 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m12, m12); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m12, m12); // [y+w, ...]
        let m12 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m13 = vector_mul(ta[3], b[1]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m13, m13); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m13, m13); // [z, w, z, w]
        let m13 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m13, m13); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m13, m13); // [y+w, ...]
        let m13 = _mm_add_ps(low, high); // [x+y+z+w, ...]

        let low = _mm_shuffle_ps::<0b_00_00_00_00>(m10, m11); // [m10, m10, m11, m11]
        let high = _mm_shuffle_ps::<0b_00_00_00_00>(m12, m13); // [m12, m12, m13, m13]
        let v1 = _mm_shuffle_ps::<0b_10_00_10_00>(low, high); // [m10, m11, m12, m13]
        
        let m20 = vector_mul(ta[0], b[2]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m20, m20); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m20, m20); // [z, w, z, w]
        let m20 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m20, m20); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m20, m20); // [y+w, ...]
        let m20 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m21 = vector_mul(ta[1], b[2]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m21, m21); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m21, m21); // [z, w, z, w]
        let m21 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m21, m21); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m21, m21); // [y+w, ...]
        let m21 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m22 = vector_mul(ta[2], b[2]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m22, m22); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m22, m22); // [z, w, z, w]
        let m22 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m22, m22); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m22, m22); // [y+w, ...]
        let m22 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m23 = vector_mul(ta[3], b[2]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m23, m23); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m23, m23); // [z, w, z, w]
        let m23 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m23, m23); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m23, m23); // [y+w, ...]
        let m23 = _mm_add_ps(low, high); // [x+y+z+w, ...]

        let low = _mm_shuffle_ps::<0b_00_00_00_00>(m20, m21); // [m20, m20, m21, m21]
        let high = _mm_shuffle_ps::<0b_00_00_00_00>(m22, m23); // [m22, m22, m23, m23]
        let v2 = _mm_shuffle_ps::<0b_10_00_10_00>(low, high); // [m20, m21, m22, m23]
        
        let m30 = vector_mul(ta[0], b[3]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m30, m30); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m30, m30); // [z, w, z, w]
        let m30 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m30, m30); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m30, m30); // [y+w, ...]
        let m30 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m31 = vector_mul(ta[1], b[3]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m31, m31); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m31, m31); // [z, w, z, w]
        let m31 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m31, m31); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m31, m31); // [y+w, ...]
        let m31 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m32 = vector_mul(ta[2], b[3]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m32, m32); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m32, m32); // [z, w, z, w]
        let m32 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m32, m32); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m32, m32); // [y+w, ...]
        let m32 = _mm_add_ps(low, high); // [x+y+z+w, ...]
        
        let m33 = vector_mul(ta[3], b[3]); // [x, y, z, w]
        let low = _mm_shuffle_ps::<0b_01_00_01_00>(m33, m33); // [x, y, x, y]
        let high = _mm_shuffle_ps::<0b_11_10_11_10>(m33, m33); // [z, w, z, w]
        let m33 = _mm_add_ps(low, high); // [x+z, y+w, x+z, y+w]
        let low = _mm_shuffle_ps::<0b_10_00_10_00>(m33, m33); // [x+z, ...]
        let high = _mm_shuffle_ps::<0b_11_01_11_01>(m33, m33); // [y+w, ...]
        let m33 = _mm_add_ps(low, high); // [x+y+z+w, ...]

        let low = _mm_shuffle_ps::<0b_00_00_00_00>(m30, m31); // [m30, m30, m31, m31]
        let high = _mm_shuffle_ps::<0b_00_00_00_00>(m32, m33); // [m32, m32, m33, m33]
        let v3 = _mm_shuffle_ps::<0b_10_00_10_00>(low, high); // [m30, m31, m32, m33]

        [v0, v1, v2, v3]
    }
}

/// Transpose of a matrix.
#[inline]
pub fn matrix_transpose(m: Matrix) -> Matrix {
    // matrix m
    // A, B, C, D
    // E, F, G, H
    // I, J, K, L
    // M, N, O, P
    //
    unsafe {
        let low01 = _mm_shuffle_ps::<0b_01_00_01_00>(m[0], m[1]); // [A, B, E, F]
        let low23 = _mm_shuffle_ps::<0b_01_00_01_00>(m[2], m[3]); // [I, J, M, N]
        let high01 = _mm_shuffle_ps::<0b_11_10_11_10>(m[0], m[1]); // [C, D, G, H]
        let high23 = _mm_shuffle_ps::<0b_11_10_11_10>(m[2], m[3]); // [K, L, O, P]

        let v0 = _mm_shuffle_ps::<0b_10_00_10_00>(low01, low23); //[A, E, I, M]
        let v1 = _mm_shuffle_ps::<0b_11_01_11_01>(low01, low23); // [B, F, J, N]
        let v2 = _mm_shuffle_ps::<0b_10_00_10_00>(high01, high23); // [C, G, K, O]
        let v3 = _mm_shuffle_ps::<0b_11_01_11_01>(high01, high23); // [D, H, L, P]

        [v0, v1, v2, v3]
    }
}

/// determinant of a matrix.
pub fn matrix_determinant(m: Matrix) -> f32 {
    const ZERO: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
    const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];

    unsafe {
        let zero = _mm_load_ps(&ZERO as *const f32);
        let one_neg_one_neg = _mm_load_ps(&ONE_NEG_ONE_NEG as *const f32);
        let neg_one_neg_one = _mm_load_ps(&NEG_ONE_NEG_ONE as *const f32);

        let m00_m00_m00_m01 = _mm_shuffle_ps::<0b_01_00_00_00>(m[0], m[0]);
        let m11_m12_m13_m12 = _mm_shuffle_ps::<0b_10_11_10_01>(m[1], m[1]);
        let m10_m10_m10_m11 = _mm_shuffle_ps::<0b_01_00_00_00>(m[1], m[1]);
        let m01_m02_m03_m02 = _mm_shuffle_ps::<0b_10_11_10_01>(m[0], m[0]);
        let a = _mm_mul_ps(m00_m00_m00_m01, m11_m12_m13_m12);
        let b = _mm_mul_ps(m10_m10_m10_m11, m01_m02_m03_m02);
        let t0 = _mm_sub_ps(a, b);

        let m01_m02_m22_m21 = _mm_shuffle_ps::<0b_01_10_10_01>(m[0], m[2]);
        let m13_m13_m33_m33 = _mm_shuffle_ps::<0b_11_11_11_11>(m[1], m[3]);
        let m11_m12_m32_m31 = _mm_shuffle_ps::<0b_01_10_10_01>(m[1], m[3]);
        let m03_m03_m23_m23 = _mm_shuffle_ps::<0b_11_11_11_11>(m[0], m[2]);
        let a = _mm_mul_ps(m01_m02_m22_m21, m13_m13_m33_m33);
        let b = _mm_mul_ps(m11_m12_m32_m31, m03_m03_m23_m23);
        let t1 = _mm_sub_ps(a, b);

        let m21_m20_m20_m20 = _mm_shuffle_ps::<0b_00_00_00_01>(m[2], m[2]);
        let m32_m33_m32_m31 = _mm_shuffle_ps::<0b_01_10_11_10>(m[3], m[3]);
        let m31_m30_m30_m30 = _mm_shuffle_ps::<0b_00_00_00_01>(m[3], m[3]);
        let m22_m23_m22_m21 = _mm_shuffle_ps::<0b_01_10_11_10>(m[2], m[2]);
        let a = _mm_mul_ps(m21_m20_m20_m20, m32_m33_m32_m31);
        let b = _mm_mul_ps(m31_m30_m30_m30, m22_m23_m22_m21);
        let t2 = _mm_sub_ps(a, b);

        let t00_t01 = _mm_shuffle_ps::<0b_00_00_01_00>(t0, zero);
        let t12_t13 = _mm_shuffle_ps::<0b_00_00_11_10>(t1, zero);
        let r0 = _mm_mul_ps(t00_t01, t12_t13);
        let r0 = _mm_mul_ps(r0, one_neg_one_neg);

        let t02_t03 = _mm_shuffle_ps::<0b_00_00_11_10>(t0, zero);
        let t20_t21 = _mm_shuffle_ps::<0b_00_00_01_00>(t2, zero);
        let r1 = _mm_mul_ps(t02_t03, t20_t21);

        let t10_t11 = _mm_shuffle_ps::<0b_00_00_01_00>(t1, zero);
        let t22_t23 = _mm_shuffle_ps::<0b_00_00_11_10>(t2, zero);
        let r2 = _mm_mul_ps(t10_t11, t22_t23);
        let r2 = _mm_mul_ps(r2, neg_one_neg_one);

        let det = _mm_add_ps(r0, r1);
        let det = _mm_add_ps(det, r2);
        let det = vector_sum(det);
        _mm_cvtss_f32(det)
    }
}

/// inverse of a matrix.
/// 
/// If the inverse of a matrix cannot be calculated, returns `None`.
/// 
#[must_use]
pub fn matrix_inverse(m: Matrix) -> Option<Matrix> {
    const ZERO: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
    const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];

    unsafe {
        let zero = _mm_load_ps(&ZERO as *const f32);
        let one_neg_one_neg = _mm_load_ps(&ONE_NEG_ONE_NEG as *const f32);
        let neg_one_neg_one = _mm_load_ps(&NEG_ONE_NEG_ONE as *const f32);

        let m00_m00_m00_m01 = _mm_shuffle_ps::<0b_01_00_00_00>(m[0], m[0]);
        let m11_m12_m13_m12 = _mm_shuffle_ps::<0b_10_11_10_01>(m[1], m[1]);
        let m10_m10_m10_m11 = _mm_shuffle_ps::<0b_01_00_00_00>(m[1], m[1]);
        let m01_m02_m03_m02 = _mm_shuffle_ps::<0b_10_11_10_01>(m[0], m[0]);
        let a = _mm_mul_ps(m00_m00_m00_m01, m11_m12_m13_m12);
        let b = _mm_mul_ps(m10_m10_m10_m11, m01_m02_m03_m02);
        let t0 = _mm_sub_ps(a, b);

        let m01_m02_m22_m21 = _mm_shuffle_ps::<0b_01_10_10_01>(m[0], m[2]);
        let m13_m13_m33_m33 = _mm_shuffle_ps::<0b_11_11_11_11>(m[1], m[3]);
        let m11_m12_m32_m31 = _mm_shuffle_ps::<0b_01_10_10_01>(m[1], m[3]);
        let m03_m03_m23_m23 = _mm_shuffle_ps::<0b_11_11_11_11>(m[0], m[2]);
        let a = _mm_mul_ps(m01_m02_m22_m21, m13_m13_m33_m33);
        let b = _mm_mul_ps(m11_m12_m32_m31, m03_m03_m23_m23);
        let t1 = _mm_sub_ps(a, b);

        let m21_m20_m20_m20 = _mm_shuffle_ps::<0b_00_00_00_01>(m[2], m[2]);
        let m32_m33_m32_m31 = _mm_shuffle_ps::<0b_01_10_11_10>(m[3], m[3]);
        let m31_m30_m30_m30 = _mm_shuffle_ps::<0b_00_00_00_01>(m[3], m[3]);
        let m22_m23_m22_m21 = _mm_shuffle_ps::<0b_01_10_11_10>(m[2], m[2]);
        let a = _mm_mul_ps(m21_m20_m20_m20, m32_m33_m32_m31);
        let b = _mm_mul_ps(m31_m30_m30_m30, m22_m23_m22_m21);
        let t2 = _mm_sub_ps(a, b);

        let t00_t01 = _mm_shuffle_ps::<0b_00_00_01_00>(t0, zero);
        let t12_t13 = _mm_shuffle_ps::<0b_00_00_11_10>(t1, zero);
        let r0 = _mm_mul_ps(t00_t01, t12_t13);
        let r0 = _mm_mul_ps(r0, one_neg_one_neg);

        let t02_t03 = _mm_shuffle_ps::<0b_00_00_11_10>(t0, zero);
        let t20_t21 = _mm_shuffle_ps::<0b_00_00_01_00>(t2, zero);
        let r1 = _mm_mul_ps(t02_t03, t20_t21);

        let t10_t11 = _mm_shuffle_ps::<0b_00_00_01_00>(t1, zero);
        let t22_t23 = _mm_shuffle_ps::<0b_00_00_11_10>(t2, zero);
        let r2 = _mm_mul_ps(t10_t11, t22_t23);
        let r2 = _mm_mul_ps(r2, neg_one_neg_one);

        let det = _mm_add_ps(r0, r1);
        let det = _mm_add_ps(det, r2);
        let det = vector_sum(det);
        let val = _mm_cvtss_f32(det);

        if val.abs() <= f32::EPSILON {
            return None;
        }

        let m00_m01_m10_m11 = _mm_shuffle_ps::<0b_01_00_01_00>(m[0], m[1]);
        let m02_m03_m12_m13 = _mm_shuffle_ps::<0b_11_10_11_10>(m[0], m[1]);
        let m20_m21_m30_m31 = _mm_shuffle_ps::<0b_01_00_01_00>(m[2], m[3]);
        let m22_m23_m32_m33 = _mm_shuffle_ps::<0b_11_10_11_10>(m[2], m[3]);
        let m00_m10_m01_m11 = _mm_shuffle_ps::<0b_11_01_10_00>(m00_m01_m10_m11, m00_m01_m10_m11);
        let m02_m12_m03_m13 = _mm_shuffle_ps::<0b_11_01_10_00>(m02_m03_m12_m13, m02_m03_m12_m13);
        let m20_m30_m21_m31 = _mm_shuffle_ps::<0b_11_01_10_00>(m20_m21_m30_m31, m20_m21_m30_m31);
        let m22_m32_m23_m33 = _mm_shuffle_ps::<0b_11_01_10_00>(m22_m23_m32_m33, m22_m23_m32_m33);
        let m00_m10_m20_m30 = _mm_shuffle_ps::<0b_01_00_01_00>(m00_m10_m01_m11, m20_m30_m21_m31);
        let m01_m11_m21_m31 = _mm_shuffle_ps::<0b_11_10_11_10>(m00_m10_m01_m11, m20_m30_m21_m31);
        let m02_m12_m22_m32 = _mm_shuffle_ps::<0b_01_00_01_00>(m02_m12_m03_m13, m22_m32_m23_m33);
        let m03_m13_m23_m33 = _mm_shuffle_ps::<0b_11_10_11_10>(m02_m12_m03_m13, m22_m32_m23_m33);

        let m10_m00_m30_m20 = _mm_shuffle_ps::<0b_10_11_00_01>(m00_m10_m20_m30, m00_m10_m20_m30);
        let m11_m01_m31_m21 = _mm_shuffle_ps::<0b_10_11_00_01>(m01_m11_m21_m31, m01_m11_m21_m31);
        let m12_m02_m32_m22 = _mm_shuffle_ps::<0b_10_11_00_01>(m02_m12_m22_m32, m02_m12_m22_m32);
        let m13_m03_m33_m23 = _mm_shuffle_ps::<0b_10_11_00_01>(m03_m13_m23_m33, m03_m13_m23_m33);

        let t12_t12_t11_t11 = _mm_shuffle_ps::<0b_01_01_10_10>(t1, t1);
        let t13_t13_t10_t10 = _mm_shuffle_ps::<0b_00_00_11_11>(t1, t1);
        let t20_t20_t03_t03 = _mm_shuffle_ps::<0b_11_11_00_00>(t2, t0);
        let t21_t21_t02_t02 = _mm_shuffle_ps::<0b_10_10_01_01>(t2, t0);
        let t22_t22_t01_t01 = _mm_shuffle_ps::<0b_01_01_10_10>(t2, t0);
        let t23_t23_t00_t00 = _mm_shuffle_ps::<0b_00_00_11_11>(t2, t0);

        let r0 = _mm_mul_ps(m11_m01_m31_m21, t12_t12_t11_t11);
        let r0 = _mm_mul_ps(r0, one_neg_one_neg);

        let r1 = _mm_mul_ps(m12_m02_m32_m22, t13_t13_t10_t10);
        let r1 = _mm_mul_ps(r1, neg_one_neg_one);

        let r2 = _mm_mul_ps(m13_m03_m33_m23, t20_t20_t03_t03);
        let r2 = _mm_mul_ps(r2, one_neg_one_neg);

        let col0 = _mm_add_ps(r0, r1);
        let col0 = _mm_add_ps(col0, r2);
        let col0 = _mm_div_ps(col0, det);

        
        let r0 = _mm_mul_ps(m10_m00_m30_m20, t12_t12_t11_t11);
        let r0 = _mm_mul_ps(r0, neg_one_neg_one);

        let r1 = _mm_mul_ps(m12_m02_m32_m22, t21_t21_t02_t02);
        let r1 = _mm_mul_ps(r1, one_neg_one_neg);

        let r2 = _mm_mul_ps(m13_m03_m33_m23, t22_t22_t01_t01);
        let r2 = _mm_mul_ps(r2, neg_one_neg_one);

        let col1 = _mm_add_ps(r0, r1);
        let col1 = _mm_add_ps(col1, r2);
        let col1 = _mm_div_ps(col1, det);


        let r0 = _mm_mul_ps(m10_m00_m30_m20, t13_t13_t10_t10);
        let r0 = _mm_mul_ps(r0, one_neg_one_neg);
        
        let r1 = _mm_mul_ps(m11_m01_m31_m21, t21_t21_t02_t02);
        let r1 = _mm_mul_ps(r1, neg_one_neg_one);

        let r2 = _mm_mul_ps(m13_m03_m33_m23, t23_t23_t00_t00);
        let r2 = _mm_mul_ps(r2, one_neg_one_neg);

        let col2 = _mm_add_ps(r0, r1);
        let col2 = _mm_add_ps(col2, r2);
        let col2 = _mm_div_ps(col2, det);


        let r0 = _mm_mul_ps(m10_m00_m30_m20, t20_t20_t03_t03);
        let r0 = _mm_mul_ps(r0, neg_one_neg_one);

        let r1 = _mm_mul_ps(m11_m01_m31_m21, t22_t22_t01_t01);
        let r1 = _mm_mul_ps(r1, one_neg_one_neg);

        let r2 = _mm_mul_ps(m12_m02_m32_m22, t23_t23_t00_t00);
        let r2 = _mm_mul_ps(r2, neg_one_neg_one);

        let col3 = _mm_add_ps(r0, r1);
        let col3 = _mm_add_ps(col3, r2);
        let col3 = _mm_div_ps(col3, det);

        Some([col0, col1, col2, col3])
    }
}
