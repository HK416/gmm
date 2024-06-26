use core::arch::aarch64::*;
use crate::data::*;



/// This is the data type used in vector operations.
pub type Vector = float32x4_t;

/// This is the data type used in vector operations.
pub type VectorU32 = uint32x4_t;

/// This is the data type used in matrix operations.
pub type Matrix = [float32x4_t; 4];


/// Load vector data from [`Boolean2`].
#[must_use]
#[inline(always)]
pub fn load_boolean2(src: Boolean2) -> VectorU32 {
    load_uinteger2(UInteger2::from(src))
}

/// Load vector data from [`Boolean3`].
#[must_use]
#[inline(always)]
pub fn load_boolean3(src: Boolean3) -> VectorU32 {
    load_uinteger3(UInteger3::from(src))
}

/// Load vector data from [`Boolean4`].
#[must_use]
#[inline(always)]
pub fn load_boolean4(src: Boolean4) -> VectorU32 {
    load_uinteger4(UInteger4::from(src))
}

/// Load vector data from [`Float2`].
#[must_use]
#[inline(always)]
pub fn load_float2(src: Float2) -> Vector {
    load_float4(Float4::from(src))
}

/// Load vector data from [`Float3`].
#[must_use]
#[inline(always)]
pub fn load_float3(src: Float3) -> Vector {
    load_float4(Float4::from(src))
}

/// Load vector data from [`Float4`].
#[inline]
#[must_use]
pub fn load_float4(src: Float4) -> Vector {
    unsafe { vld1q_f32(&src as *const Float4 as *const f32) }
}

/// Load matrix data from [`Float3x3`].
#[must_use]
#[inline(always)]
pub fn load_float3x3(src: Float3x3) -> Matrix {
    load_float4x4(Float4x4::from(src))
}

/// Load matrix data from [`Float4x4`].
#[must_use]
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
#[must_use]
#[inline(always)]
pub fn load_uinteger2(src: UInteger2) -> VectorU32 {
    load_uinteger4(UInteger4::from(src))
}

/// Load vector data from [`UInteger3`].
#[must_use]
#[inline(always)]
pub fn load_uinteger3(src: UInteger3) -> VectorU32 {
    load_uinteger4(UInteger4::from(src))
}

/// Load vector data from [`UInteger4`].
#[must_use]
#[inline(always)]
pub fn load_uinteger4(src: UInteger4) -> VectorU32 {
    unsafe { vld1q_u32(&src as *const UInteger4 as *const u32) }
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
#[inline]
#[must_use]
pub fn store_float4(src: Vector) -> Float4 {
    let mut dst = Float4::default();
    unsafe { vst1q_f32(&mut dst as *mut Float4 as *mut f32, src) };
    dst
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
    Float4x4 { 
        x_axis: store_float4(src[0]), 
        y_axis: store_float4(src[1]), 
        z_axis: store_float4(src[2]), 
        w_axis: store_float4(src[3]) 
    }
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
#[inline]
#[must_use]
pub fn store_uinteger4(src: VectorU32) -> UInteger4 {
    let mut dst = UInteger4::default();
    unsafe { vst1q_u32(&mut dst as *mut UInteger4 as *mut u32, src) };
    dst
}



/// Adds two vectors.
#[inline]
#[must_use]
pub fn vector_add(a: Vector, b: Vector) -> Vector {
    unsafe { vaddq_f32(a, b) }
}

/// Subracts two vectors.
#[inline]
#[must_use]
pub fn vector_sub(a: Vector, b: Vector) -> Vector {
    unsafe { vsubq_f32(a, b) }
}

/// Multiplies two vectors.
#[inline]
#[must_use]
pub fn vector_mul(a: Vector, b: Vector) -> Vector {
    unsafe { vmulq_f32(a, b) }
}

/// Divides two vectors.
#[inline]
#[must_use]
pub fn vector_div(a: Vector, b: Vector) -> Vector {
    unsafe { vdivq_f32(a, b) }
}

/// Absolute value on vector elements
#[inline]
#[must_use]
pub fn vector_abs(v: Vector) -> Vector {
    unsafe { vabsq_f32(v) }
}

/// Negativizes the given vector.
#[inline]
#[must_use]
pub fn vector_neg(v: Vector) -> Vector {
    unsafe { vnegq_f32(v) }
}

/// Takes the smaller of the elements of the two vectors.
#[inline]
#[must_use]
pub fn vector_min(a: Vector, b: Vector) -> Vector {
    unsafe { vminq_f32(a, b) }
}

/// Takes the larger of the elements of the two vectors.
#[inline]
#[must_use]
pub fn vector_max(a: Vector, b: Vector) -> Vector {
    unsafe { vmaxq_f32(a, b) }
}

/// Checks if the elements of two given vectors are less.
#[inline]
#[must_use]
pub fn vector_lt(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vcltq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are less than or equal.
#[inline]
#[must_use]
pub fn vector_le(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vcleq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are greater.
#[inline]
#[must_use]
pub fn vector_gt(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vcgtq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are greater than or equal.
#[inline]
#[must_use]
pub fn vector_ge(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vcgeq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are equal.
/// This function does not use [`f32::EPSILON`].
/// 
#[inline]
#[must_use]
pub fn vector_eq(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vceqq_f32(a, b)) }
}

/// Checks if the elements of two given vectors are not equal.
/// This function does not use [`f32::EPSILON`].
/// 
#[inline]
#[must_use]
pub fn vector_ne(a: Vector, b: Vector) -> Boolean4 {
    unsafe { store_boolean4(vmvnq_u32(vceqq_f32(a, b))) }
}

/// Returns a vector filled by adding all the elements of the vector.
#[inline]
#[must_use]
pub fn vector_sum(v: Vector) -> Vector {
    unsafe {
        let v = vpaddq_f32(v, v); // [x+y, z+w, x+y, z+w]
        vpaddq_f32(v, v) // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]
    }
}

/// Transformation of the given vector.
#[inline]
#[must_use]
pub fn vector_transform(m: Matrix, v: Vector) -> Vector {
    unsafe {
        let tm = matrix_transpose(m);
        let e0 = vector_mul(tm[0], v);
        let e0 = vpaddq_f32(e0, e0);
        let e0 = vpaddq_f32(e0, e0);

        let e1 = vector_mul(tm[1], v);
        let e1 = vpaddq_f32(e1, e1);
        let e1 = vpaddq_f32(e1, e1);

        let e2 = vector_mul(tm[2], v);
        let e2 = vpaddq_f32(e2, e2);
        let e2 = vpaddq_f32(e2, e2);

        let e3 = vector_mul(tm[3], v);
        let e3 = vpaddq_f32(e3, e3);
        let e3 = vpaddq_f32(e3, e3);

        let a = vextq_f32::<0b01>(e0, e1); // [e0, e0, e0, e1]
        let b = vextq_f32::<0b11>(e2, e3); // [e2, e3, e3, e3]
        vextq_f32::<0b10>(a, b) // [e0, e1, e2, e3]
    }
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
#[inline]
#[must_use]
pub fn vector2_dot(a: Vector, b: Vector) -> f32 {
    unsafe { 
        let mul = vmul_f32(vget_low_f32(a), vget_low_f32(b));
        let sum = vpadd_f32(mul, mul);
        vget_lane_f32::<0x00>(sum)
    }
}

/// Dot product of a three-element vector
#[inline]
#[must_use]
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
#[must_use]
pub fn vector4_dot(a: Vector, b: Vector) -> f32 {
    unsafe {
        let mul = vmulq_f32(a, b); // [ax*bx, ay*by, az*bz, aw*bw]
        let sum = vpaddq_f32(mul, mul); // [ax*bx+ay*by, az*bz+aw*bw, ...]
        let sum = vpaddq_f32(sum, sum); // [ax*bx+ay*by+az*bz+aw*bw, ...]
        vgetq_lane_f32::<0b00>(sum)
    }
}

/// Cross product of a three-element vector
#[inline]
#[must_use]
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
        let a_zw = vget_high_f32(a); // [az, aw]
        let a_yz = vext_f32::<0b01>(a_xy, a_zw); // [ay, az]
        let a_zx = vext_f32::<0b01>(a_yz, a_xy); // [az, ax]
        let v0 = vcombine_f32(a_yz, a_xy); // [ay, az, ax, ay]
        let v2 = vcombine_f32(a_zx, a_yz); // [az, ax, ay, az]

        let b_xy = vget_low_f32(b); // [bx, by]
        let b_zw = vget_high_f32(b); // [bz, bw]
        let b_yz = vext_f32::<0b01>(b_xy, b_zw); // [by, bz]
        let b_zx = vext_f32::<0b01>(b_yz, b_xy); // [bz, bx]
        let v1 = vcombine_f32(b_zx, b_yz); // [bz, bx, by, bz]
        let v3 = vcombine_f32(b_yz, b_xy); // [by, bz, bx, by]

        let t0 = vmulq_f32(v0, v1);
        let t1 = vmulq_f32(v2, v3);
        let res = vsubq_f32(t0, t1);
        let res = vreinterpretq_u32_f32(res);
        let mask = vld1q_u32(&BIT_MASK as *const u32);
        let res = vandq_u32(res, mask);
        vreinterpretq_f32_u32(res)
    }
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
#[inline]
#[must_use]
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
#[inline]
#[must_use]
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
#[must_use]
pub fn quaternion_inverse(q: Vector) -> Option<Vector> {
    quaternion_normalize(q)
        .map(|q| quaternion_conjugate(q))
}



/// Adds two matrices.
#[must_use]
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
#[must_use]
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
#[must_use]
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
#[must_use]
pub fn matrix_mul(a: Matrix, b: Matrix) -> Matrix {
    unsafe {
        let ta = matrix_transpose(a);

        let m00 = vector_mul(ta[0], b[0]); // [x, y, z, w]
        let m00 = vpaddq_f32(m00, m00); // [x+y, z+w, x+y, z+w]
        let m00 = vpaddq_f32(m00, m00); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]
        
        let m01 = vector_mul(ta[1], b[0]); // [x, y, z, w]
        let m01 = vpaddq_f32(m01, m01); // [x+y, z+w, x+y, z+w]
        let m01 = vpaddq_f32(m01, m01); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let m02 = vector_mul(ta[2], b[0]); // [x, y, z, w]
        let m02 = vpaddq_f32(m02, m02); // [x+y, z+w, x+y, z+w]
        let m02 = vpaddq_f32(m02, m02); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let m03 = vector_mul(ta[3], b[0]); // [x, y, z, w]
        let m03 = vpaddq_f32(m03, m03); // [x+y, z+w, x+y, z+w]
        let m03 = vpaddq_f32(m03, m03); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let low = vtrn1q_f32(m00, m01); // [m00, m01, m00, m01]
        let high = vtrn1q_f32(m02, m03); // [m02, m03, m02, m03]
        let v0 = vextq_f32::<0b10>(low, high); // [m00, m01, m02, m03]
        

        let m10 = vector_mul(ta[0], b[1]); // [x, y, z, w]
        let m10 = vpaddq_f32(m10, m10); // [x+y, z+w, x+y, z+w]
        let m10 = vpaddq_f32(m10, m10); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]
        
        let m11 = vector_mul(ta[1], b[1]); // [x, y, z, w]
        let m11 = vpaddq_f32(m11, m11); // [x+y, z+w, x+y, z+w]
        let m11 = vpaddq_f32(m11, m11); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let m12 = vector_mul(ta[2], b[1]); // [x, y, z, w]
        let m12 = vpaddq_f32(m12, m12); // [x+y, z+w, x+y, z+w]
        let m12 = vpaddq_f32(m12, m12); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let m13 = vector_mul(ta[3], b[1]); // [x, y, z, w]
        let m13 = vpaddq_f32(m13, m13); // [x+y, z+w, x+y, z+w]
        let m13 = vpaddq_f32(m13, m13); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let low = vtrn1q_f32(m10, m11); // [m10, m11, m10, m11]
        let high = vtrn1q_f32(m12, m13); // [m12, m13, m12, m13]
        let v1 = vextq_f32::<0b10>(low, high); // [m10, m11, m12, m13]


        let m20 = vector_mul(ta[0], b[2]); // [x, y, z, w]
        let m20 = vpaddq_f32(m20, m20); // [x+y, z+w, x+y, z+w]
        let m20 = vpaddq_f32(m20, m20); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]
        
        let m21 = vector_mul(ta[1], b[2]); // [x, y, z, w]
        let m21 = vpaddq_f32(m21, m21); // [x+y, z+w, x+y, z+w]
        let m21 = vpaddq_f32(m21, m21); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let m22 = vector_mul(ta[2], b[2]); // [x, y, z, w]
        let m22 = vpaddq_f32(m22, m22); // [x+y, z+w, x+y, z+w]
        let m22 = vpaddq_f32(m22, m22); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let m23 = vector_mul(ta[3], b[2]); // [x, y, z, w]
        let m23 = vpaddq_f32(m23, m23); // [x+y, z+w, x+y, z+w]
        let m23 = vpaddq_f32(m23, m23); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let low = vtrn1q_f32(m20, m21); // [m20, m21, m20, m21]
        let high = vtrn1q_f32(m22, m23); // [m22, m23, m22, m23]
        let v2 = vextq_f32::<0b10>(low, high); // [m20, m21, m22, m23]


        let m30 = vector_mul(ta[0], b[3]); // [x, y, z, w]
        let m30 = vpaddq_f32(m30, m30); // [x+y, z+w, x+y, z+w]
        let m30 = vpaddq_f32(m30, m30); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]
        
        let m31 = vector_mul(ta[1], b[3]); // [x, y, z, w]
        let m31 = vpaddq_f32(m31, m31); // [x+y, z+w, x+y, z+w]
        let m31 = vpaddq_f32(m31, m31); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let m32 = vector_mul(ta[2], b[3]); // [x, y, z, w]
        let m32 = vpaddq_f32(m32, m32); // [x+y, z+w, x+y, z+w]
        let m32 = vpaddq_f32(m32, m32); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let m33 = vector_mul(ta[3], b[3]); // [x, y, z, w]
        let m33 = vpaddq_f32(m33, m33); // [x+y, z+w, x+y, z+w]
        let m33 = vpaddq_f32(m33, m33); // [x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w]

        let low = vtrn1q_f32(m30, m31); // [m30, m31, m30, m31]
        let high = vtrn1q_f32(m32, m33); // [m32, m33, m32, m33]
        let v3 = vextq_f32::<0b10>(low, high); // [m30, m31, m32, m33]

        [v0, v1, v2, v3]
    }
}

/// Transpose of a matrix.
#[inline]
#[must_use]
pub fn matrix_transpose(m: Matrix) -> Matrix {
    // matrix m
    // A, B, C, D
    // E, F, G, H
    // I, J, K, L
    // M, N, O, P
    //
    unsafe {
        // matrix m0, m1
        // v01.0 [A, E, C, G]
        // v01.1 [B, F, D, H]
        // v23.0 [I, M, K, O]
        // v23.1 [J, N, L, P]
        //
        let v01 = vtrnq_f32(m[0], m[1]); 
        let v23 = vtrnq_f32(m[2], m[3]);
        
        let low = vget_low_f32(v01.0); // [A, E]
        let high = vget_low_f32(v23.0); // [I, M]
        let v0 = vcombine_f32(low, high); // [A, E, I, M]

        let low = vget_low_f32(v01.1); // [B, F]
        let high = vget_low_f32(v23.1); // [J, N]
        let v1 = vcombine_f32(low, high); // [B, F, J, N]

        let low = vget_high_f32(v01.0); // [C, G]
        let high = vget_high_f32(v23.0); // [K, O]
        let v2 = vcombine_f32(low, high); // [C, G, K, O]

        let low = vget_high_f32(v01.1); // [D, H]
        let high = vget_high_f32(v23.1); // [L, P]
        let v3 = vcombine_f32(low, high); // [G, H, L, P]

        [v0, v1, v2, v3]
    }
}

/// determinant of a matrix.
#[must_use]
pub fn matrix_determinant(m: Matrix) -> f32 {
    // det = m00 * (m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m13*m22*m31 - m12*m21*m33 - m11*m23*m32)
    //      - m10 * (m01*m22*m33 + m02*m23*m31 + m03*m21*m32 - m03*m22*m31 - m02*m21*m33 - m01*m23*m32)
    //      + m20 * (m01*m12*m33 + m02*m13*m31 + m03*m11*m32 - m03*m12*m31 - m02*m11*m33 - m01*m13*m32)
    //      - m30 * (m01*m12*m23 + m02*m13*m21 + m03*m11*m22 - m03*m12*m21 - m02*m11*m23 - m01*m13*m22)
    //
    const BIT_MASK: [f32; 4] = [1.0, 1.0, 1.0, 0.0];
    unsafe {
        let mask = vld1q_f32(&BIT_MASK as *const f32);

        let m0_xy = vget_low_f32(m[0]); // [m00, m01]
        let m0_zw = vget_high_f32(m[0]); // [m02, m03]
        let m0_yx = vext_f32::<0b01>(m0_xy, m0_xy); // [m01, m00]
        let m0_wz = vext_f32::<0b01>(m0_zw, m0_zw); // [m03, m02]
        let m0_yz = vext_f32::<0b01>(m0_xy, m0_zw); // [m01, m02]
        let m0_wx = vext_f32::<0b01>(m0_zw, m0_xy); // [m03, m00]
        let m0_yzwx = vcombine_f32(m0_yz, m0_wx); // [m01, m02, m03, m00]
        let m0_wzyx = vcombine_f32(m0_wz, m0_yx); // [m03, m02, m01, m00]

        let m1_xy = vget_low_f32(m[1]); // [m10, m11]
        let m1_zw = vget_high_f32(m[1]); // [m12, m13]
        let m1_yx = vext_f32::<0b01>(m1_xy, m1_xy); // [m11, m10]
        let m1_wz = vext_f32::<0b01>(m1_zw, m1_zw); // [m13, m12]
        let m1_zy = vext_f32::<0b01>(m1_wz, m1_yx); // [m12, m11]
        let m1_wx = vext_f32::<0b01>(m1_zw, m1_xy); // [m13, m10]
        let m1_yzwx = vextq_f32::<0b01>(m[1], m[1]); // [m11, m12, m13, m10]
        let m1_wzyx = vcombine_f32(m1_wz, m1_yx); // [m13, m12, m11, m10]
        let m1_zwyx = vcombine_f32(m1_zw, m1_yx); // [m12, m13, m11, m10]
        let m1_zywx = vcombine_f32(m1_zy, m1_wx);

        let m2_xy = vget_low_f32(m[2]); // [m20, m21]
        let m2_zw = vget_high_f32(m[2]); // [m22, m23]
        let m2_yx = vext_f32::<0b01>(m2_xy, m2_xy); // [m21, m20]
        let m2_yz = vext_f32::<0b01>(m2_xy, m2_zw); // [m21, m22]
        let m2_zy = vext_f32::<0b01>(m2_yz, m2_yz); // [m22, m21]
        let m2_wx = vext_f32::<0b01>(m2_zw, m2_xy); // [m23, m20]
        let m2_wy = vext_f32::<0b01>(m2_zw, m2_yx); // [m23, m21]
        let m2_zx = vext_f32::<0b01>(m2_yz, m2_xy); // [m22, m20]
        let m2_yw = vext_f32::<0b01>(m2_wy, m2_wy); // [m21, m23]
        let m2_ywzx = vcombine_f32(m2_yw, m2_zx); // [m21, m23, m22, m20]
        let m2_zwyx = vcombine_f32(m2_zw, m2_yx); // [m22, m23, m21, m20]
        let m2_zywx = vcombine_f32(m2_zy, m2_wx); // [m22, m21, m23, m20]
        let m2_wyzx = vcombine_f32(m2_wy, m2_zx); // [m23, m21, m22, m20]
        
        let m3_xy = vget_low_f32(m[3]); // [m30, m31]
        let m3_zw = vget_high_f32(m[3]); // [m32, m33]
        let m3_yx = vext_f32::<0b01>(m3_xy, m3_xy); // [m31, m30]
        let m3_xz = vext_f32::<0b01>(m3_yx, m3_zw); // [m30, m32]
        let m3_zx = vext_f32::<0b01>(m3_xz, m3_xz); // [m32, m30]
        let m3_wy = vext_f32::<0b01>(m3_zw, m3_yx); // [m33, m31]
        let m3_yw = vext_f32::<0b01>(m3_wy, m3_wy); // [m31, m33]
        let m3_wyzx = vcombine_f32(m3_wy, m3_zx); // [m33, m31, m32, m30]
        let m3_ywzx = vcombine_f32(m3_yw, m3_zx); // [m31, m33, m32, m30]

        //---------------------------------------------------------------------
        // m11*m22*m33 + m12*m23*m31 + m13*m21*m32
        let lhs = vmulq_f32(m1_yzwx, m2_zwyx);
        let lhs = vmulq_f32(lhs, m3_wyzx);

        // m13*m22*m31 + m12*m21*m33 + m11*m23*m32
        let rhs = vmulq_f32(m1_wzyx, m2_zywx);
        let rhs = vmulq_f32(rhs, m3_ywzx);

        // (m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m13*m22*m31 - m12*m21*m33 - m11*m23*m32)
        let v0 = vsubq_f32(lhs, rhs);
        let v0 = vmulq_f32(v0, mask);

        // m00 * (m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m13*m22*m31 - m12*m21*m33 - m11*m23*m32)
        let v0 = vmulq_f32(vdupq_laneq_f32::<0b00>(m[0]), v0);

        
        //---------------------------------------------------------------------
        // m01*m22*m33 + m02*m23*m31 + m03*m21*m32
        let lhs = vmulq_f32(m0_yzwx, m2_zwyx);
        let lhs = vmulq_f32(lhs, m3_wyzx);

        // m03*m22*m31 + m02*m21*m33 + m01*m23*m32
        let rhs = vmulq_f32(m0_wzyx, m2_zywx);
        let rhs = vmulq_f32(rhs, m3_ywzx);

        // m01*m22*m33 + m02*m23*m31 + m03*m21*m32 - m03*m22*m31 - m02*m21*m33 - m01*m23*m32
        let v1 = vsubq_f32(lhs, rhs);
        let v1 = vmulq_f32(v1, mask);

        // m10 * (m01*m22*m33 + m02*m23*m31 + m03*m21*m32 - m03*m22*m31 - m02*m21*m33 - m01*m23*m32)
        let v1 = vmulq_f32(vdupq_laneq_f32::<0b00>(m[1]), v1);

        
        //---------------------------------------------------------------------
        // m01*m12*m33 + m02*m13*m31 + m03*m11*m32
        let lhs = vmulq_f32(m0_yzwx, m1_zwyx);
        let lhs = vmulq_f32(lhs, m3_wyzx);

        // m03*m12*m31 + m02*m11*m33 + m01*m13*m32
        let rhs = vmulq_f32(m0_wzyx, m1_zywx);
        let rhs = vmulq_f32(rhs, m3_ywzx);

        // m01*m12*m33 + m02*m13*m31 + m03*m11*m32 - m03*m12*m31 - m02*m11*m33 - m01*m13*m32
        let v2 = vsubq_f32(lhs, rhs);
        let v2 = vmulq_f32(v2, mask);

        // m20 * (m01*m12*m33 + m02*m13*m31 + m03*m11*m32 - m03*m12*m31 - m02*m11*m33 - m01*m13*m32)
        let v2 = vmulq_f32(vdupq_laneq_f32::<0b00>(m[2]), v2);


        //---------------------------------------------------------------------
        // m01*m12*m23 + m02*m13*m21 + m03*m11*m22
        let lhs = vmulq_f32(m0_yzwx, m1_zwyx);
        let lhs = vmulq_f32(lhs, m2_wyzx);

        // m03*m12*m21 + m02*m11*m23 + m01*m13*m22
        let rhs = vmulq_f32(m0_wzyx, m1_zywx);
        let rhs = vmulq_f32(rhs, m2_ywzx);

        // m01*m12*m23 + m02*m13*m21 + m03*m11*m22 - m03*m12*m21 - m02*m11*m23 - m01*m13*m22
        let v3 = vsubq_f32(lhs, rhs);
        let v3 = vmulq_f32(v3, mask);

        // m30 * (m01*m12*m23 + m02*m13*m21 + m03*m11*m22 - m03*m12*m21 - m02*m11*m23 - m01*m13*m22)
        let v3 = vmulq_f32(vdupq_laneq_f32::<0b00>(m[3]), v3);


        // result
        let res = vsubq_f32(v0, v1);
        let res = vaddq_f32(res, v2);
        let res = vsubq_f32(res, v3);
        let res = vector_sum(res);
        vgetq_lane_f32::<0b00>(res)
    }
}

/// inverse of a matrix.
/// 
/// If the inverse of a matrix cannot be calculated, returns `None`.
/// 
#[must_use]
pub fn matrix_inverse(m: Matrix) -> Option<Matrix> {
    const ONE_NEG: [f32; 2] = [1.0, -1.0];
    const NEG_ONE: [f32; 2] = [-1.0, 1.0];
    const ONE_NEG_ONE_NEG: [f32; 4] = [1.0, -1.0, 1.0, -1.0];
    const NEG_ONE_NEG_ONE: [f32; 4] = [-1.0, 1.0, -1.0, 1.0];

    unsafe {
        let m00_m01 = vget_low_f32(m[0]);
        let m01_m00 = vext_f32::<0b01>(m00_m01, m00_m01);
        let m00_m00 = vext_f32::<0b01>(m01_m00, m00_m01);
        let m02_m03 = vget_high_f32(m[0]);
        let m01_m02 = vext_f32::<0b01>(m00_m01, m02_m03);
        let m03_m02 = vext_f32::<0b01>(m02_m03, m02_m03);
        let m03_m03 = vext_f32::<0b01>(m02_m03, m03_m02);

        let m10_m11 = vget_low_f32(m[1]);
        let m11_m10 = vext_f32::<0b01>(m10_m11, m10_m11);
        let m10_m10 = vext_f32::<0b01>(m11_m10, m10_m11);
        let m12_m13 = vget_high_f32(m[1]);
        let m11_m12 = vext_f32::<0b01>(m10_m11, m12_m13);
        let m13_m12 = vext_f32::<0b01>(m12_m13, m12_m13);
        let m13_m13 = vext_f32::<0b01>(m12_m13, m13_m12);

        let m20_m21 = vget_low_f32(m[2]);
        let m21_m20 = vext_f32::<0b01>(m20_m21, m20_m21);
        let m20_m20 = vext_f32::<0b01>(m21_m20, m20_m21);
        let m22_m23 = vget_high_f32(m[2]);
        let m23_m22 = vext_f32::<0b01>(m22_m23, m22_m23);
        let m22_m21 = vext_f32::<0b01>(m23_m22, m21_m20);
        let m23_m23 = vext_f32::<0b01>(m22_m23, m23_m22);

        let m30_m31 = vget_low_f32(m[3]);
        let m31_m30 = vext_f32::<0b01>(m30_m31, m30_m31);
        let m30_m30 = vext_f32::<0b01>(m31_m30, m30_m31);
        let m32_m33 = vget_high_f32(m[3]);
        let m33_m32 = vext_f32::<0b01>(m32_m33, m32_m33);
        let m32_m31 = vext_f32::<0b01>(m33_m32, m31_m30);
        let m33_m33 = vext_f32::<0b01>(m32_m33, m33_m32);

        let m00_m00_m00_m01 = vcombine_f32(m00_m00, m00_m01);
        let m11_m12_m13_m12 = vcombine_f32(m11_m12, m13_m12);
        let m10_m10_m10_m11 = vcombine_f32(m10_m10, m10_m11);
        let m01_m02_m03_m02 = vcombine_f32(m01_m02, m03_m02);
        let a = vmulq_f32(m00_m00_m00_m01, m11_m12_m13_m12);
        let b = vmulq_f32(m10_m10_m10_m11, m01_m02_m03_m02);
        let t0 = vsubq_f32(a, b);

        let m01_m02_m22_m21 = vcombine_f32(m01_m02, m22_m21);
        let m13_m13_m33_m33 = vcombine_f32(m13_m13, m33_m33);
        let m11_m12_m32_m31 = vcombine_f32(m11_m12, m32_m31);
        let m03_m03_m23_m23 = vcombine_f32(m03_m03, m23_m23);
        let a = vmulq_f32(m01_m02_m22_m21, m13_m13_m33_m33);
        let b = vmulq_f32(m11_m12_m32_m31, m03_m03_m23_m23);
        let t1 = vsubq_f32(a, b);

        let m21_m20_m20_m20 = vcombine_f32(m21_m20, m20_m20);
        let m32_m33_m32_m31 = vcombine_f32(m32_m33, m32_m31);
        let m31_m30_m30_m30 = vcombine_f32(m31_m30, m30_m30);
        let m22_m23_m22_m21 = vcombine_f32(m22_m23, m22_m21);
        let a = vmulq_f32(m21_m20_m20_m20, m32_m33_m32_m31);
        let b = vmulq_f32(m31_m30_m30_m30, m22_m23_m22_m21);
        let t2 = vsubq_f32(a, b);

        let mask = vld1_f32(&ONE_NEG as *const f32);
        let t00_t01 = vget_low_f32(t0);
        let t12_t13 = vget_high_f32(t1);
        let r0 = vmul_f32(t00_t01, t12_t13);
        let r0 = vmul_f32(r0, mask);
        
        let t02_t03 = vget_high_f32(t0);
        let t20_t21 = vget_low_f32(t2);
        let r1 = vmul_f32(t02_t03, t20_t21);

        let mask = vld1_f32(&NEG_ONE as *const f32);
        let t10_t11 = vget_low_f32(t1);
        let t22_t23 = vget_high_f32(t2);
        let r2 = vmul_f32(t10_t11, t22_t23);
        let r2 = vmul_f32(r2, mask);

        let det = vadd_f32(r0, r1);
        let det = vadd_f32(det, r2);
        let det = vpadd_f32(det, det);
        let det = vget_lane_f32::<0b00>(det);

        if det.abs() <= f32::EPSILON {
            return None;
        }

        let recip_det = det.recip();

        let m10_m00 = vext_f32::<0b01>(m11_m10, m00_m01);
        let m11_m01 = vext_f32::<0b01>(m10_m11, m01_m00);
        let m12_m02 = vext_f32::<0b01>(m13_m12, m02_m03);
        let m13_m03 = vext_f32::<0b01>(m12_m13, m03_m02);
        let m30_m20 = vext_f32::<0b01>(m31_m30, m20_m21);
        let m31_m21 = vext_f32::<0b01>(m30_m31, m21_m20);
        let m32_m22 = vext_f32::<0b01>(m33_m32, m22_m23);
        let m33_m23 = vext_f32::<0b01>(m32_m33, m23_m22);

        let t01_t00 = vext_f32::<0b01>(t00_t01, t00_t01);
        let t03_t02 = vext_f32::<0b01>(t02_t03, t02_t03);
        let t11_t10 = vext_f32::<0b01>(t10_t11, t10_t11);
        let t13_t12 = vext_f32::<0b01>(t12_t13, t12_t13);
        let t21_t20 = vext_f32::<0b01>(t20_t21, t20_t21);
        let t23_t22 = vext_f32::<0b01>(t22_t23, t22_t23);
        
        let t00_t00 = vext_f32::<0b01>(t01_t00, t00_t01);
        let t01_t01 = vext_f32::<0b01>(t00_t01, t01_t00);
        let t02_t02 = vext_f32::<0b01>(t03_t02, t02_t03);
        let t03_t03 = vext_f32::<0b01>(t02_t03, t03_t02);
        let t10_t10 = vext_f32::<0b01>(t11_t10, t10_t11);
        let t11_t11 = vext_f32::<0b01>(t10_t11, t11_t10);
        let t12_t12 = vext_f32::<0b01>(t13_t12, t12_t13);
        let t13_t13 = vext_f32::<0b01>(t12_t13, t13_t12);
        let t20_t20 = vext_f32::<0b01>(t21_t20, t20_t21);
        let t21_t21 = vext_f32::<0b01>(t20_t21, t21_t20);
        let t22_t22 = vext_f32::<0b01>(t23_t22, t22_t23);
        let t23_t23 = vext_f32::<0b01>(t22_t23, t23_t22);

        let one_neg_one_neg = vld1q_f32(&ONE_NEG_ONE_NEG as *const f32);
        let neg_one_neg_one = vld1q_f32(&NEG_ONE_NEG_ONE as *const f32);

        let m10_m00_m30_m20 = vcombine_f32(m10_m00, m30_m20);
        let m11_m01_m31_m21 = vcombine_f32(m11_m01, m31_m21);
        let m12_m02_m32_m22 = vcombine_f32(m12_m02, m32_m22);
        let m13_m03_m33_m23 = vcombine_f32(m13_m03, m33_m23);

        let t12_t12_t11_t11 = vcombine_f32(t12_t12, t11_t11);
        let t13_t13_t10_t10 = vcombine_f32(t13_t13, t10_t10);
        let t20_t20_t03_t03 = vcombine_f32(t20_t20, t03_t03);
        let t21_t21_t02_t02 = vcombine_f32(t21_t21, t02_t02);
        let t22_t22_t01_t01 = vcombine_f32(t22_t22, t01_t01);
        let t23_t23_t00_t00 = vcombine_f32(t23_t23, t00_t00);

        let r0 = vmulq_f32(m11_m01_m31_m21, t12_t12_t11_t11);
        let r0 = vmulq_f32(r0, one_neg_one_neg);

        let r1 = vmulq_f32(m12_m02_m32_m22, t13_t13_t10_t10);
        let r1 = vmulq_f32(r1, neg_one_neg_one);

        let r2 = vmulq_f32(m13_m03_m33_m23, t20_t20_t03_t03);
        let r2 = vmulq_f32(r2, one_neg_one_neg);

        let col0 = vaddq_f32(r0, r1);
        let col0 = vaddq_f32(col0, r2);
        let col0 = vmulq_n_f32(col0, recip_det);


        let r0 = vmulq_f32(m10_m00_m30_m20, t12_t12_t11_t11);
        let r0 = vmulq_f32(r0, neg_one_neg_one);

        let r1 = vmulq_f32(m12_m02_m32_m22, t21_t21_t02_t02);
        let r1 = vmulq_f32(r1, one_neg_one_neg);

        let r2 = vmulq_f32(m13_m03_m33_m23, t22_t22_t01_t01);
        let r2 = vmulq_f32(r2, neg_one_neg_one);

        let col1 = vaddq_f32(r0, r1);
        let col1 = vaddq_f32(col1, r2);
        let col1 = vmulq_n_f32(col1, recip_det);

        
        let r0 = vmulq_f32(m10_m00_m30_m20, t13_t13_t10_t10);
        let r0 = vmulq_f32(r0, one_neg_one_neg);

        let r1 = vmulq_f32(m11_m01_m31_m21, t21_t21_t02_t02);
        let r1 = vmulq_f32(r1, neg_one_neg_one);

        let r2 = vmulq_f32(m13_m03_m33_m23, t23_t23_t00_t00);
        let r2 = vmulq_f32(r2, one_neg_one_neg);

        let col2 = vaddq_f32(r0, r1);
        let col2 = vaddq_f32(col2, r2);
        let col2 = vmulq_n_f32(col2, recip_det);


        let r0 = vmulq_f32(m10_m00_m30_m20, t20_t20_t03_t03);
        let r0 = vmulq_f32(r0, neg_one_neg_one);

        let r1 = vmulq_f32(m11_m01_m31_m21, t22_t22_t01_t01);
        let r1 = vmulq_f32(r1, one_neg_one_neg);

        let r2 = vmulq_f32(m12_m02_m32_m22, t23_t23_t00_t00);
        let r2 = vmulq_f32(r2, neg_one_neg_one);

        let col3 = vaddq_f32(r0, r1);
        let col3 = vaddq_f32(col3, r2);
        let col3 = vmulq_n_f32(col3, recip_det);

        Some([col0, col1, col2, col3])
    }
}
