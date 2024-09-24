use core::fmt;
use core::ops;
use core::arch::aarch64::*;
use crate::{
    Vector, 
    Integer2, Integer3, Integer4, 
    UInteger2, UInteger3, UInteger4
};



/// This is a vector data type that uses the `SIMD` instruction.
/// 
/// Using the `arm neon` instruction.
/// 
#[repr(C)]
#[derive(Clone, Copy)]
pub union VectorInt {
    /// member variables for constant variables.
    arr: [i32; 4], 

    pub(crate) inner: int32x4_t
}

impl VectorInt {
    /// All elements are zeros.
    pub const ZERO: Self = Self { arr: [0; 4] };

    /// All elements are one.
    pub const ONE: Self = Self { arr: [1; 4] };

    /// All elements are negative one.
    pub const NEG_ONE: Self = Self { arr: [-1; 4] };

    /// positive unit vector on x-axis.
    pub const X: Self = Self { arr: [1, 0, 0, 0] };

    /// positive unit vector on y-axis.
    pub const Y: Self = Self { arr: [0, 1, 0, 0] };

    /// positive unit vector on z-axis.
    pub const Z: Self = Self { arr: [0, 0, 1, 0] };

    /// positive unit vector on w-axis.
    pub const W: Self = Self { arr: [0, 0, 0, 1] };

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self { arr: [-1, 0, 0, 0] };

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self { arr: [0, -1, 0, 0] };

    /// negative unit vector on z-axis.
    pub const NEG_Z: Self = Self { arr: [0, 0, -1, 0] };

    /// negative unit vector on w-axis.
    pub const NEG_W: Self = Self { arr: [0, 0, 0, -1] };
}

impl VectorInt {
    /// Creates with given elements.
    #[inline]
    #[must_use]
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        unsafe {
            let arr = [x, y, z, w];
            Self { inner: vld1q_s32(arr.as_ptr()) }
        }
    }

    /// Fills all elements with the given values.
    #[inline]
    #[must_use]
    pub fn fill(v: i32) -> Self {
        unsafe { Self { inner: vdupq_n_s32(v) } }
    }

    /// Creates from a given array.
    #[inline]
    #[must_use]
    pub fn from_array(arr: [i32; 4]) -> Self {
        unsafe { Self { inner: vld1q_s32(arr.as_ptr()) } }
    }

    /// Stores the value in an array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> [i32; 4] {
        let mut arr = [0; 4];
        unsafe { vst1q_s32(arr.as_mut_ptr(), self.inner) };
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
    pub fn from_slice(slice: &[i32]) -> Self {
        #[cfg(feature = "use-assertion")]
        assert!(slice.len() >= 4, "The given array slice has less than four elements!");
        unsafe { Self { inner: vld1q_s32(slice.as_ptr()) } }
    }

    /// Loads a value from a given `Integer2`.
    #[inline]
    #[must_use]
    pub fn load_int2(val: Integer2) -> Self {
        Self::load_int4(val.into())
    }

    /// Stores the value in a `Integer2`.
    #[inline]
    #[must_use]
    pub fn store_int2(self) -> Integer2 {
        self.store_int4().xy()
    }

    /// Loads a value from a given `Integer3`.
    #[inline]
    #[must_use]
    pub fn load_int3(val: Integer3) -> Self {
        Self::load_int4(val.into())
    }

    /// Stores the value in a `Integer3`.
    #[inline]
    #[must_use]
    pub fn store_int3(self) -> Integer3 {
        self.store_int4().xyz()
    }

    /// Loads a value from a given `Integer4`.
    #[inline]
    #[must_use]
    pub fn load_int4(val: Integer4) -> Self {
        unsafe {
            Self { inner: vld1q_s32(&val as *const _ as *const i32) }
        }
    }

    /// Stores the value in a `Integer4`.
    #[inline]
    #[must_use]
    pub fn store_int4(self) -> Integer4 {
        unsafe {
            let mut val = Integer4::default();
            vst1q_s32(&mut val as *mut _ as *mut i32, self.inner);
            val
        }
    }

    /// Loads a value from a given `UInteger2`.
    #[inline]
    #[must_use]
    pub fn load_uint2(val: UInteger2) -> Self {
        Self::load_uint4(val.into())
    }

    /// Stores the value in a `Integer2`.
    #[inline]
    #[must_use]
    pub fn store_uint2(self) -> UInteger2 {
        self.store_uint4().xy()
    }

    /// Loads a value from a given `Integer3`.
    #[inline]
    #[must_use]
    pub fn load_uint3(val: UInteger3) -> Self {
        Self::load_uint4(val.into())
    }

    /// Stores the value in a `Integer3`.
    #[inline]
    #[must_use]
    pub fn store_uint3(self) -> UInteger3 {
        self.store_uint4().xyz()
    }

    /// Loads a value from a given `Integer4`.
    #[inline]
    #[must_use]
    pub fn load_uint4(val: UInteger4) -> Self {
        unsafe {
            Self { inner: vld1q_s32(&val as *const _ as *const i32) }
        }
    }

    /// Stores the value in a `Integer4`.
    #[inline]
    #[must_use]
    pub fn store_uint4(self) -> UInteger4 {
        unsafe {
            let mut val = UInteger4::default();
            vst1q_s32(&mut val as *mut _ as *mut i32, self.inner);
            val
        }
    }
}

impl VectorInt {
    /// Get the `x` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_x(&self) -> i32 {
        unsafe { vgetq_lane_s32::<0b00>(self.inner) }
    }
    
    /// Set the `x` element of a vector.
    #[inline]
    pub fn set_x(&mut self, v: i32) {
        unsafe { self.inner = vsetq_lane_s32::<0b00>(v, self.inner) }
    }
    
    /// Get the `y element of a vector.
    #[inline]
    #[must_use]
    pub fn get_y(&self) -> i32 {
        unsafe { vgetq_lane_s32::<0b01>(self.inner) }
    }
    
    /// Set the `y` element of a vector.
    #[inline]
    pub fn set_y(&mut self, v: i32) {
        unsafe { self.inner = vsetq_lane_s32::<0b01>(v, self.inner) }
    }
    
    /// Get the `z` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_z(&self) -> i32 {
        unsafe { vgetq_lane_s32::<0b10>(self.inner) }
    }
    
    /// Set the `z`` element of a vector.
    #[inline]
    pub fn set_z(&mut self, v: i32) {
        unsafe { self.inner = vsetq_lane_s32::<0b10>(v, self.inner) }
    }
    
    /// Get the `w` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_w(&self) -> i32 {
        unsafe { vgetq_lane_s32::<0b11>(self.inner) }
    }
    
    /// Set the `w` element of a vector.
    #[inline]
    pub fn set_w(&mut self, v: i32) {
        unsafe { self.inner = vsetq_lane_s32::<0b11>(v, self.inner) }
    }

    /// Takes the samller of the elements of the two vectors.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        unsafe { VectorInt { inner: vminq_s32(self.inner, rhs.inner) } }
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        unsafe { VectorInt { inner: vmaxq_s32(self.inner, rhs.inner) } }
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    #[must_use]
    pub fn lt(self, rhs: Self) -> VectorInt {
        unsafe { 
            let comp = vcltq_s32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are less than or equal.
    #[inline]
    #[must_use]
    pub fn le(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcleq_s32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    #[must_use]
    pub fn gt(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcgtq_s32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are greater than or equal.
    #[inline]
    #[must_use]
    pub fn ge(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcgeq_s32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are equal.
    #[inline]
    #[must_use]
    pub fn eq(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vceqq_s32(self.inner, rhs.inner);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are not equal.
    #[inline]
    #[must_use]
    pub fn ne(self, rhs: Self) -> VectorInt {
        !self.eq(rhs)
    }

    /// Absolute value on vector elements.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        unsafe { VectorInt { inner: vabsq_s32(self.inner) } }
    }

    /// Return a vector filled by adding all the elements of the vector.
    #[inline]
    #[must_use]
    pub fn sum(self) -> Self {
        unsafe {
            let sum = vpaddq_s32(self.inner, self.inner);
            return VectorInt { inner: vpaddq_s32(sum, sum) }
        }
    }

    /// Returns the sum of all elements in a vector.
    #[inline]
    #[must_use]
    pub fn sum_into(self) -> i32 {
        self.sum().get_x()
    }

    /// Returns a vector filled with the dot products of the two-element vectors.
    #[inline]
    #[must_use]
    pub fn vec2_dot(self, rhs: Self) -> Self {
        unsafe {
            let mul = self * rhs;
            let low = vget_low_s32(mul.inner);
            let sum = vpadd_s32(low, low);
            return VectorInt { inner: vcombine_s32(sum, sum) };
        }
    }

    /// Dot product of the two-element vectors.
    #[inline]
    #[must_use]
    pub fn vec2_dot_into(self, rhs: Self) -> i32 {
        self.vec2_dot(rhs).get_x()
    }

    /// Returns a vector filled with the dot products of the three-element vectors.
    #[inline]
    #[must_use]
    pub fn vec3_dot(self, rhs: Self) -> Self {
        unsafe {
            let mul = self * rhs;
            let low = vget_low_s32(mul.inner);
            let low = vpadd_s32(low, low);
            let high = vdup_laneq_s32::<0b10>(mul.inner);
            let sum = vadd_s32(low, high);
            return VectorInt { inner: vcombine_s32(sum, sum) };
        }
    }

    /// Dot product of the three-element vectors.
    #[inline]
    #[must_use]
    pub fn vec3_dot_into(self, rhs: Self) -> i32 {
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
    pub fn vec4_dot_into(self, rhs: Self) -> i32 {
        self.vec4_dot(rhs).get_x()
    }
}

impl Default for VectorInt {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<Vector> for VectorInt {
    #[inline]
    fn from(value: Vector) -> Self {
        unsafe { VectorInt { inner: vreinterpretq_s32_f32(value.inner) } }
    }
}

impl From<[i32; 4]> for VectorInt {
    #[inline]
    fn from(value: [i32; 4]) -> Self {
        Self::from_array(value)
    }
}

impl Into<[i32; 4]> for VectorInt {
    #[inline]
    fn into(self) -> [i32; 4] {
        self.into_array()
    }
}

impl From<Integer2> for VectorInt {
    #[inline]
    fn from(value: Integer2) -> Self {
        Self::load_int2(value)
    }
}

impl Into<Integer2> for VectorInt {
    #[inline]
    fn into(self) -> Integer2 {
        self.store_int2()
    }
}

impl From<Integer3> for VectorInt {
    #[inline]
    fn from(value: Integer3) -> Self {
        Self::load_int3(value)
    }
}

impl Into<Integer3> for VectorInt {
    #[inline]
    fn into(self) -> Integer3 {
        self.store_int3()
    }
}

impl From<Integer4> for VectorInt {
    #[inline]
    fn from(value: Integer4) -> Self {
        Self::load_int4(value)
    }
}

impl Into<Integer4> for VectorInt {
    #[inline]
    fn into(self) -> Integer4 {
        self.store_int4()
    }
}

impl From<UInteger2> for VectorInt {
    #[inline]
    fn from(value: UInteger2) -> Self {
        Self::load_uint2(value)
    }
}

impl Into<UInteger2> for VectorInt {
    #[inline]
    fn into(self) -> UInteger2 {
        self.store_uint2()
    }
}

impl From<UInteger3> for VectorInt {
    #[inline]
    fn from(value: UInteger3) -> Self {
        Self::load_uint3(value)
    }
}

impl Into<UInteger3> for VectorInt {
    #[inline]
    fn into(self) -> UInteger3 {
        self.store_uint3()
    }
}

impl From<UInteger4> for VectorInt {
    #[inline]
    fn from(value: UInteger4) -> Self {
        Self::load_uint4(value)
    }
}

impl Into<UInteger4> for VectorInt {
    #[inline]
    fn into(self) -> UInteger4 {
        self.store_uint4()
    }
}

impl fmt::Debug for VectorInt {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(VectorInt))
            .field(unsafe { &self.inner })
            .finish()
    }
}

impl ops::Add<VectorInt> for i32 {
    type Output = VectorInt;
    /// Adds a scalar value to each element of a vector.
    #[inline]
    fn add(self, rhs: VectorInt) -> Self::Output {
        VectorInt::fill(self) + rhs
    }
}

impl ops::Add<i32> for VectorInt {
    type Output = VectorInt;
    /// Adds a scalar value to each element of a vector.
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        self + VectorInt::fill(rhs)
    }
}

impl ops::AddAssign<i32> for VectorInt {
    /// Adds a scalar value to each element of a vector.
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs
    }
}

impl ops::Add<VectorInt> for VectorInt {
    type Output = VectorInt;
    /// Adds two vectors.
    #[inline]
    fn add(self, rhs: VectorInt) -> Self::Output {
        unsafe { VectorInt { inner: vaddq_s32(self.inner, rhs.inner) } }
    }
}

impl ops::AddAssign<VectorInt> for VectorInt {
    /// Adds two vectors.
    #[inline]
    fn add_assign(&mut self, rhs: VectorInt) {
        *self = *self + rhs
    }
}

impl ops::Sub<VectorInt> for i32 {
    type Output = VectorInt;
    #[inline]
    fn sub(self, rhs: VectorInt) -> Self::Output {
        VectorInt::fill(self) - rhs
    }
}

impl ops::Sub<i32> for VectorInt {
    type Output = VectorInt;
    /// Subtracts a scalar value to each element of a vector.
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        self - VectorInt::fill(rhs)
    }
}

impl ops::SubAssign<i32> for VectorInt {
    /// Subtracts a scalar value to each element of a vector.
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs
    }
}

impl ops::Sub<VectorInt> for VectorInt {
    type Output = VectorInt;
    /// Subtracts two vectors.
    #[inline]
    fn sub(self, rhs: VectorInt) -> Self::Output {
        unsafe { VectorInt { inner: vsubq_s32(self.inner, rhs.inner) } }
    }
}

impl ops::SubAssign<VectorInt> for VectorInt {
    /// Subtracts two vectors.
    #[inline]
    fn sub_assign(&mut self, rhs: VectorInt) {
        *self = *self - rhs
    }
}

impl ops::Neg for VectorInt {
    type Output = VectorInt;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        unsafe { VectorInt { inner: vnegq_s32(self.inner) } }
    }
}

impl ops::Mul<VectorInt> for i32 {
    type Output = VectorInt;
    /// Multiplies each element of a vector by a scalar value.
    #[inline]
    fn mul(self, rhs: VectorInt) -> Self::Output {
        VectorInt::fill(self) * rhs
    }
}

impl ops::Mul<i32> for VectorInt {
    type Output = VectorInt;
    /// Multiplies each element of a vector by a scalar value.
    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        self * VectorInt::fill(rhs)
    }
}

impl ops::MulAssign<i32> for VectorInt {
    /// Multiplies each element of a vector by a scalar value.
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs
    }
}

impl ops::Mul<VectorInt> for VectorInt {
    type Output = VectorInt;
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul(self, rhs: VectorInt) -> Self::Output {
        unsafe { VectorInt { inner: vmulq_s32(self.inner, rhs.inner) } }
    }
}

impl ops::MulAssign<VectorInt> for VectorInt {
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul_assign(&mut self, rhs: VectorInt) {
        *self = *self * rhs
    }
}

impl ops::Div<VectorInt> for i32 {
    type Output = VectorInt;
    /// Divides each element of a vector by a scalar value.
    #[inline]
    fn div(self, rhs: VectorInt) -> Self::Output {
        VectorInt::fill(self) / rhs
    }
}

impl ops::Div<i32> for VectorInt {
    type Output = VectorInt;
    /// Divides each element of a vector by a scalar value.
    #[inline]
    fn div(self, rhs: i32) -> Self::Output {
        self / VectorInt::fill(rhs)
    }
}

impl ops::DivAssign<i32> for VectorInt {
    /// Divides each element of a vector by a scalar value.
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs
    }
}

impl ops::Div<VectorInt> for VectorInt {
    type Output = VectorInt;
    /// Element-wise division of two vectors.
    #[inline]
    fn div(self, rhs: VectorInt) -> Self::Output {
        unsafe { 
            let a = vcvtq_f32_s32(self.inner);
            let b = vcvtq_f32_s32(rhs.inner);
            let result = vdivq_f32(a, b);
            return VectorInt { inner: vcvtq_s32_f32(result) }
        }
    }
}

impl ops::DivAssign<VectorInt> for VectorInt {
    /// Element-wise division of two vectors.
    #[inline]
    fn div_assign(&mut self, rhs: VectorInt) {
        *self = *self / rhs
    }
}

impl ops::BitAnd<VectorInt> for VectorInt {
    type Output = VectorInt;
    /// Element-wise bit `AND` operation of two vectors.
    #[inline]
    fn bitand(self, rhs: VectorInt) -> Self::Output {
        unsafe { VectorInt { inner: vandq_s32(self.inner, rhs.inner) } }
    }
}

impl ops::BitAndAssign<VectorInt> for VectorInt {
    /// Element-wise bit `AND` operation of two vectors.
    #[inline]
    fn bitand_assign(&mut self, rhs: VectorInt) {
        *self = *self & rhs
    }
}

impl ops::BitOr<VectorInt> for VectorInt {
    type Output = VectorInt;
    /// Element-wise bit `OR` operation of two vectors.
    #[inline]
    fn bitor(self, rhs: VectorInt) -> Self::Output {
        unsafe { VectorInt { inner: vorrq_s32(self.inner, rhs.inner) } }
    }
}

impl ops::BitOrAssign<VectorInt> for VectorInt {
    /// Element-wise bit `OR` operation of two vectors.
    #[inline]
    fn bitor_assign(&mut self, rhs: VectorInt) {
        *self = *self | rhs
    }
}

impl ops::BitXor<VectorInt> for VectorInt {
    type Output = VectorInt;
    /// Element-wise bit `XOR` operation of two vectors.
    #[inline]
    fn bitxor(self, rhs: VectorInt) -> Self::Output {
        unsafe { VectorInt { inner: veorq_s32(self.inner, rhs.inner) } }
    }
}

impl ops::BitXorAssign<VectorInt> for VectorInt {
    /// Element-wise bit `XOR` operation of two vectors.
    #[inline]
    fn bitxor_assign(&mut self, rhs: VectorInt) {
        *self = *self ^ rhs
    }
}

impl ops::Not for VectorInt {
    type Output = VectorInt;
    /// Element-wise bit `NOT` operation of a vector.
    #[inline]
    fn not(self) -> Self::Output {
        unsafe { VectorInt { inner: vmvnq_s32(self.inner) } }
    }
}
