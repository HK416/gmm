use core::fmt;
use core::ops;
use core::arch::aarch64::*;
use crate::{ VectorInt, Float2, Float3, Float4 };

use super::Quaternion;



/// This is a vector data type that uses the `SIMD` instruction.
/// 
/// Using the `arm neon` instruction.
/// 
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub union Vector {
    /// member variables for constant variables.
    arr: [f32; 4], 

    pub(crate) inner: float32x4_t, 
}

impl Vector {
    /// All elements are zeros.
    pub const ZERO: Self = Self { arr: [0.0; 4] };

    /// All elements are one.
    pub const ONE: Self = Self { arr: [1.0; 4] };

    /// All elements are negative one.
    pub const NEG_ONE: Self = Self { arr: [-1.0; 4] };

    /// All elements are `f32::MIN`.
    pub const MIN: Self = Self { arr: [f32::MIN; 4] };

    /// All elements are `f32::MAX`.
    pub const MAX: Self = Self { arr: [f32::MAX; 4] };

    /// All elements are `f32::INFINITY`.
    pub const INFINITY: Self = Self { arr: [f32::INFINITY; 4] };

    /// All elements are `f32::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self { arr: [f32::NEG_INFINITY; 4] };

    /// positive unit vector on x-axis.
    pub const X: Self = Self { arr: [1.0, 0.0, 0.0, 0.0] };

    /// positive unit vector on y-axis.
    pub const Y: Self = Self { arr: [0.0, 1.0, 0.0, 0.0] };

    /// positive unit vector on z-axis.
    pub const Z: Self = Self { arr: [0.0, 0.0, 1.0, 0.0] };

    /// positive unit vector on w-axis.
    pub const W: Self = Self { arr: [0.0, 0.0, 0.0, 1.0] };

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self { arr: [-1.0, 0.0, 0.0, 0.0] };

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self { arr: [0.0, -1.0, 0.0, 0.0] };

    /// negative unit vector on z-axis.
    pub const NEG_Z: Self = Self { arr: [0.0, 0.0, -1.0, 0.0] };

    /// negative unit vector on w-axis.
    pub const NEG_W: Self = Self { arr: [0.0, 0.0, 0.0, -1.0] };

    /// Some SIMD instructions do not conform to IEEE-754. (for performance benefits)
    /// 
    /// So we compare using a separate Epsilon constant.
    /// 
    pub const EPSILON: Self = Self { arr: [1.192092896e-6; 4] };
}

impl Vector {
    /// Creates with given elements.
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe {
            let arr = [x, y, z, w];
            Self { inner: vld1q_f32(arr.as_ptr()) }
        }
    }

    /// Fills all elements with the given values.
    #[inline]
    #[must_use]
    pub fn fill(v: f32) -> Self {
        unsafe { Self { inner: vdupq_n_f32(v) } }
    }

    /// Creates from a given array.
    #[inline]
    #[must_use]
    pub fn from_array(arr: [f32; 4]) -> Self {
        unsafe { Self { inner: vld1q_f32(arr.as_ptr()) } }
    }

    /// Stores the value in an array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> [f32; 4] {
        let mut arr = [0.0; 4];
        unsafe { vst1q_f32(arr.as_mut_ptr(), self.inner) };
        return arr;
    }

    /// Creates from a given array of slice.
    /// 
    /// # Panics
    /// When the `use-assertion` feature is enabled, it will [`panic!`]
    /// if the array slice has less than four elements.
    /// 
    #[inline]
    #[must_use]
    pub fn from_slice(slice: &[f32]) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(slice.len() >= 4, "The given array slice has less than four elements!");
        unsafe { Self { inner: vld1q_f32(slice.as_ptr()) } }
    }

    /// Converts a given quaternion to a vector.
    #[inline]
    #[must_use]
    pub fn from_quaternion(q: Quaternion) -> Self {
        Self { inner: unsafe { q.inner } }
    }

    /// Converts a vector to a quaternion.
    #[inline]
    #[must_use]
    pub fn into_quaternion(self) -> Quaternion {
        Quaternion { inner: unsafe { self.inner } }
    }

    /// Loads a value from a given `Float2`.
    #[inline]
    #[must_use]
    pub fn load_float2(val: Float2) -> Self {
        Self::load_float4(val.into())
    }

    /// Stores the value in a `Float2`.
    #[inline]
    #[must_use]
    pub fn store_float2(self) -> Float2 {
        self.store_float4().xy()
    }

    /// Loads a value from a given `Float3`.
    #[inline]
    #[must_use]
    pub fn load_float3(val: Float3) -> Self {
        Self::load_float4(val.into())
    }

    /// Stores the value in a `Float3`.
    #[inline]
    #[must_use]
    pub fn store_float3(self) -> Float3 {
        self.store_float4().xyz()
    }

    /// Loads a value from a given `Float4`.
    #[inline]
    #[must_use]
    pub fn load_float4(val: Float4) -> Self {
        unsafe {
            Self { inner: vld1q_f32(&val as *const _ as *const f32) }
        }
    }

    /// Stores the value in a `Float4`.
    #[inline]
    #[must_use]
    pub fn store_float4(self) -> Float4 {
        unsafe {
            let mut val = Float4::default();
            vst1q_f32(&mut val as *mut _ as *mut f32, self.inner);
            val
        }
    }
}

impl Vector {
    /// Get the `x` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_x(&self) -> f32 {
        unsafe { vgetq_lane_f32::<0b00>(self.inner) }
    }

    /// Set the `x` element of a vector.
    #[inline]
    pub fn set_x(&mut self, v: f32) {
        unsafe { self.inner = vsetq_lane_f32::<0b00>(v, self.inner) }
    }

    /// Get the `y element of a vector.
    #[inline]
    #[must_use]
    pub fn get_y(&self) -> f32 {
        unsafe { vgetq_lane_f32::<0b01>(self.inner) }
    }

    /// Set the `y` element of a vector.
    #[inline]
    pub fn set_y(&mut self, v: f32) {
        unsafe { self.inner = vsetq_lane_f32::<0b01>(v, self.inner) }
    }

    /// Get the `z` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_z(&self) -> f32 {
        unsafe { vgetq_lane_f32::<0b10>(self.inner) }
    }

    /// Set the `z`` element of a vector.
    #[inline]
    pub fn set_z(&mut self, v: f32) {
        unsafe { self.inner = vsetq_lane_f32::<0b10>(v, self.inner) }
    }

    /// Get the `w` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_w(&self) -> f32 {
        unsafe { vgetq_lane_f32::<0b11>(self.inner) }
    }

    /// Set the `w` element of a vector.
    #[inline]
    pub fn set_w(&mut self, v: f32) {
        unsafe { self.inner = vsetq_lane_f32::<0b11>(v, self.inner) }
    }

    /// Takes the samller of the elements of the two vectors.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        unsafe { Vector { inner: vminq_f32(self.inner, rhs.inner) } }
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        unsafe { Vector { inner: vmaxq_f32(self.inner, rhs.inner) } }
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    #[must_use]
    pub fn lt(self, rhs: Self) -> VectorInt {
        unsafe { 
            let comp = vcltq_f32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are less than or equal.
    #[inline]
    #[must_use]
    pub fn le(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcleq_f32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    #[must_use]
    pub fn gt(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcgtq_f32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are greater than or equal.
    #[inline]
    #[must_use]
    pub fn ge(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcgeq_f32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are equal.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn eq(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vceqq_f32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are not equal.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn ne(self, rhs: Self) -> VectorInt {
        !self.eq(rhs)
    }

    /// Absolute value on vector elements.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        unsafe { Vector { inner: vabsq_f32(self.inner) } }
    }

    /// Return a vector filled by adding all the elements of the vector.
    #[inline]
    #[must_use]
    pub fn sum(self) -> Self {
        unsafe {
            let sum = vpaddq_f32(self.inner, self.inner);
            let sum = vpaddq_f32(sum, sum);
            return Vector { inner: sum };
        }
    }

    /// Returns the sum of all elements in a vector.
    #[inline]
    #[must_use]
    pub fn sum_into(self) -> f32 {
        self.sum().get_x()
    }

    /// Returns a vector filled with the dot products of the two-element vectors.
    #[inline]
    #[must_use]
    pub fn vec2_dot(self, rhs: Self) -> Self {
        unsafe {
            let mul = self * rhs;
            let low = vget_low_f32(mul.inner);
            let sum = vpadd_f32(low, low);
            return Vector { inner: vcombine_f32(sum, sum) };
        }
    }

    /// Dot product of the two-element vectors.
    #[inline]
    #[must_use]
    pub fn vec2_dot_into(self, rhs: Self) -> f32 {
        self.vec2_dot(rhs).get_x()
    }

    /// Returns a vector filled with the dot products of the three-element vectors.
    #[inline]
    #[must_use]
    pub fn vec3_dot(self, rhs: Self) -> Self {
        unsafe {
            let mul = self * rhs;
            let low = vget_low_f32(mul.inner);
            let low = vpadd_f32(low, low);
            let high = vdup_laneq_f32::<0b10>(mul.inner);
            let sum = vadd_f32(low, high);
            return Vector { inner: vcombine_f32(sum, sum) };
        }
    }

    /// Dot product of the three-element vectors.
    #[inline]
    #[must_use]
    pub fn vec3_dot_into(self, rhs: Self) -> f32 {
        self.vec3_dot(rhs).get_x()
    }

    /// Returns a vector filled with the dot products of the four-element vectors.
    #[inline]
    #[must_use]
    pub fn vec4_dot(self, rhs: Self) -> Self {
        (self * rhs).sum()
    }

    /// Dot product of the four-element vectors.
    #[inline]
    #[must_use]
    pub fn vec4_dot_into(self, rhs: Self) -> f32 {
        self.vec4_dot(rhs).get_x()
    }

    /// Cross product of the three-element vectors.
    #[must_use]
    pub fn vec3_cross(self, rhs: Self) -> Self {
        const MASK_XYZ: Vector = Vector { arr: [1.0, 1.0, 1.0, 0.0] };
        unsafe {
            let lx_ly = vget_low_f32(self.inner);
            let lz_lw = vget_high_f32(self.inner);
            let ly_lz = vext_f32::<0b01>(lx_ly, lz_lw);
            let lz_lx = vext_f32::<0b01>(ly_lz, lx_ly);

            let rx_ry = vget_low_f32(rhs.inner);
            let rz_rw = vget_high_f32(rhs.inner);
            let ry_rz = vext_f32::<0b01>(rx_ry, rz_rw);
            let rz_rx = vext_f32::<0b01>(ry_rz, rx_ry);

            let ly_lz_lx = Vector { inner: vcombine_f32(ly_lz, lx_ly) };
            let lz_lx_ly = Vector { inner: vcombine_f32(lz_lx, ly_lz) };
            let rz_rx_ry = Vector { inner: vcombine_f32(rz_rx, ry_rz) };
            let ry_rz_rx = Vector { inner: vcombine_f32(ry_rz, rx_ry) };

            (ly_lz_lx * rz_rx_ry - lz_lx_ly * ry_rz_rx) * MASK_XYZ
        }
    }

    /// Length squared of a two-element vector.
    #[inline]
    #[must_use]
    pub fn vec2_len_sq(self) -> f32 {
        self.vec2_dot(self).get_x()
    }

    /// Length of a two-element vector.
    #[inline]
    #[must_use]
    pub fn vec2_len(self) -> f32 {
        self.vec2_len_sq().sqrt()
    }

    /// Length squared of a three-element vector.
    #[inline]
    #[must_use]
    pub fn vec3_len_sq(self) -> f32 {
        self.vec3_dot(self).get_x()
    }

    /// Length of a three-element vector.
    #[inline]
    #[must_use]
    pub fn vec3_len(self) -> f32 {
        self.vec3_len_sq().sqrt()
    }

    /// Length squared of a four-element vector.
    #[inline]
    #[must_use]
    pub fn vec4_len_sq(self) -> f32 {
        self.vec4_dot(self).get_x()
    }

    /// Length of a four-element vector.
    #[inline]
    #[must_use]
    pub fn vec4_len(self) -> f32 {
        self.vec4_len_sq().sqrt()
    }

    /// Returns `true` if it is a unit vector.
    #[inline]
    #[must_use]
    pub fn is_vec2_normalized(self) -> bool {
        (self.vec2_len() - 1.0) <= f32::EPSILON
    }

    /// Normalizes a two-element vector.
    /// 
    /// Undefined behavior may occur if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    /// # Panics
    /// When `use-assertion` feature is enabled, [`panic!`] will be called 
    /// if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn vec2_normalize(self) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(self.vec2_len() <= f32::EPSILON, "The length of the vector is less than or equal to `f32::EPSILON`!");
        
        const MASK_XY: Vector = Vector { arr: [1.0, 1.0, 0.0, 0.0] };
        self * self.vec2_len().recip() * MASK_XY
    }

    /// Normalizes a two-element vector. 
    /// 
    /// Returns `None` if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn try_vec2_normalize(self) -> Option<Self> {
        const MASK_XY: Vector = Vector { arr: [1.0, 1.0, 0.0, 0.0] };
        let length = self.vec2_len();
        if length <= f32::EPSILON {
            return None;
        }
        Some(self * length.recip() * MASK_XY)
    }

    /// Returns `true` if it is a unit vector.
    #[inline]
    #[must_use]
    pub fn is_vec3_normalized(self) -> bool {
        (self.vec3_len() - 1.0).abs() <= f32::EPSILON
    }

    /// Normalizes a three-element vector.
    /// 
    /// Undefined behavior may occur if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    /// # Panics
    /// When `use-assertion` feature is enabled, [`panic!`] will be called 
    /// if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn vec3_normalize(self) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(self.vec3_len() <= f32::EPSILON, "The length of the vector is less than or equal to `f32::EPSILON`!");

        const MASK_XYZ: Vector = Vector { arr: [1.0, 1.0, 1.0, 0.0] };
        self * self.vec3_len().recip() * MASK_XYZ
    }

    /// Normalizes a three-element vector. 
    /// 
    /// Returns `None` if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn try_vec3_normalize(self) -> Option<Self> {
        const MASK_XYZ: Vector = Vector { arr: [1.0, 1.0, 1.0, 0.0] };
        let length = self.vec3_len();
        if length <= f32::EPSILON {
            return None;
        }
        Some(self * length.recip() * MASK_XYZ)
    }

    /// Returns `true` if it is a unit vector.
    #[inline]
    #[must_use]
    pub fn is_vec4_normalized(self) -> bool {
        (self.vec4_len() - 1.0).abs() <= f32::EPSILON
    }

    /// Normalizes a four-element vector.
    /// 
    /// Undefined behavior may occur if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    /// # Panics
    /// When `use-assertion` feature is enabled, [`panic!`] will be called 
    /// if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn vec4_normalize(self) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(self.vec4_len() <= f32::EPSILON, "The length of the vector is less than or equal to `f32::EPSILON`!");
        self * self.vec4_len().recip()
    }

    /// Normalizes a four-element vector. 
    /// 
    /// Returns `None` if the length of the vector is less than or equal to [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn try_vec4_normalize(self) -> Option<Self> {
        let length = self.vec4_len();
        if length <= f32::EPSILON {
            return None;
        }
        Some(self * length.recip())
    }

    /// Returns a vector containing the smallest integer less than or equal to the number for each element of the vector.
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        unsafe { Vector { inner: vrndmq_f32(self.inner) } }
    }

    /// Returns a vector containing the smallest integer greater than or equal to the number for each element of the vector.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        unsafe { Vector { inner: vrndpq_f32(self.inner) } }
    }

    /// Returns a vector with the numbers for each element of the vector rounded down to the nearest decimal.
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        unsafe { Vector { inner: vrndnq_f32(self.inner) } }
    }

    /// Returns a vector that is a linear interpolation of two vectors.
    /// 
    /// The given `t` must be in the range zero to one.
    ///  
    /// The closer `t` is to one, the more it becomes equal to the given `rhs`.
    /// 
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        self * (1.0 - t) + rhs * t
    }
}

impl Default for Vector {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<[f32; 4]> for Vector {
    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self::from_array(value)
    }
}

impl Into<[f32; 4]> for Vector {
    #[inline]
    fn into(self) -> [f32; 4] {
        self.into_array()
    }
}

impl From<Float2> for Vector {
    #[inline]
    fn from(value: Float2) -> Self {
        Self::load_float2(value)
    }
}

impl Into<Float2> for Vector {
    #[inline]
    fn into(self) -> Float2 {
        self.store_float2()
    }
}

impl From<Float3> for Vector {
    #[inline]
    fn from(value: Float3) -> Self {
        Self::load_float3(value)
    }
}

impl Into<Float3> for Vector {
    #[inline]
    fn into(self) -> Float3 {
        self.store_float3()
    }
}

impl From<Float4> for Vector {
    #[inline]
    fn from(value: Float4) -> Self {
        Self::load_float4(value)
    }
}

impl Into<Float4> for Vector {
    #[inline]
    fn into(self) -> Float4 {
        self.store_float4()
    }
}

impl fmt::Debug for Vector {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vector))
            .field(unsafe { &self.inner })
            .finish()
    }
}

impl ops::Add<Vector> for f32 {
    type Output = Vector;
    /// Adds a scalar value to each element of a vector.
    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::fill(self) + rhs
    }
}

impl ops::Add<f32> for Vector {
    type Output = Vector;
    /// Adds a scalar value to each element of a vector.
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        self + Self::fill(rhs)
    }
}

impl ops::AddAssign<f32> for Vector {
    /// Adds a scalar value to each element of a vector.
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    /// Adds two vectors.
    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        unsafe { Vector { inner: vaddq_f32(self.inner, rhs.inner) } }
    }
}

impl ops::AddAssign<Vector> for Vector {
    /// Adds two vectors.
    #[inline]
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs
    }
}

impl ops::Sub<Vector> for f32 {
    type Output = Vector;
    /// Subtracts a scalar value to each element of a vector.
    #[inline]
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::fill(self) - rhs
    }
}

impl ops::Sub<f32> for Vector {
    type Output = Vector;
    /// Subtracts a scalar value to each element of a vector.
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        self - Self::fill(rhs)
    }
}

impl ops::SubAssign<f32> for Vector {
    /// Subtracts a scalar value to each element of a vector.
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    /// Subtracts two vectors.
    #[inline]
    fn sub(self, rhs: Vector) -> Self::Output {
        unsafe { Vector { inner: vsubq_f32(self.inner, rhs.inner) } }
    }
}

impl ops::SubAssign<Vector> for Vector {
    /// Subtracts two vectors.
    #[inline]
    fn sub_assign(&mut self, rhs: Vector) {
        *self = *self - rhs
    }
}

impl ops::Neg for Vector {
    type Output = Vector;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        unsafe { Vector { inner: vnegq_f32(self.inner) } }
    }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;
    /// Multiplies each element of a vector by a scalar value.
    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::fill(self) * rhs
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;
    /// Multiplies each element of a vector by a scalar value.
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self * Self::fill(rhs)
    }
}

impl ops::MulAssign<f32> for Vector {
    /// Multiplies each element of a vector by a scalar value.
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        unsafe { Vector { inner: vmulq_f32(self.inner, rhs.inner) } }
    }
}

impl ops::MulAssign<Vector> for Vector {
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul_assign(&mut self, rhs: Vector) {
        *self = *self * rhs
    }
}

impl ops::Div<Vector> for f32 {
    type Output = Vector;
    /// Divides each element of a vector by a scalar value.
    #[inline]
    fn div(self, rhs: Vector) -> Self::Output {
        Vector::fill(self) / rhs
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;
    /// Divides each element of a vector by a scalar value.
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self / Self::fill(rhs)
    }
}

impl ops::DivAssign<f32> for Vector {
    /// Divides each element of a vector by a scalar value.
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs
    }
}

impl ops::Div<Vector> for Vector {
    type Output = Vector;
    /// Element-wise division of two vectors.
    #[inline]
    fn div(self, rhs: Vector) -> Self::Output {
        unsafe { Vector { inner: vdivq_f32(self.inner, rhs.inner) } }
    }
}

impl ops::DivAssign<Vector> for Vector {
    /// Element-wise division of two vectors.
    #[inline]
    fn div_assign(&mut self, rhs: Vector) {
        *self = *self / rhs
    }
}
