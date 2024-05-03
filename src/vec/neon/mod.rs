use core::arch::aarch64::*;
use crate::data::*;



/// This is the data type used in vector operations.
pub type Vector = float32x4_t;



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
    unsafe { vld1q_f32(&src as *const Float4 as *const f32) }
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
    unsafe { vst1q_f32(&mut dst as *mut Float4 as *mut f32, src) };
    dst
}



/// Adds two vectors.
#[inline]
pub fn vector_add(a: Vector, b: Vector) -> Vector {
    unsafe { vaddq_f32(a, b) }
}

/// Subtracts two vectors.
#[inline]
pub fn vector_sub(a: Vector, b: Vector) -> Vector {
    unsafe { vsubq_f32(a, b) }
}

/// Multiplies two vectors.
#[inline]
pub fn vector_mul(a: Vector, b: Vector) -> Vector {
    unsafe { vmulq_f32(a, b) }
}

/// Divides two vectors.
#[inline]
pub fn vector_div(a: Vector, b: Vector) -> Vector {
    unsafe { vdivq_f32(a, b) }
}

/// Absolute value on vector elements
#[inline]
pub fn vector_abs(v: Vector) -> Vector {
    unsafe { vabsq_f32(v) }
}

/// Negativizes the given vector.
#[inline]
pub fn vector_neg(v: Vector) -> Vector {
    unsafe { vnegq_f32(v) }
}

/// Takes the smaller of the elements of the two vectors.
#[inline]
pub fn vector_min(a: Vector, b: Vector) -> Vector {
    unsafe { vminq_f32(a, b) }
}

/// Takes the larger of the elements of the two vectors.
#[inline]
pub fn vector_max(a: Vector, b: Vector) -> Vector {
    unsafe { vmaxq_f32(a, b) }
}



/// Length sqared of a two-element vector.
#[inline(always)]
pub fn vector2_length_sq(v: Vector) -> f32 {
    vector2_dot(v, v)
}

/// Length sqared of a three-element vector.
#[inline(always)]
pub fn vector3_length_sq(v: Vector) -> f32 {
    vector3_dot(v, v)
}

/// Length sqared of a four-element vector.
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

/// Dot product of a two-element vector
#[inline]
pub fn vector2_dot(a: Vector, b: Vector) -> f32 {
    unsafe { 
        let mul = vmul_f32(vget_low_f32(a), vget_low_f32(b));
        let sum = vpadd_f32(mul, mul);
        vget_lane_f32::<0>(sum)
    }
}

/// Dot product of a three-element vector
#[inline]
pub fn vector3_dot(a: Vector, b: Vector) -> f32 {
    unsafe {
        let mul = vmulq_f32(a, b);
        let low = vget_low_f32(mul);
        let high = vget_high_f32(mul);
        let v1 = vpadd_f32(low, low);
        let v2 = vdup_lane_f32::<0>(high);
        let sum = vadd_f32(v1, v2);
        vget_lane_f32::<0>(sum)
    }
}

/// Dot product of a four-element vector
#[inline]
pub fn vector4_dot(a: Vector, b: Vector) -> f32 {
    unsafe {
        let mul = vmulq_f32(a, b);
        let low = vget_low_f32(mul);
        let high = vget_high_f32(mul);
        let v1 = vpadd_f32(low, low);
        let v2 = vpadd_f32(high, high);
        let sum = vadd_f32(v1, v2);
        vget_lane_f32::<0>(sum)
    }
}

/// Cross product of a three-element vector
#[inline]
pub fn vector3_cross(a: Vector, b: Vector) -> Vector {
    unsafe {
        const BIT_MASK: [u32; 4] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x0000000];

        let a_xy = vget_low_f32(a); // [ax, ay]
        let a_zz = vdup_lane_f32::<0>(vget_high_f32(a)); // [az, az]
        let a_zx = vext_f32::<0>(a_zz, a_xy); // [az, ax]
        let a_yz = vext_f32::<1>(a_xy, a_zz); // [ay, az]

        let b_xy = vget_low_f32(b); // [bx, by]
        let b_yx = vrev64_f32(b_xy); // [by, bx]
        let b_zz = vdup_lane_f32::<0>(vget_high_f32(b)); // [bz, bz]
        let b_yz = vext_f32::<0>(b_yx, b_zz); // [by, bz]
        let b_zx = vext_f32::<0>(b_zz, b_xy); // [bz, bx]

        let result = vmulq_f32(vcombine_f32(a_yz, a_xy), vcombine_f32(b_zx, b_yx)); // [ay*bz, az*bx, ax*by, _]
        let result = vmlsq_f32(result, vcombine_f32(a_zx, a_yz), vcombine_f32(b_yz, b_xy)); // [ay*bz-az*by, az*bx-ax*bz, ax*by-ay*bx, _]
        vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(result), vld1q_dup_u32(&BIT_MASK as *const u32)))
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
