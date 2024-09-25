use core::fmt;
use core::ops;
use crate::{
    Vector, 
    Integer2, Integer3, Integer4, 
    UInteger2, UInteger3, UInteger4, 
};



/// This is a vector data type that uses the `Scalar` instruction.
/// 
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VectorInt {
    pub(crate) arr: [i32; 4]
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
        Self { arr: [x, y, z, w] }
    }

    /// Fills all elements with the given values.
    #[inline]
    #[must_use]
    pub fn fill(v: i32) -> Self {
        Self { arr: [v; 4] }
    }

    /// Creates from a given array.
    #[inline]
    #[must_use]
    pub fn from_array(arr: [i32; 4]) -> Self {
        Self { arr }
    }

    /// Stores the value in an array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> [i32; 4] {
        self.arr
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
        let mut arr = [0; 4];
        for (i, e) in arr.iter_mut().enumerate() {
            *e = slice[i]
        }
        Self { arr }
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
        Self { arr: val.to_array() }
    }

    /// Stores the value in a `Integer4`.
    #[inline]
    #[must_use]
    pub fn store_int4(self) -> Integer4 {
        Integer4::from_array(self.into_array())
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
        let val = val.to_array();
        let mut arr = [0; 4];
        for (i, e) in arr.iter_mut().enumerate() {
            *e = unsafe { core::mem::transmute_copy(&val[i]) }
        }
        Self { arr }
    }

    /// Stores the value in a `Integer4`.
    #[inline]
    #[must_use]
    pub fn store_uint4(self) -> UInteger4 {
        UInteger4::from_slice(
            &self.into_array().map(|v| unsafe { core::mem::transmute_copy(&v) })
        )
    }
}

impl VectorInt {
    /// Get the `x` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_x(&self) -> i32 {
        self.arr[0]
    }

    /// Set the `x` element of a vector.
    #[inline]
    pub fn set_x(&mut self, v: i32) {
        self.arr[0] = v
    }

    /// Get the `y element of a vector.
    #[inline]
    #[must_use]
    pub fn get_y(&self) -> i32 {
        self.arr[1]
    }

    /// Set the `y` element of a vector.
    #[inline]
    pub fn set_y(&mut self, v: i32) {
        self.arr[1] = v
    }

    /// Get the `z` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_z(&self) -> i32 {
        self.arr[2]
    }

    /// Set the `z` element of a vector.
    #[inline]
    pub fn set_z(&mut self, v: i32) {
        self.arr[2] = v
    }

    /// Get the `w` element of a vector.
    #[inline]
    #[must_use]
    pub fn get_w(&self) -> i32 {
        self.arr[3]
    }

    /// Set the `w` element of a vector.
    #[inline]
    pub fn set_w(&mut self, v: i32) {
        self.arr[3] = v
    }

    /// Takes the samller of the elements of the two vectors.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            arr: [
                self.get_x().min(rhs.get_x()), 
                self.get_y().min(rhs.get_y()), 
                self.get_z().min(rhs.get_z()), 
                self.get_w().min(rhs.get_w()) 
            ]
        }
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            arr: [
                self.get_x().max(rhs.get_x()), 
                self.get_y().max(rhs.get_y()), 
                self.get_z().max(rhs.get_z()), 
                self.get_w().max(rhs.get_w()) 
            ]
        }
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    #[must_use]
    pub fn lt(self, rhs: Self) -> VectorInt {
        VectorInt::new(
            self.get_x().lt(&rhs.get_x()).then(|| -1).unwrap_or(0), 
            self.get_y().lt(&rhs.get_y()).then(|| -1).unwrap_or(0), 
            self.get_z().lt(&rhs.get_z()).then(|| -1).unwrap_or(0), 
            self.get_w().lt(&rhs.get_w()).then(|| -1).unwrap_or(0) 
        )
    }

    /// Checks if the elements of two vectors are less than or eqaul.
    #[inline]
    #[must_use]
    pub fn le(self, rhs: Self) -> VectorInt { 
        VectorInt::new(
            self.get_x().le(&rhs.get_x()).then(|| -1).unwrap_or(0), 
            self.get_y().le(&rhs.get_y()).then(|| -1).unwrap_or(0), 
            self.get_z().le(&rhs.get_z()).then(|| -1).unwrap_or(0), 
            self.get_w().le(&rhs.get_w()).then(|| -1).unwrap_or(0) 
        )
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    #[must_use]
    pub fn gt(self, rhs: Self) -> VectorInt {
        VectorInt::new(
            self.get_x().gt(&rhs.get_x()).then(|| -1).unwrap_or(0), 
            self.get_y().gt(&rhs.get_y()).then(|| -1).unwrap_or(0), 
            self.get_z().gt(&rhs.get_z()).then(|| -1).unwrap_or(0), 
            self.get_w().gt(&rhs.get_w()).then(|| -1).unwrap_or(0) 
        )
    }

    /// Checks if the elements of two vectors are greater than or eqaul.
    #[inline]
    #[must_use]
    pub fn ge(self, rhs: Self) -> VectorInt {
        VectorInt::new(
            self.get_x().ge(&rhs.get_x()).then(|| -1).unwrap_or(0), 
            self.get_y().ge(&rhs.get_y()).then(|| -1).unwrap_or(0), 
            self.get_z().ge(&rhs.get_z()).then(|| -1).unwrap_or(0), 
            self.get_w().ge(&rhs.get_w()).then(|| -1).unwrap_or(0) 
        )
    }

    /// Checks if the elements of two vectors are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    #[must_use]
    pub fn eq(self, rhs: Self) -> VectorInt {
        VectorInt::new(
            self.get_x().eq(&rhs.get_x()).then(|| -1).unwrap_or(0), 
            self.get_y().eq(&rhs.get_y()).then(|| -1).unwrap_or(0), 
            self.get_z().eq(&rhs.get_z()).then(|| -1).unwrap_or(0), 
            self.get_w().eq(&rhs.get_w()).then(|| -1).unwrap_or(0) 
        )
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
        VectorInt { arr: [
            self.get_x().abs(), 
            self.get_y().abs(), 
            self.get_z().abs(), 
            self.get_w().abs() 
        ] }
    }
    
    /// Return a vector filled by adding all the elements of the vector.
    #[inline]
    #[must_use]
    pub fn sum(self) -> Self {
        Self { arr: [self.arr.into_iter().sum(); 4] }
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
        const MASK_XY: VectorInt = VectorInt { arr: [1, 1, 0, 0] };
        (self * rhs * MASK_XY).sum()
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
        const MASK_XYZ: VectorInt = VectorInt { arr: [1, 1, 1, 0] };
        (self * rhs * MASK_XYZ).sum()
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
        let val = value.into_array();
        let mut arr = [0; 4];
        for (i, e) in arr.iter_mut().enumerate() {
            *e = unsafe { core::mem::transmute_copy(&val[i]) }
        }
        Self { arr }
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
            .field(&self.arr)
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
        Self { arr: [
            self.get_x() + rhs.get_x(), 
            self.get_y() + rhs.get_y(), 
            self.get_z() + rhs.get_z(), 
            self.get_w() + rhs.get_w() 
        ] }
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
        Self { arr: [
            self.get_x() - rhs.get_x(), 
            self.get_y() - rhs.get_y(), 
            self.get_z() - rhs.get_z(), 
            self.get_w() - rhs.get_w() 
        ] }
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
        Self { arr: [
            -self.get_x(), 
            -self.get_y(), 
            -self.get_z(), 
            -self.get_w() 
        ] }
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
        Self { arr: [
            self.get_x() * rhs.get_x(), 
            self.get_y() * rhs.get_y(), 
            self.get_z() * rhs.get_z(), 
            self.get_w() * rhs.get_w() 
        ] }
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
        Self { arr: [
            self.get_x() / rhs.get_x(), 
            self.get_y() / rhs.get_y(), 
            self.get_z() / rhs.get_z(), 
            self.get_w() / rhs.get_w() 
        ] }
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
        Self { arr: [
            self.get_x() & rhs.get_x(), 
            self.get_y() & rhs.get_y(), 
            self.get_z() & rhs.get_z(), 
            self.get_w() & rhs.get_w() 
        ] }
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
        Self { arr: [
            self.get_x() | rhs.get_x(), 
            self.get_y() | rhs.get_y(), 
            self.get_z() | rhs.get_z(), 
            self.get_w() | rhs.get_w() 
        ] }
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
        Self { arr: [
            self.get_x() ^ rhs.get_x(), 
            self.get_y() ^ rhs.get_y(), 
            self.get_z() ^ rhs.get_z(), 
            self.get_w() ^ rhs.get_w() 
        ] }
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
        Self { arr: [
            self.get_x(), 
            self.get_y(), 
            self.get_z(), 
            self.get_w() 
        ] }
    }
}
