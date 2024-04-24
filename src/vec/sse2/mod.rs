#[cfg(target_pointer_width = "32")]
use core::arch::x86::*;

#[cfg(target_pointer_width = "64")]
use core::arch::x86_64::*;

use crate::data::*;


/// This is the data type used in vector operations.
pub type Vector = __m128;


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
