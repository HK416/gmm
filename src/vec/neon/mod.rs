use core::arch::aarch64::*;
use crate::data::*;



/// This is the data type used in vector operations.
pub type Vector = float32x4_t;

/// This is the data type used in vector operations.
pub type VectorU32 = uint32x4_t;

/// This is the data type used in matrix operations.
pub type Matrix = [float32x4_t; 4];


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
    unsafe { vst1q_u32(&mut dst as *mut UInteger4 as *mut u32, src) };
    dst
}



/// Adds two vectors.
#[inline]
pub fn vector_add(a: Vector, b: Vector) -> Vector {
    unsafe { vaddq_f32(a, b) }
}

/// Subracts two vectors.
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
        let mul = vmulq_f32(a, b); // [ax*bx, ay*by, az*bz, aw*bw]
        let sum = vpaddq_f32(mul, mul); // [ax*bx+ay*by, az*bz+aw*bw, ...]
        let sum = vpaddq_f32(sum, sum); // [ax*bx+ay*by+az*bz+aw*bw, ...]
        println!("sum={:?}", sum);
        vgetq_lane_f32::<0b00>(sum)
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
pub fn matrix_determinant(m: Matrix) -> f32 {
    // det = m00 * (m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m13*m22*m31 - m12*m21*m33 - m11*m23*m32)
    //      - m10 * (m01*m22*m33 + m02*m23*m31 + m03*m21*m32 - m03*m22*m31 - m02*m21*m33 - m01*m23*m32)
    //      + m20 * (m01*m12*m33 + m02*m13*m33 - m01*m13*m32 - m03*m12*m31 - m02*m11*m33 - m01*m13*m32)
    //      - m30 * (m01*m12*m23 + m02*m13*m21 + m03*m11*m22 - m03*m12*m21 - m02*m11*m23 - m01*m13*m22)
    //
    const BIT_MASK: [u32; 4] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000];
    unsafe {
        
        let m1_yzwx = vextq_f32::<0b01>(m[1], m[1]); // [m11, m12, m13, m10]
        let m2_yzwx = vextq_f32::<0b01>(m[2], m[2]); // [m21, m22, m23, m20]
        let m2_zwyz = vextq_f32::<0b10>(m[2], m2_yzwx); // [m22, m23, m21, m22]
        let m3_yzwx = vextq_f32::<0b01>(m[3], m[3]); // [m31, m32, m33, m30]
        let m3_wyzw = vextq_f32::<0b11>(m[3], m3_yzwx); // [m33, m31, m32, m33]
        let temp = vmulq_f32(m1_yzwx, m2_zwyz); // [m11*m22, m12*m23, m13*m21, --]
        let v0 = vmulq_f32(temp, m3_wyzw); // [m11*m22*m33, m12*m23*m31, m13*m21*m32, --]

        let m1_yxwz = vrev64q_f32(m[1]); // [m11, m10, m13, m12]
        let m1_wzyx = vcombine_f32(vget_high_f32(m1_yxwz), vget_low_f32(m1_yxwz)); // [m13, m12, m11, m10]
        let m2_wzzy = vrev64q_f32(m2_zwyz); // [m23, m22, m22, m21]
        let m2_zywz = vcombine_f32(vget_high_f32(m2_wzzy), vget_low_f32(m2_wzzy)); // [m22, m31, m23, m22]
        let m3_wy = vget_low_f32(m3_wyzw); // [m33, m31]
        let m3_yw = vext_f32::<0b01>(m3_wy, m3_wy); // [m31, m33]
        let m3_zw = vget_high_f32(m3_wyzw); // [m32, m33]
        let m3_ywzw = vcombine_f32(m3_yw, m3_zw); // [m31, m33, m32, m33]
        let temp = vmulq_f32(m1_wzyx, m2_zywz); // [m13*m22, m12*m21, m11*m23, --]
        let v1 = vmulq_f32(temp, m3_ywzw); // [m13*m22*m31, m12*m21*m33, m11*m23*m32, --]

        let m0_xxxx = vdupq_laneq_f32::<0b00>(m[0]); // [m00, ...]        
        let temp = vsubq_f32(v0, v1); // [m11*m22*m33-m13*m22*m31, m12*m23*m31-m12*m21*m33, m13*m21*m32-m11*m23*m32, --]
        let res0 = vmulq_f32(m0_xxxx, temp); 


        let m0_yzwx = vextq_f32::<0b01>(m[0], m[0]); // [m01, m02, m03, m00]
        let temp = vmulq_f32(m0_yzwx, m2_zwyz); // [m01*m22, m02*m23, m03*m21, --]
        let v0 = vmulq_f32(temp, m3_wyzw); // [m01*m22*m33, m02*m23*m31, m03*m21*m32, --]

        let m0_yxwz = vrev64q_f32(m[0]); // [m01, m00, m03, m02]
        let m0_wzyx = vcombine_f32(vget_high_f32(m0_yxwz), vget_low_f32(m0_yxwz)); // [m03, m02, m01, m00]
        let temp = vmulq_f32(m0_wzyx, m2_zywz); // [m03*m22, m02*m21, m01*m23, --]
        let v1 = vmulq_f32(temp, m3_ywzw); // [m03*m22*m31, m02*m21*m33, m01*m23*m32, --]

        let m1_xxxx = vdupq_laneq_f32::<0b00>(m[1]);
        let temp = vsubq_f32(v0, v1); // [m01*m22*m33-m03*m22*m31, m02*m23*m31-m02*m21*m33, m03*m21*m32-m01*m23*m32, --]
        let res1 = vmulq_f32(m1_xxxx, temp);


        let m1_wz = vget_low_f32(m1_wzyx); // [m13, m12]
        let m1_zw = vext_f32::<0b01>(m1_wz, m1_wz); // [m12, m13]
        let m1_yx = vget_high_f32(m1_wzyx); // [m11, m10]
        let m1_zwyx = vcombine_f32(m1_zw, m1_yx); // [m12, m13, m11, m10]
        let temp = vmulq_f32(m0_yzwx, m1_zwyx); // [m01*m12, m02*m13, m03*m11, --]
        let v0 = vmulq_f32(temp, m3_wyzw); // [m01*m12*m33, m02*m13*m31, m03*m11*m32, --]

        let m1_zy = vext_f32::<0b01>(m1_wz, m1_yx); // [m13, m11]
        let m1_zywz = vcombine_f32(m1_zy, m1_wz); // [m12, m11, m13, m12]
        let temp = vmulq_f32(m0_wzyx, m1_zywz); // [m03*m12, m02*m11, m01*m13, --]
        let v1 = vmulq_f32(temp, m3_ywzw); // [m03*m12*m31, m02*m11*m33, m01*m13*m32, --]

        let m2_xxxx = vdupq_laneq_f32::<0b00>(m[2]); // [m20, m20, m20, m20]
        let temp = vsubq_f32(v0, v1); // [m01*m12*m33-m03*m12*m31, m02*m13*m31-m02*m11*m33, m03*m11*m32-m01*m13*m32, --]
        let res2 = vmulq_f32(m2_xxxx, temp);


        let m2_wyzx = vextq_f32::<0b01>(m2_zwyz, m2_xxxx); // [m23, m21, m22, m20]
        let temp = vmulq_f32(m0_yzwx, m1_zwyx); // [m01*m12, m02*m13, m03*m11, --]
        let v0 = vmulq_f32(temp, m2_wyzx); // [m01*m12*m23, m02*m13*m21, m03*m11*m22, --]

        let m1_yz = vget_low_f32(m1_yzwx); // [m11, m12]
        let m1_zy = vext_f32::<0b01>(m1_yz, m1_yz); // [m12, m11]
        let m1_wx = vget_high_f32(m1_yzwx); // [m13, m10]
        let m1_zywx = vcombine_f32(m1_zy, m1_wx); // [m12, m11, m13, m10]
        let m2_ywzz = vextq_f32::<0b01>(m2_zywz, m2_zywz);
        let temp = vmulq_f32(m0_wzyx, m1_zywx); // [m03*m12, m02*m11, m01*m13, --]
        let v1 = vmulq_f32(temp, m2_ywzz); // [m03*m12*m21, m02*m11*m23, m01*m13*m22, --]

        let m3_xxxx = vdupq_laneq_f32::<0b00>(m[3]); // [m30, m30, m30, m30]
        let temp = vsubq_f32(v0, v1); // [m01*m12*m23-m03*m12*m21, m02*m13*m21-m02*m11*m23, m03*m11*m22-m01*m13*m22, --]
        let res3 = vmulq_f32(m3_xxxx, temp);

        let v0 = vsubq_f32(res0, res1);
        let v1 = vsubq_f32(res2, res3);
        let temp = vaddq_f32(v0, v1);

        let temp = vreinterpretq_u32_f32(temp);
        let mask = vld1q_u32(&BIT_MASK as *const u32);
        let temp = vandq_u32(temp, mask);
        let temp = vreinterpretq_f32_u32(temp);

        let res = vpaddq_f32(temp, temp);
        let res = vpaddq_f32(res, res);
        vgetq_lane_f32::<0b00>(res)
    }
}

/// inverse of a matrix.
/// 
/// If the inverse of a matrix cannot be calculated, returns `None`.
/// 
pub fn matrix_inverse(m: Matrix) -> Option<Matrix> {
    // a00 = m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m11*m23*m32 - m12*m21*m33 - m13*m22*m31
    // a01 = m01*m23*m32 + m02*m21*m33 + m03*m22*m31 - m01*m22*m33 - m02*m23*m31 - m03*m21*m32
    // a02 = m01*m12*m33 + m02*m13*m31 + m03*m11*m32 - m01*m13*m32 - m02*m11*m33 - m03*m12*m31
    // a03 = m01*m13*m22 + m02*m11*m23 + m03*m12*m21 - m01*m12*m23 - m02*m13*m21 - m03*m11*m22
    // 
    // a10 = m10*m23*m32 + m12*m20*m33 + m13*m22*m30 - m10*m22*m33 - m12*m23*m30 - m13*m20*m32
    // a11 = m00*m22*m33 + m02*m23*m30 + m03*m20*m32 - m00*m23*m32 - m02*m20*m33 - m03*m22*m30
    // a12 = m00*m13*m32 + m02*m10*m33 + m03*m12*m30 - m00*m12*m33 - m02*m13*m30 - m03*m10*m32
    // a13 = m00*m12*m23 + m02*m13*m20 + m03*m10*m22 - m00*m13*m22 - m02*m10*m23 - m03*m12*m20
    //
    // a20 = m10*m21*m33 + m11*m23*m30 + m13*m20*m31 - m10*m23*m31 - m11*m20*m33 - m13*m21*m30
    // a21 = m00*m23*m31 + m01*m20*m33 + m03*m21*m30 - m00*m21*m33 - m01*m23*m30 - m03*m20*m31
    // a22 = m00*m11*m33 + m01*m13*m30 + m03*m10*m31 - m00*m13*m31 - m01*m10*m33 - m03*m11*m30
    // a23 = m00*m13*m21 + m01*m10*m23 + m03*m11*m20 - m00*m11*m23 - m01*m13*m20 - m03*m10*m21
    //
    // a30 = m10*m22*m31 + m11*m20*m32 + m12*m21*m30 - m10*m21*m32 - m11*m22*m30 - m12*m20*m31
    // a31 = m00*m21*m32 + m01*m22*m30 + m02*m20*m31 - m00*m22*m31 - m01*m20*m32 - m02*m21*m30
    // a32 = m00*m12*m31 + m01*m10*m32 + m02*m11*m30 - m00*m11*m32 - m01*m12*m30 - m02*m10*m31
    // a33 = m00*m11*m22 + m01*m12*m20 + m02*m10*m21 - m00*m12*m21 - m01*m10*m22 - m02*m11*m20
    // 


    let det = matrix_determinant(m);
    if det.abs() <= f32::EPSILON {
        return None;
    }

    unsafe { 
        let recip_det = det.recip();

        let m00 = vdupq_laneq_f32::<0b00>(m[0]); // [m00, m00, m00, m00]
        let m01 = vdupq_laneq_f32::<0b01>(m[0]); // [m01, m01, m01, m01]
        let m02 = vdupq_laneq_f32::<0b10>(m[0]); // [m02, m02, m02, m02]
        let m03 = vdupq_laneq_f32::<0b11>(m[0]); // [m03, m03, m03, m03]
        let m10 = vdupq_laneq_f32::<0b00>(m[1]); // [m10, m10, m10, m10]
        let m11 = vdupq_laneq_f32::<0b01>(m[1]); // [m11, m11, m11, m11]
        let m12 = vdupq_laneq_f32::<0b10>(m[1]); // [m12, m12, m12, m12]
        let m13 = vdupq_laneq_f32::<0b11>(m[1]); // [m13, m13, m13, m13]
        
        let m1_xy = vget_low_f32(m[1]); // [m10, m11]
        let m1_yx = vext_f32::<0b01>(m1_xy, m1_xy); // [m11, m10]
        let m2_xy = vget_low_f32(m[2]); // [m20, m21]
        let m2_yx = vext_f32::<0b01>(m2_xy, m2_xy); // [m21, m20]
        let m3_xy = vget_low_f32(m[3]); // [m30, m31]
        let m3_yx = vext_f32::<0b01>(m3_xy, m3_xy); // [m31, m30]

        let m1_zw = vget_high_f32(m[1]); // [m12, m13]
        let m1_wz = vext_f32::<0b01>(m1_zw, m1_zw); // [m13, m12]
        let m2_zw = vget_high_f32(m[2]); // [m22, m23]
        let m2_wz = vext_f32::<0b01>(m2_zw, m2_zw); // [m23, m22]
        let m3_zw = vget_high_f32(m[3]); // [m32, m33]
        let m3_wz = vext_f32::<0b01>(m3_zw, m3_zw); // [m33, m32]
        
        let m1_yw = vext_f32::<0b01>(m1_xy, m1_wz); // [m11, m13]
        let m1_wy = vext_f32::<0b01>(m1_yw, m1_yw); // [m13, m11]
        let m2_yw = vext_f32::<0b01>(m2_xy, m2_wz); // [m21, m23]
        let m2_wy = vext_f32::<0b01>(m2_yw, m2_yw); // [m23, m21]
        let m3_yw = vext_f32::<0b01>(m3_xy, m3_wz); // [m31, m33]
        let m3_wy = vext_f32::<0b01>(m3_yw, m3_yw); // [m33, m31]

        let m1_yz = vext_f32::<0b01>(m1_xy, m1_zw); // [m11, m12]
        let m1_zy = vext_f32::<0b01>(m1_yz, m1_yz); // [m12, m11]
        let m2_yz = vext_f32::<0b01>(m2_xy, m2_zw); // [m21, m22]
        let m2_zy = vext_f32::<0b01>(m2_yz, m2_yz); // [m22, m21]
        let m3_yz = vext_f32::<0b01>(m3_xy, m3_zw); // [m31, m32]
        let m3_zy = vext_f32::<0b01>(m3_yz, m3_yz); // [m32, m31]

        let m1_zx = vext_f32::<0b01>(m1_wz, m1_xy); // [m12, m10]
        let m1_xz = vext_f32::<0b01>(m1_zx, m1_zx); // [m10, m12]
        let m2_zx = vext_f32::<0b01>(m2_wz, m2_xy); // [m22, m20]
        let m2_xz = vext_f32::<0b01>(m2_zx, m2_zx); // [m20, m22]
        let m3_zx = vext_f32::<0b01>(m3_wz, m3_xy); // [m32, m30]
        let m3_xz = vext_f32::<0b01>(m3_zx, m3_zx); // [m30, m32]

        let m1_xw = vext_f32::<0b01>(m1_yx, m1_wz); // [m10, m13]
        let m1_wx = vext_f32::<0b01>(m1_xw, m1_xw); // [m13, m10]
        let m2_xw = vext_f32::<0b01>(m2_yx, m2_wz); // [m20, m23]
        let m2_wx = vext_f32::<0b01>(m2_xw, m2_xw); // [m23, m20]
        let m3_xw = vext_f32::<0b01>(m3_yx, m3_wz); // [m30, m33]
        let m3_wx = vext_f32::<0b01>(m3_xw, m3_xw); // [m33, m30]

        let m30_m21 = vext_f32::<0b01>(m3_yx, m2_yx); // [m30, m21]
        let m30_m22 = vext_f32::<0b01>(m3_yx, m2_zw); // [m30, m22]
        let m30_m23 = vext_f32::<0b01>(m3_yx, m2_wz); // [m30, m23]
        let m31_m20 = vext_f32::<0b01>(m3_xy, m2_xy); // [m31, m20]
        let m31_m22 = vext_f32::<0b01>(m3_xy, m2_zw); // [m31, m22]
        let m31_m23 = vext_f32::<0b01>(m3_xy, m2_wy); // [m31, m23]
        let m32_m20 = vext_f32::<0b01>(m3_wz, m2_xy); // [m32, m20]
        let m32_m21 = vext_f32::<0b01>(m3_wz, m2_yx); // [m32, m21]
        let m32_m23 = vext_f32::<0b01>(m3_wz, m2_wz); // [m32, m23]
        let m33_m20 = vext_f32::<0b01>(m3_zw, m2_xy); // [m33, m20]
        let m33_m21 = vext_f32::<0b01>(m3_zw, m2_yx); // [m33, m21]
        let m33_m22 = vext_f32::<0b01>(m3_zw, m2_zw); // [m33, m22]

        // generate [a00, a01, a02, a03]
        let e00 = vextq_f32::<0b11>(m11, m01); // [m11, m01, m01, m01]
        let e01 = vcombine_f32(m2_zw, m1_zw); // [m22, m23, m12, m13]
        let e02 = vcombine_f32(m3_wz, m33_m22); // [m33, m32, m33, m22]
        let e0 = vmulq_f32(e00, e01);
        let e0 = vmulq_f32(e0, e02);

        let e10 = vextq_f32::<0b11>(m12, m02); // [m12, m02, m02, m02]
        let e11 = vcombine_f32(m2_wy, m1_wy); // [m23, m21, m13, m11]
        let e12 = vcombine_f32(m3_yw, m31_m23); // [m31, m33, m31, m23]
        let e1 = vmulq_f32(e10, e11);
        let e1 = vmulq_f32(e1, e12);
        
        let e20 = vextq_f32::<0b11>(m13, m03); // [m13, m03, m03, m03]
        let e21 = vcombine_f32(m2_yz, m1_yz); // [m21, m22, m11, m12]
        let e22 = vcombine_f32(m3_zy, m32_m21); // [m32, m31, m32, m21]
        let e2 = vmulq_f32(e20, e21);
        let e2 = vmulq_f32(e2, e22);

        let e30 = vextq_f32::<0b11>(m11, m01); // [m11, m01, m01, m01]
        let e31 = vcombine_f32(m2_wz, m1_wz); // [m23, m22, m13, m12]
        let e32 = vcombine_f32(m3_zw, m32_m23); // [m32, m33, m32, m23]
        let e3 = vmulq_f32(e30, e31);
        let e3 = vmulq_f32(e3, e32);

        let e40 = vextq_f32::<0b11>(m12, m02); // [m12, m02, m02, m02]
        let e41 = vcombine_f32(m2_yw, m1_yw); // [m21, m23, m11, m13]
        let e42 = vcombine_f32(m3_wy, m33_m21); // [m33, m31, m33, m21]
        let e4 = vmulq_f32(e40, e41); 
        let e4 = vmulq_f32(e4, e42);

        let e50 = vextq_f32::<0b11>(m13, m03); // [m13, m03, m03, m03]
        let e51 = vcombine_f32(m2_zy, m1_zy); // [m22, m21, m12, m11]
        let e52 = vcombine_f32(m3_yz, m31_m22); // [m31, m32, m31, m22]
        let e5 = vmulq_f32(e50, e51);
        let e5 = vmulq_f32(e5, e52);

        let lhs = vaddq_f32(e0, e1);
        let lhs = vaddq_f32(lhs, e2);
        let rhs = vaddq_f32(e3, e4);
        let rhs = vaddq_f32(rhs, e5);
        let v0 = vsubq_f32(lhs, rhs);
        let v0 = vmulq_n_f32(v0, recip_det);


        // generate [a10, a11, a12, a13]
        let e00 = vextq_f32::<0b11>(m10, m00); // [m10, m00, m00, m00]
        let e01 = vcombine_f32(m2_wz, m1_wz); // [m23, m22, m13, m12]
        let e02 = vcombine_f32(m3_zw, m32_m23); // [m32, m33, m32, m23]
        let e0 = vmulq_f32(e00, e01);
        let e0 = vmulq_f32(e0, e02);

        let e10 = vextq_f32::<0b11>(m12, m02); // [m12, m02, m02, m02]
        let e11 = vcombine_f32(m2_xw, m1_xw); // [m20, m23, m10, m13]
        let e12 = vcombine_f32(m3_wx, m33_m20); // [m33, m30, m33, m20]
        let e1 = vmulq_f32(e10, e11);
        let e1 = vmulq_f32(e1, e12);

        let e20 = vextq_f32::<0b11>(m13, m03); // [m13, m03, m03, m03]
        let e21 = vcombine_f32(m2_zx, m1_zx); // [m22, m20, m12, m10]
        let e22 = vcombine_f32(m3_xz, m30_m22); // [m30, m32, m30, m22]
        let e2 = vmulq_f32(e20, e21);
        let e2 = vmulq_f32(e2, e22);

        let e30 = vextq_f32::<0b11>(m10, m00); // [m10, m00, m00, m00]
        let e31 = vcombine_f32(m2_zw, m1_zw); // [m22, m23, m12, m13]
        let e32 = vcombine_f32(m3_wz, m33_m22); // [m33, m32, m33, m22]
        let e3 = vmulq_f32(e30, e31);
        let e3 = vmulq_f32(e3, e32);

        let e40 = vextq_f32::<0b11>(m12, m02); // [m12, m02, m02, m02]
        let e41 = vcombine_f32(m2_wx, m1_wx); // [m23, m20, m13, m10]
        let e42 = vcombine_f32(m3_xw, m30_m23); // [m30, m33, m30, m23]
        let e4 = vmulq_f32(e40, e41);
        let e4 = vmulq_f32(e4, e42);

        let e50 = vextq_f32::<0b11>(m13, m03); // [m13, m03, m03, m03]
        let e51 = vcombine_f32(m2_xz, m1_xz); // [m20, m22, m10, m12]
        let e52 = vcombine_f32(m3_zx, m32_m20); // [m32, m30, m32, m20]
        let e5 = vmulq_f32(e50, e51);
        let e5 = vmulq_f32(e5, e52);

        let lhs = vaddq_f32(e0, e1);
        let lhs = vaddq_f32(lhs, e2);
        let rhs = vaddq_f32(e3, e4);
        let rhs = vaddq_f32(rhs, e5);
        let v1 = vsubq_f32(lhs, rhs);
        let v1 = vmulq_n_f32(v1, recip_det);


        // generate [a20, a21, a22, a23]
        let e00 = vextq_f32::<0b11>(m10, m00); // [m10, m00, m00, m00]
        let e01 = vcombine_f32(m2_yw, m1_yw); // [m21, m23, m11, m13]
        let e02 = vcombine_f32(m3_wy, m33_m21); // [m33, m31, m33, m21]
        let e0 = vmulq_f32(e00, e01);
        let e0 = vmulq_f32(e0, e02);

        let e10 = vextq_f32::<0b11>(m11, m01); // [m11, m01, m01, m01]
        let e11 = vcombine_f32(m2_wx, m1_wx); // [m23, m20, m13, m10]
        let e12 = vcombine_f32(m3_xw, m30_m23); // [m30, m33, m30, m23]
        let e1 = vmulq_f32(e10, e11);
        let e1 = vmulq_f32(e1, e12);

        let e20 = vextq_f32::<0b11>(m13, m03); // [m13, m03, m03, m03]
        let e21 = vcombine_f32(m2_xy, m1_xy); // [m20, m21, m10, m11]
        let e22 = vcombine_f32(m3_yx, m31_m20); // [m31, m30, m31, m20]
        let e2 = vmulq_f32(e20, e21);
        let e2 = vmulq_f32(e2, e22);

        let e30 = vextq_f32::<0b11>(m10, m00); // [m10, m00, m00, m00]
        let e31 = vcombine_f32(m2_wy, m1_wy); // [m23, m21, m13, m11]
        let e32 = vcombine_f32(m3_yw, m31_m23); // [m31, m33, m31, m23]
        let e3 = vmulq_f32(e30, e31);
        let e3 = vmulq_f32(e3, e32);

        let e40 = vextq_f32::<0b11>(m11, m01); // [m11, m01, m01, m01]
        let e41 = vcombine_f32(m2_xw, m1_xw); // [m20, m23, m10, m13]
        let e42 = vcombine_f32(m3_wx, m33_m20); // [m33, m30, m33, m20]
        let e4 = vmulq_f32(e40, e41);
        let e4 = vmulq_f32(e4, e42);

        let e50 = vextq_f32::<0b11>(m13, m03); // [m13, m03, m03, m03]
        let e51 = vcombine_f32(m2_yx, m1_yx); // [m21, m20, m11, m10]
        let e52 = vcombine_f32(m3_xy, m30_m21); // [m30, m31, m30, m21]
        let e5 = vmulq_f32(e50, e51);
        let e5 = vmulq_f32(e5, e52);

        let lhs = vaddq_f32(e0, e1);
        let lhs = vaddq_f32(lhs, e2);
        let rhs = vaddq_f32(e3, e4);
        let rhs = vaddq_f32(rhs, e5);
        let v2 = vsubq_f32(lhs, rhs);
        let v2 = vmulq_n_f32(v2, recip_det);


        // generate [a30, a31, a32, a33]
        let e00 = vextq_f32::<0b11>(m10, m00); // [m10, m00, m00, m00]
        let e01 = vcombine_f32(m2_zy, m1_zy); // [m22, m21, m12, m11]
        let e02 = vcombine_f32(m3_yz, m31_m22); // [m31, m32, m31, m22]
        let e0 = vmulq_f32(e00, e01);
        let e0 = vmulq_f32(e0, e02);

        let e10 = vextq_f32::<0b11>(m11, m01); // [m11, m01, m01, m01]
        let e11 = vcombine_f32(m2_xz, m1_xz); // [m20, m22, m10, m12]
        let e12 = vcombine_f32(m3_zx, m32_m20); // [m32, m30, m32, m20]
        let e1 = vmulq_f32(e10, e11);
        let e1 = vmulq_f32(e1, e12);

        let e20 = vextq_f32::<0b11>(m12, m02); // [m12, m02, m02, m02]
        let e21 = vcombine_f32(m2_yx, m1_yx); // [m21, m20, m11, m10]
        let e22 = vcombine_f32(m3_xy, m30_m21); // [m30, m31, m30, m21]
        let e2 = vmulq_f32(e20, e21);
        let e2 = vmulq_f32(e2, e22);

        let e30 = vextq_f32::<0b11>(m10, m00); // [m10, m00, m00, m00]
        let e31 = vcombine_f32(m2_yz, m1_yz); // [m21, m22, m11, m12]
        let e32 = vcombine_f32(m3_zy, m32_m21); // [m32, m31, m32, m21]
        let e3 = vmulq_f32(e30, e31);
        let e3 = vmulq_f32(e3, e32);

        let e40 = vextq_f32::<0b11>(m11, m01); // [m11, m01, m01, m01]
        let e41 = vcombine_f32(m2_zx, m1_zx); // [m22, m20, m12, m10]
        let e42 = vcombine_f32(m3_xz, m30_m22); // [m30, m32, m30, m22]
        let e4 = vmulq_f32(e40, e41);
        let e4 = vmulq_f32(e4, e42);

        let e50 = vextq_f32::<0b11>(m12, m02); // [m12, m02, m02, m02]
        let e51 = vcombine_f32(m2_xy, m1_xy); // [m20, m21, m10, m11]
        let e52 = vcombine_f32(m3_yx, m31_m20); // [m31, m30, m31, m20]
        let e5 = vmulq_f32(e50, e51);
        let e5 = vmulq_f32(e5, e52);

        let lhs = vaddq_f32(e0, e1);
        let lhs = vaddq_f32(lhs, e2);
        let rhs = vaddq_f32(e3, e4);
        let rhs = vaddq_f32(rhs, e5);
        let v3 = vsubq_f32(lhs, rhs);
        let v3 = vmulq_n_f32(v3, recip_det);

        Some([v0, v1, v2, v3])
    }
}
