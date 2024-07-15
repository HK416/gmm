use core::fmt;
use core::ops;
use core::arch::aarch64::*;
use super::VectorInt;
use crate::{ Float2, Float3, Float4 };



/// This is a vector data type that uses the `SIMD` instruction.
/// 
/// It is recommended not to use this data type as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vector(pub(crate) float32x4_t);

impl Vector {
    /// Takes the samller of the elements of the two vectors.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        unsafe { Vector(vminq_f32(*self, *rhs)) }
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        unsafe { Vector(vmaxq_f32(*self, *rhs)) }
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    pub fn lt(self, rhs: Self) -> VectorInt {
        unsafe { 
            let comp = vcltq_f32(*self, *rhs);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt(cast);
        }
    }

    /// Checks if the elements of two vectors are less than or eqaul.
    #[inline]
    pub fn le(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcleq_f32(*self, *rhs);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt(cast);
        }
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    pub fn gt(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcgtq_f32(*self, *rhs);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt(cast);
        }
    }

    /// Checks if the elements of two vectors are greater than or eqaul.
    #[inline]
    pub fn ge(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vcgeq_f32(*self, *rhs);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt(cast);
        }
    }

    /// Checks if the elements of two vectors are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn eq(self, rhs: Self) -> VectorInt {
        unsafe {
            let comp = vceqq_f32(*self, *rhs);
            let cast = vreinterpretq_s32_u32(comp);
            return VectorInt(cast);
        }
    }

    /// Checks if the elements of two vectors are not eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn ne(self, rhs: Self) -> VectorInt {
        !self.eq(rhs)
    }
}

impl Vector {
    /// Absolute value on vector elements.
    #[inline]
    pub fn abs(self) -> Self {
        unsafe { Vector(vabsq_f32(*self)) }
    }
    
    /// Return a vector filled by adding all the elements of the vector.
    #[inline]
    pub fn sum(self) -> Self {
        unsafe {
            let sum = vpaddq_f32(*self, *self);
            let sum = vpaddq_f32(sum, sum);
            return Vector(sum);
        }
    }

    /// Dot product of tow two-element vectors.
    #[inline]
    pub fn vec2_dot(self, rhs: Self) -> Vector {
        let mul = self * rhs;
        unsafe {
            let low = vget_low_f32(*mul);
            let sum = vpadd_f32(low, low);
            let sum = vcombine_f32(sum, sum);
            return Vector(sum);
        }
    }

    /// Dot product of tow three-element vectors.
    #[inline]
    pub fn vec3_dot(self, rhs: Self) -> Vector {
        let mul = self * rhs;
        unsafe {
            let low = vget_low_f32(*mul);
            let low = vpadd_f32(low, low);
            let high = vdup_laneq_f32::<0b10>(*mul);
            let sum = vadd_f32(low, high);
            return Vector(vcombine_f32(sum, sum));
        }
    }

    /// Dot product of two four-element vectors.
    #[inline]
    pub fn vec4_dot(self, rhs: Self) -> Vector {
        (self * rhs).sum()
    }

    /// Cross product of two three-element vectors.
    #[inline]
    pub fn vec3_cross(self, rhs: Self) -> Vector {
        const MASK_XYZ: [f32; 4] = [1.0, 1.0, 1.0, 0.0];
        unsafe {
            let lx_ly = vget_low_f32(*self);
            let lz_lw = vget_high_f32(*self);
            let ly_lz = vext_f32::<0b01>(lx_ly, lz_lw);
            let lz_lx = vext_f32::<0b01>(ly_lz, lx_ly);

            let rx_ry = vget_low_f32(*rhs);
            let rz_rw = vget_high_f32(*rhs);
            let ry_rz = vext_f32::<0b01>(rx_ry, rz_rw);
            let rz_rx = vext_f32::<0b01>(ry_rz, rx_ry);

            let ly_lz_lx = vcombine_f32(ly_lz, lx_ly);
            let lz_lx_ly = vcombine_f32(lz_lx, ly_lz);
            let rz_rx_ry = vcombine_f32(rz_rx, ry_rz);
            let ry_rz_rx = vcombine_f32(ry_rz, rx_ry);

            let a = vmulq_f32(ly_lz_lx, rz_rx_ry);
            let b = vmulq_f32(lz_lx_ly, ry_rz_rx);
            let result = vsubq_f32(a, b);
            
            let mask = vld1q_f32(&MASK_XYZ as *const _ as *const f32);
            let result = vmulq_f32(result, mask);

            return Vector(result);
        }
    }

    /// Length squared of a two-element vector.
    #[inline]
    pub fn vec2_len_sq(self) -> f32 {
        unsafe { vgetq_lane_f32::<0b00>(*self.vec2_dot(self)) }
    }

    /// Length of a two-element vector.
    #[inline]
    pub fn vec2_len(self) -> f32 {
        self.vec2_len_sq().sqrt()
    }

    /// Length squared of a three-element vector.
    #[inline]
    pub fn vec3_len_sq(self) -> f32 {
        unsafe { vgetq_lane_f32::<0b00>(*self.vec3_dot(self)) }
    }

    /// Length of a three-element vector.
    #[inline]
    pub fn vec3_len(self) -> f32 {
        self.vec3_len_sq().sqrt()
    }

    /// Length squared of a four-element vector.
    #[inline]
    pub fn vec4_len_sq(self) -> f32 {
        unsafe { vgetq_lane_f32::<0b00>(*self.vec4_dot(self)) }
    }

    /// Length of a four-element vector.
    #[inline]
    pub fn vec4_len(self) -> f32 {
        self.vec4_len_sq().sqrt()
    }

    /// Normalizes a two-element vector.
    /// If normalization fails, `None` is returned.
    #[inline]
    pub fn vec2_normalize(self) -> Option<Self> {
        const MASK_XY: Float4 = Float4::new(1.0, 1.0, 0.0, 0.0);
        let len = self.vec2_len();
        match len <= f32::EPSILON {
            false => {
                let mask: Vector = MASK_XY.into();
                let recip_len: Vector = Float4::fill(len.recip()).into();
                Some(self * recip_len * mask)
            },
            true => None,
        }
    }

    /// Normalizes a three-element vector.
    /// If normalization fails, `None` is returned.
    #[inline]
    pub fn vec3_normalize(self) -> Option<Self> {
        const MASK_XYZ: Float4 = Float4::new(1.0, 1.0, 1.0, 0.0);
        let len = self.vec3_len();
        match len <= f32::EPSILON {
            false => {
                let mask: Vector = MASK_XYZ.into();
                let recip_len: Vector = Float4::fill(len.recip()).into();
                Some(self * recip_len * mask)
            },
            true => None,
        }
    }

    /// Normalizes a four-element vector.
    /// If normalization fails, `None` is returned.
    #[inline]
    pub fn vec4_normalize(self) -> Option<Self> {
        let len = self.vec4_len();
        match len <= f32::EPSILON {
            false => {
                let recip_len: Vector = Float4::fill(len.recip()).into();
                Some(self * recip_len)
            },
            true => None,
        }
    }
}

impl From<[f32; 4]> for Vector {
    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self::from(Float4::from(value))
    }
}

impl Into<[f32; 4]> for Vector {
    #[inline]
    fn into(self) -> [f32; 4] {
        let value: Float4 = self.into();
        value.into()
    }
}

impl From<VectorInt> for Vector {
    #[inline]
    fn from(value: VectorInt) -> Self {
        unsafe { Vector(vreinterpretq_f32_s32(*value)) }
    }
}

impl From<Float2> for Vector {
    #[inline]
    fn from(value: Float2) -> Self {
        Self::from(Float4::from(value))
    }
}

impl Into<Float2> for Vector {
    #[inline]
    fn into(self) -> Float2 {
        let value: Float4 = self.into();
        return Float2::from(value);
    }
}

impl From<Float3> for Vector {
    #[inline]
    fn from(value: Float3) -> Self {
        Self::from(Float4::from(value))
    }
}

impl Into<Float3> for Vector {
    #[inline]
    fn into(self) -> Float3 {
        let value: Float4 = self.into();
        return Float3::from(value);
    }
}

impl From<Float4> for Vector {
    #[inline]
    fn from(value: Float4) -> Self {
        unsafe { Vector(vld1q_f32(&value as * const _ as *const f32)) }
    }
}

impl Into<Float4> for Vector {
    #[inline]
    fn into(self) -> Float4 {
        let mut value = Float4::default();
        unsafe { vst1q_f32(&mut value as *mut _ as *mut f32, self.0) };
        return value;
    }
}

impl ops::Deref for Vector {
    type Target = float32x4_t;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Vector {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ops::Add<Self> for Vector {
    type Output = Self;
    /// Adds two vectors.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Vector(vaddq_f32(self.0, rhs.0)) }
    }
}

impl ops::AddAssign<Self> for Vector {
    /// Adds two vectors. (assign)
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub<Self> for Vector {
    type Output = Self;
    /// Subtracts two vectors.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Vector(vsubq_f32(*self, *rhs)) }
    }
}

impl ops::SubAssign<Self> for Vector {
    /// Subtracts two vectors. (assign)
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl ops::Neg for Vector {
    type Output = Self;
    /// Nagative.
    #[inline]
    fn neg(self) -> Self::Output {
        unsafe { Vector(vnegq_f32(*self)) }
    }
}

impl ops::Mul<Self> for Vector {
    type Output = Self;
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Vector(vmulq_f32(*self, *rhs)) }
    }
}

impl ops::MulAssign<Self> for Vector {
    /// Element-wise multiplication of two vectors. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl ops::Div<Self> for Vector {
    type Output = Self;
    /// Element-wise division of two vectors.
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Vector(vdivq_f32(*self, *rhs)) }
    }
}

impl ops::DivAssign<Self> for Vector {
    /// Element-wise division of two vectors. (assign)
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl fmt::Debug for Vector {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vector))
            .field(&*self)
            .finish()
    }
}
