use core::arch::aarch64::*;
use crate::data::*;



/// This is the data type used in vector operations.
pub type Vector = float32x4_t;

/// This is the data type used in vector operations.
pub type VectorU32 = uint32x4_t;

/// This is the data type used in matrix operations.
pub type MATRIX = [float32x4_t; 4];


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
    unsafe { vld1q_f32(&src as *const Float4 as *const f32) }
}

/// Load matrix data from [`Float3x3`].
#[inline(always)]
pub fn load_float3x3(src: Float3x3) -> MATRIX {
    load_float4x4(Float4x4::from(src))
}

/// Load matrix data from [`Float4x4`].
#[inline(always)]
pub fn load_float4x4(src: Float4x4) -> MATRIX {
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
#[inline(always)]
pub fn load_uinteger4(src: UInteger4) -> VectorU32 {
    unsafe { vld1q_u32(&src as *const UInteger4 as *const u32) }
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
    unsafe { vst1q_f32(&mut dst as *mut Float4 as *mut f32, src) };
    dst
}

/// Store matrix data in [`Float3x3`].
#[inline(always)]
pub fn store_float3x3(src: MATRIX) -> Float3x3 {
    Float3x3::from(store_float4x4(src))
}

/// Store matrix data in [`Float4x4`].
#[inline(always)]
pub fn store_float4x4(src: MATRIX) -> Float4x4 {
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
    unsafe { vst1q_u32(&mut dst as *mut UInteger4 as *mut u32, src) };
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

/// Checks if the elements of two given vectors are less.
#[inline]
pub fn vector_lt(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vcltq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are less than or equal.
#[inline]
pub fn vector_le(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vcleq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are greater.
#[inline]
pub fn vector_gt(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vcgtq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are greater than or equal.
#[inline]
pub fn vector_ge(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vcgeq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are equal.
/// This function does not use [`f32::EPSILON`].
/// 
#[inline]
pub fn vector_eq(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vceqq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are not equal.
/// This function does not use [`f32::EPSILON`].
/// 
#[inline]
pub fn vector_ne(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vmvnq_u32(vceqq_f32(a, b))) }
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
        vget_lane_f32::<0x00>(sum)
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
        let v2 = vdup_lane_f32::<0x00>(high);
        let sum = vadd_f32(v1, v2);
        vget_lane_f32::<0x00>(sum)
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
        vget_lane_f32::<0x00>(sum)
    }
}

/// Cross product of a three-element vector
#[inline]
pub fn vector3_cross(a: Vector, b: Vector) -> Vector {
    const BIT_MASK: [u32; 4] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x0000000];
    unsafe {
        // a x b = [ay*bz-az*by, az*bx-ax*bz, ax*by-ay*bx]
        // v0 = { ay, az, ax, -- }
        // v1 = { bz, bx, by, -- }
        // v2 = { az, ax, ay, -- }
        // v3 = { by, bz, bx, -- }
        // v0*v1-v2*v3
        //

        let a_xy = vget_low_f32(a); // [ax, ay]
        let a_zz = vdup_lane_f32::<0x00>(vget_high_f32(a)); // [az, az]
        let a_zx = vext_f32::<0x01>(a_zz, a_xy); // [az, ax]
        let a_yz = vext_f32::<0x01>(a_xy, a_zz); // [ay, az]

        let b_xy = vget_low_f32(b); // [bx, by]
        let b_yx = vrev64_f32(b_xy); // [by, bx]
        let b_zz = vdup_lane_f32::<0x00>(vget_high_f32(b)); // [bz, bz]
        let b_yz = vext_f32::<0x01>(b_xy, b_zz); // [by, bz]
        let b_zx = vext_f32::<0x01>(b_zz, b_xy); // [bz, bx]

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

        let b_xy = vget_low_f32(b); // [bx, by]
        let b_zw = vget_high_f32(b); // [bz, bw]
        let b_yx = vrev64_f32(b_xy); // [by, bx]
        let b_wz = vrev64_f32(b_zw); // [bw, bz]
        
        let low = b_wz; // [bw, bz]
        let high = vext_f32::<0x01>(vneg_f32(b_xy), b_xy); // [-by, bx]
        let i_res = vmulq_f32(a, vcombine_f32(low, high)); // [ax*bw, ay*bz, -az*by, aw*bx]
        let tmp = vpaddq_f32(i_res, i_res);
        let i = vpaddq_f32(tmp, tmp);

        let low = vext_f32::<0x01>(vneg_f32(b_wz), b_wz); // [-bz, bw]
        let high = vext_f32::<0x01>(b_yx, b_yx); // [bx, by]
        let j_res = vmulq_f32(a, vcombine_f32(low, high)); // [-ax*bz, ay*bw, az*bx, aw*by]
        let tmp = vpaddq_f32(j_res, j_res);
        let j = vpaddq_f32(tmp, tmp);

        let low = vext_f32::<0x01>(b_xy, vneg_f32(b_xy)); // [by, -bx]
        let high = vext_f32::<0x01>(b_zw, b_zw); // [bw, bz]
        let k_res = vmulq_f32(a, vcombine_f32(low, high)); // [ax*by, -ay*bx, az*bw, aw*bz]
        let tmp = vpaddq_f32(k_res, k_res);
        let k = vpaddq_f32(tmp, tmp);

        let low = vneg_f32(b_xy); // [-bx, -by]
        let high = vext_f32::<0x01>(vneg_f32(b_wz), b_wz); // [-bz, bw]
        let w_res = vmulq_f32(a, vcombine_f32(low, high)); // [-ax*bx, -ay*by, -az*bz, aw*bw]
        let tmp = vpaddq_f32(w_res, w_res);
        let w = vpaddq_f32(tmp, tmp);

        let low = vext_f32::<0x01>(vget_low_f32(i), vget_low_f32(j));
        let high = vext_f32::<0x01>(vget_low_f32(k), vget_low_f32(w));

        vcombine_f32(low, high)
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
        let neg = vld1q_f32(&NEG as *const f32);
        vmulq_f32(neg, q)
    }
}

/// Returns the inverse of the quaternion. 
/// If normalization fails, `None` is returned.
#[inline]
pub fn quaternion_inverse(q: Vector) -> Option<Vector> {
    quaternion_normalize(q)
        .map(|q| quaternion_conjugate(q))
}
