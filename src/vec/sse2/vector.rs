use core::fmt;
use core::ops;

#[cfg(target_pointer_width = "32")]
use core::arch::x86::*;

#[cfg(target_pointer_width = "64")]
use core::arch::x86_64::*;

use crate::{ Quaternion, VectorInt, Float2, Float3, Float4 };



/// This is a vector data type that uses the `SIMD` instruction.
/// 
/// Using the `sse2` instruction.
/// 
#[repr(C)]
#[derive(Clone, Copy)]
pub union Vector {
    /// member variables for constant variables.
    arr: [f32; 4], 

    pub(crate) inner: __m128, 
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
            Self { inner: _mm_loadu_ps(arr.as_ptr()) }
        }
    }

    /// Fills all elements with the given values.
    #[inline]
    #[must_use]
    pub fn fill(v: f32) -> Self {
        unsafe { Self { inner: _mm_set1_ps(v) } }
    }

    /// Creates from a given array.
    #[inline]
    #[must_use]
    pub fn from_array(arr: [f32; 4]) -> Self {
        unsafe { Self { inner: _mm_loadu_ps(arr.as_ptr()) } }
    }

    /// Stores the value in an array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> [f32; 4] {
        let mut arr = [0.0; 4];
        unsafe { _mm_storeu_ps(arr.as_mut_ptr(), self.inner) };
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
        unsafe { Self { inner: _mm_loadu_ps(slice.as_ptr()) } }
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
            Self { inner: _mm_loadu_ps(&val as *const _ as *const f32) }
        }
    }

    /// Stores the value in a `Float4`.
    #[inline]
    #[must_use]
    pub fn store_float4(self) -> Float4 {
        unsafe {
            let mut val = Float4::default();
            _mm_storeu_ps(&mut val as *mut _ as *mut f32, self.inner);
            val
        }
    }
}

impl Vector {
    /// Get the `x` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_x(&self) -> f32 {
        unsafe { _mm_cvtss_f32(self.inner) }
    }

    /// Set the `x` element of a vector.
    #[inline]
    pub fn set_x(&mut self, v: f32) {
        unsafe {
            let x = _mm_set_ss(v);
            self.inner = _mm_move_ss(self.inner, x)
        }
    }

    /// Get the `y element of a vector.
    #[inline]
    #[must_use]
    pub fn get_y(&self) -> f32 {
        unsafe { 
            let temp = _mm_shuffle_ps::<0b_01_01_01_01>(self.inner, self.inner);
            _mm_cvtss_f32(temp)
        }
    }

    /// Set the `y` element of a vector.
    #[inline]
    pub fn set_y(&mut self, v: f32) {
        unsafe {
            let v = _mm_set_ss(v);
            self.inner = match is_x86_feature_detected!("sse4.1") {
                true => _mm_insert_ps::<0x10>(self.inner, v), 
                false => {
                    let yxzw = _mm_shuffle_ps::<0b_11_10_00_01>(self.inner, self.inner);
                    let vxzw = _mm_move_ss(yxzw, v);
                    _mm_shuffle_ps::<0b_11_10_00_01>(vxzw, vxzw)
                }
            };
        }
    }

    /// Get the `z` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_z(&self) -> f32 {
        unsafe { 
            let temp = _mm_shuffle_ps::<0b_10_10_10_10>(self.inner, self.inner);
            _mm_cvtss_f32(temp)
        }
    }

    /// Set the `z` element of a vector.
    #[inline]
    pub fn set_z(&mut self, v: f32) {
        unsafe {
            let v = _mm_set_ss(v);
            self.inner = match is_x86_feature_detected!("sse4.1") {
                true => _mm_insert_ps::<0x20>(self.inner, v), 
                false => {
                    let zwxy = _mm_shuffle_ps::<0b_01_00_11_10>(self.inner, self.inner);
                    let vwxy = _mm_move_ss(zwxy, v);
                    _mm_shuffle_ps::<0b_01_00_11_10>(vwxy, vwxy)
                }
            }
        }
    }

    /// Get the `w` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_w(&self) -> f32 {
        unsafe { 
            let temp = _mm_shuffle_ps::<0b_11_11_11_11>(self.inner, self.inner);
            _mm_cvtss_f32(temp)
        }
    }

    /// Set the `w` element of a vector.
    #[inline]
    pub fn set_w(&mut self, v: f32) {
        unsafe {
            let v = _mm_set_ss(v);
            self.inner = match is_x86_feature_detected!("sse4.1") {
                true => _mm_insert_ps::<0x30>(self.inner, v), 
                false => {
                    let wyzx = _mm_shuffle_ps::<0b_00_10_01_11>(self.inner, self.inner);
                    let vyzx = _mm_move_ss(wyzx, v);
                    _mm_shuffle_ps::<0b_00_10_01_11>(vyzx, vyzx)
                }
            }
        }
    }

    /// Takes the samller of the elements of the two vectors.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        unsafe { Vector { inner: _mm_min_ps(self.inner, rhs.inner) } }
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        unsafe { Vector { inner: _mm_max_ps(self.inner, rhs.inner) } }
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    #[must_use]
    pub fn lt(self, rhs: Self) -> VectorInt {
        unsafe { 
            let comp = _mm_cmplt_ps(self.inner, rhs.inner);
            let cast = _mm_castps_si128(comp);
            return VectorInt { inner: cast }; 
        }
    }

    /// Checks if the elements of two vectors are less than or eqaul.
    #[inline]
    #[must_use]
    pub fn le(self, rhs: Self) -> VectorInt { 
        self.lt(rhs) | self.eq(rhs)
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    #[must_use]
    pub fn gt(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = _mm_cmpgt_ps(self.inner, rhs.inner);
            let cast = _mm_castps_si128(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are greater than or eqaul.
    #[inline]
    #[must_use]
    pub fn ge(self, rhs: Self) -> VectorInt {
        self.gt(rhs) | self.eq(rhs)
    }

    /// Checks if the elements of two vectors are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn eq(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = _mm_cmpeq_ps(self.inner, rhs.inner);
            let cast = _mm_castps_si128(comp);
            return VectorInt { inner: cast };
        }
    }

    /// Checks if the elements of two vectors are not eqaul.
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
        self.max(-self)
    }
    
    /// Return a vector filled by adding all the elements of the vector.
    #[inline]
    #[must_use]
    pub fn sum(self) -> Self {
        unsafe {
            let sum = match is_x86_feature_detected!("sse3") {
                true => {
                    let temp = _mm_hadd_ps(self.inner, self.inner);
                    _mm_hadd_ps(temp, temp)
                }, 
                false => {
                    let low = _mm_shuffle_ps::<0b_01_00_01_00>(self.inner, self.inner);
                    let high = _mm_shuffle_ps::<0b_11_10_11_10>(self.inner, self.inner);
                    let low = _mm_add_ps(low, high);
                    let high = _mm_shuffle_ps::<0b_10_11_00_01>(low, low);
                    _mm_add_ps(low, high)
                }
            };
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
    pub fn vec2_dot(self, rhs: Self) -> Vector {
        unsafe {
            match is_x86_feature_detected!("sse4.1") {
                true => Vector { inner: _mm_dp_ps(self.inner, rhs.inner, 0x3F) },
                false => {
                    const MASK_XY: Vector = Vector { arr: [1.0, 1.0, 0.0, 0.0] };
                    (self * rhs * MASK_XY).sum()
                }
            }
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
    pub fn vec3_dot(self, rhs: Self) -> Vector {
        unsafe {
            match is_x86_feature_detected!("sse4.1") {
                true => Vector { inner: _mm_dp_ps(self.inner, rhs.inner, 0x7F) }, 
                false => {
                    const MASK_XYZ: Vector = Vector { arr: [1.0, 1.0, 1.0, 0.0] };
                    (self * rhs * MASK_XYZ).sum()
                }
            }
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
    pub fn vec4_dot(self, rhs: Self) -> Vector {
        unsafe { 
            match is_x86_feature_detected!("sse4.1") {
                true => Vector { inner: _mm_dp_ps(self.inner, rhs.inner, 0xFF) }, 
                false => (self * rhs).sum()
            }
        }
    }

    /// Dot product of the four-element vectors.
    #[inline]
    #[must_use]
    pub fn vec4_dot_into(self, rhs: Self) -> f32 {
        self.vec4_dot(rhs).get_x()
    }

    /// Cross product of two three-element vectors.
    #[must_use]
    pub fn vec3_cross(self, rhs: Self) -> Vector {
        const MASK_XYZ: Vector = Vector { arr: [1.0, 1.0, 1.0, 0.0] };
        unsafe {
            let ly_lz_lx = _mm_shuffle_ps::<0b_00_00_10_01>(self.inner, self.inner);
            let lz_lx_ly = _mm_shuffle_ps::<0b_00_01_00_10>(self.inner, self.inner);
            let rz_rx_ry = _mm_shuffle_ps::<0b_00_01_00_10>(rhs.inner, rhs.inner);
            let ry_rz_rx = _mm_shuffle_ps::<0b_00_00_10_01>(rhs.inner, rhs.inner);
            
            let a = _mm_mul_ps(ly_lz_lx, rz_rx_ry);
            let b = _mm_mul_ps(lz_lx_ly, ry_rz_rx);
            Vector { inner: _mm_sub_ps(a, b) } * MASK_XYZ
        }
    }

    /// Length squared of a two-element vector.
    #[inline]
    #[must_use]
    pub fn vec2_len_sq(self) -> f32 {
        self.vec2_dot_into(self)
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
        self.vec3_dot_into(self)
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
        self.vec4_dot_into(self)
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
        (self.vec2_len() - 1.0).abs() <= f32::EPSILON
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
        // If `sse4.1`` is not supported, hardware acceleration is not used.
        unsafe {
            match is_x86_feature_detected!("sse4.1") {
                true => Vector { inner: _mm_floor_ps(self.inner) }, 
                false => {
                    let mut arr = self.into_array();
                    for e in arr.iter_mut() {
                        *e = e.floor();
                    }
                    Vector::from_array(arr)
                }
            }
        }
    }

    /// Returns a vector containing the smallest integer greater than or equal to the number for each element of the vector.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        // If `sse4.1`` is not supported, hardware acceleration is not used.
        unsafe {
            match is_x86_feature_detected!("sse4.1") {
                true => Vector { inner: _mm_ceil_ps(self.inner) }, 
                false => {
                    let mut arr = self.into_array();
                    for e in arr.iter_mut() {
                        *e = e.ceil();
                    }
                    Vector::from_array(arr)
                }
            }
        }
    }

    /// Returns a vector with the numbers for each element of the vector rounded down to the nearest decimal.
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        // If `sse4.1`` is not supported, hardware acceleration is not used.
        unsafe {
            match is_x86_feature_detected!("sse4.1") {
                true => {
                    const FLAG: i32 = _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC;
                    Vector { inner: _mm_round_ps::<FLAG>(self.inner) }
                }, 
                false => {
                    let mut arr = self.into_array();
                    for e in arr.iter_mut() {
                        *e = e.round();
                    }
                    Vector::from_array(arr)
                }
            }
        }
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
        unsafe { Vector { inner: _mm_add_ps(self.inner, rhs.inner) } }
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
        unsafe { Vector { inner: _mm_sub_ps(self.inner, rhs.inner) } }
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
        unsafe { Vector { inner: _mm_sub_ps(_mm_setzero_ps(), self.inner) } }
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
        unsafe { Vector { inner: _mm_mul_ps(self.inner, rhs.inner) } }
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
        unsafe { Vector { inner: _mm_div_ps(self.inner, rhs.inner) } }
    }
}

impl ops::DivAssign<Vector> for Vector {
    /// Element-wise division of two vectors.
    #[inline]
    fn div_assign(&mut self, rhs: Vector) {
        *self = *self / rhs
    }
}
