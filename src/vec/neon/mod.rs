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

#[inline]
pub fn vector_abs(v: Vector) -> Vector {
    unsafe { vabsq_f32(v) }
}



#[inline(always)]
pub fn vector2_length_sq(v: Vector) -> f32 {
    vector2_dot(v, v)
}

#[inline(always)]
pub fn vector3_length_sq(v: Vector) -> f32 {
    vector3_dot(v, v)
}

#[inline(always)]
pub fn vector4_length_sq(v: Vector) -> f32 {
    vector4_dot(v, v)
}

#[inline]
pub fn vector2_dot(a: Vector, b: Vector) -> f32 {
    unsafe { 
        let mul = vmul_f32(vget_low_f32(a), vget_low_f32(b));
        let sum = vpadd_f32(mul, mul);
        vget_lane_f32::<0>(sum)
    }
}

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
