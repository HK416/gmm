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

/// Determinant of a matrix.
#[inline]
pub fn matrix_determinant(m: Matrix) -> f32 {
    // det = m00 * (m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m13*m22*m31 - m12*m21*m33 - m11*m23*m32)
    //      - m10 * (m01*m22*m33 + m02*m23*m31 + m03*m21*m32 - m03*m22*m31 - m02*m21*m33 - m01*m23*m32)
    //      + m20 * (m01*m12*m33 + m02*m13*m33 - m01*m13*m32 - m03*m12*m31 - m02*m11*m33 - m01*m13*m32)
    //      - m30 * (m01*m12*m23 + m02*m13*m21 + m03*m11*m22 - m03*m12*m21 - m02*m11*m23 - m01*m13*m22)
    //
    const BIT_MASK: [u32; 4] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000];
    unsafe {
        let m00 = _mm_shuffle_ps::<0b_00_00_00_00>(m[0], m[0]); // [m00, ...]

        let tmp0 =_mm_shuffle_ps::<0b_00_11_10_01>(m[1], m[1]); // [m11, m12, m13, m10]
        let tmp1 = _mm_shuffle_ps::<0b_00_01_11_10>(m[2], m[2]); // [m22, m23, m21, m20]
        let tmp2 = _mm_shuffle_ps::<0b_00_10_01_11>(m[3], m[3]); // [m33, m31, m32, m30]
        let temp = _mm_mul_ps(tmp0, tmp1); // [m11*m22, m12*m23, m13*m21, --]
        let v0 = _mm_mul_ps(temp, tmp2); // [m11*m22*m33, m12*m23*m31, m13*m21*m32, --]

        let tmp0 = _mm_shuffle_ps::<0b_00_01_10_11>(m[1], m[1]); // [m13, m12, m11, m10]
        let tmp1 = _mm_shuffle_ps::<0b_00_11_01_10>(m[2], m[2]); // [m22, m21, m23, m20]
        let tmp2 = _mm_shuffle_ps::<0b_00_10_11_01>(m[3], m[3]); // [m31, m33, m32, m30]
        let temp = _mm_mul_ps(tmp0, tmp1); // [m13*m22, m12*m21, m11*m23, --]
        let v1 = _mm_mul_ps(temp, tmp2); // [m13*m22*m31, m12*m21*m33, m11*m23*m32, --]
        let temp = _mm_sub_ps(v0, v1); // [m11*m22*m33-m13*m22*m31, m12*m23*m31-m12*m21*m33, m13*m21*m32-m11*m23*m32, --]
        let res0 = _mm_mul_ps(m00, temp);


        let m10 = _mm_shuffle_ps::<0b_00_00_00_00>(m[1], m[1]); // [m10, ...]

        let tmp0 =_mm_shuffle_ps::<0b_00_11_10_01>(m[0], m[0]); // [m01, m02, m03, m00]
        let tmp1 = _mm_shuffle_ps::<0b_00_01_11_10>(m[2], m[2]); // [m22, m23, m21, m20]
        let tmp2 = _mm_shuffle_ps::<0b_00_10_01_11>(m[3], m[3]); // [m33, m31, m32, m30]
        let temp = _mm_mul_ps(tmp0, tmp1); // [m01*m22, m02*m23, m03*m21, --]
        let v0 = _mm_mul_ps(temp, tmp2); // [m01*m22*m33, m02*m23*m31, m03*m21*m32, --]

        let tmp0 = _mm_shuffle_ps::<0b_00_01_10_11>(m[0], m[0]); // [m03, m02, m01, m00]
        let tmp1 = _mm_shuffle_ps::<0b_00_11_01_10>(m[2], m[2]); // [m22, m21, m23, m20]
        let tmp2 = _mm_shuffle_ps::<0b_00_10_11_01>(m[3], m[3]); // [m31, m33, m32, m30]
        let temp = _mm_mul_ps(tmp0, tmp1); // [m03*m22, m02*m21, m01*m23, --]
        let v1 = _mm_mul_ps(temp, tmp2); // [m03*m22*m31, m02*m21*m33, m01*m23*m32, --]
        let temp = _mm_sub_ps(v0, v1); // [m01*m22*m33-m03*m22*m31, m02*m23*m31-m02*m21*m33, m03*m21*m32-m01*m23*m32, --]
        let res1 = _mm_mul_ps(m10, temp);

        
        let m20 = _mm_shuffle_ps::<0b_00_00_00_00>(m[2], m[2]); // [m20, ...]
        
        let tmp0 =_mm_shuffle_ps::<0b_00_11_10_01>(m[0], m[0]); // [m01, m02, m03, m00]
        let tmp1 = _mm_shuffle_ps::<0b_00_01_11_10>(m[1], m[1]); // [m12, m13, m11, m10]
        let tmp2 = _mm_shuffle_ps::<0b_00_10_01_11>(m[3], m[3]); // [m33, m31, m32, m30]
        let temp = _mm_mul_ps(tmp0, tmp1); // [m01*m12, m02*m13, m03*m11, --]
        let v0 = _mm_mul_ps(temp, tmp2); // [m01*m12*m33, m02*m13*m31, m03*m11*m32, --]

        let tmp0 = _mm_shuffle_ps::<0b_00_01_10_11>(m[0], m[0]); // [m03, m02, m01, m00]
        let tmp1 = _mm_shuffle_ps::<0b_00_11_01_10>(m[1], m[1]); // [m12, m11, m13, m20]
        let tmp2 = _mm_shuffle_ps::<0b_00_10_11_01>(m[3], m[3]); // [m31, m33, m32, m30]
        let temp = _mm_mul_ps(tmp0, tmp1); // [m03*m12, m02*m11, m01*m13, --]
        let v1 = _mm_mul_ps(temp, tmp2); // [m03*m12*m31, m02*m11*m33, m01*m13*m32, --]
        let temp = _mm_sub_ps(v0, v1); // [m01*m12*m33-m03*m12*m31, m02*m13*m31-m02*m11*m33, m03*m11*m32-m01*m13*m32, --]
        let res2 = _mm_mul_ps(m20, temp);


        let m30 = _mm_shuffle_ps::<0b_00_00_00_00>(m[3], m[3]); // [m30, ...]
        
        let tmp0 =_mm_shuffle_ps::<0b_00_11_10_01>(m[0], m[0]); // [m01, m02, m03, m00]
        let tmp1 = _mm_shuffle_ps::<0b_00_01_11_10>(m[1], m[1]); // [m12, m13, m11, m10]
        let tmp2 = _mm_shuffle_ps::<0b_00_10_01_11>(m[2], m[2]); // [m23, m21, m22, m20]
        let temp = _mm_mul_ps(tmp0, tmp1); // [m01*m12, m02*m13, m03*m11, --]
        let v0 = _mm_mul_ps(temp, tmp2); // [m01*m12*m23, m02*m13*m21, m03*m11*m22, --]

        let tmp0 = _mm_shuffle_ps::<0b_00_01_10_11>(m[0], m[0]); // [m03, m02, m01, m00]
        let tmp1 = _mm_shuffle_ps::<0b_00_11_01_10>(m[1], m[1]); // [m12, m11, m13, m20]
        let tmp2 = _mm_shuffle_ps::<0b_00_10_11_01>(m[2], m[2]); // [m21, m23, m22, m20]
        let temp = _mm_mul_ps(tmp0, tmp1); // [m03*m12, m02*m11, m01*m13, --]
        let v1 = _mm_mul_ps(temp, tmp2); // [m03*m12*m21, m02*m11*m23, m01*m13*m22, --]
        let temp = _mm_sub_ps(v0, v1); // [m01*m12*m23-m03*m12*m21, m02*m13*m21-m02*m11*m23, m03*m11*m22-m01*m13*m22, --]
        let res3 = _mm_mul_ps(m30, temp);
        
        let tmp0 = _mm_sub_ps(res0, res1);
        let tmp1 = _mm_sub_ps(res2, res3);
        let temp = _mm_add_ps(tmp0, tmp1);

        let temp = _mm_and_ps(temp, _mm_load_ps(&BIT_MASK as *const u32 as *const f32));
        let tmp0 = _mm_shuffle_ps::<0b_00_00_00_00>(temp, temp); 
        let tmp1 = _mm_shuffle_ps::<0b_01_01_01_01>(temp, temp);
        let tmp2 = _mm_shuffle_ps::<0b_10_10_10_10>(temp, temp);
        let temp = _mm_add_ps(tmp0, tmp1);
        let temp = _mm_add_ps(temp, tmp2);

        _mm_cvtss_f32(temp)
    }
}