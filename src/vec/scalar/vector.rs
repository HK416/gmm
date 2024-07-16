use core::fmt;
use core::ops;
use crate::{ 
    VectorInt, 
    Boolean4, UInteger4, 
    Float2, Float3, Float4, 
};



/// This is a vector data type that uses the `SIMD` instruction.
/// 
/// Using the `scalar-math` feature disables the use of `SIMD` instructions.
/// 
/// It is recommended not to use this data type as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vector(pub(crate) Float4);

impl Vector {
    /// Takes the samller of the elements of the two vectors.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Float4 {
            x: self[0].min(rhs[0]), 
            y: self[1].min(rhs[1]), 
            z: self[2].min(rhs[2]), 
            w: self[3].min(rhs[3]) 
        }.into()
    }

    /// Takes the larger of the elements of the two vectors.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Float4 { 
            x: self[0].max(rhs[0]), 
            y: self[1].max(rhs[1]), 
            z: self[2].max(rhs[2]), 
            w: self[3].max(rhs[3]) 
        }.into()
    }

    /// Checks if the elements of two vectors are less.
    #[inline]
    pub fn lt(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].lt(&rhs[0]), 
            y: self[1].lt(&rhs[1]), 
            z: self[2].lt(&rhs[2]), 
            w: self[3].lt(&rhs[3]) 
        }).into()
    }

    /// Checks if the elements of two vectors are less than or eqaul.
    #[inline]
    pub fn le(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].le(&rhs[0]), 
            y: self[1].le(&rhs[1]), 
            z: self[2].le(&rhs[2]), 
            w: self[3].le(&rhs[3]) 
        }).into()
    }

    /// Checks if the elements of two vectors are greater.
    #[inline]
    pub fn gt(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].gt(&rhs[0]), 
            y: self[1].gt(&rhs[1]), 
            z: self[2].gt(&rhs[2]), 
            w: self[3].gt(&rhs[3]) 
        }).into()
    }

    /// Checks if the elements of two vectors are greater than or eqaul.
    #[inline]
    pub fn ge(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].ge(&rhs[0]), 
            y: self[1].ge(&rhs[1]), 
            z: self[2].ge(&rhs[2]), 
            w: self[3].ge(&rhs[3]) 
        }).into()
    }

    /// Checks if the elements of two vectors are eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn eq(self, rhs: Self) -> VectorInt {
        UInteger4::from(Boolean4 {
            x: self[0].eq(&rhs[0]), 
            y: self[1].eq(&rhs[1]), 
            z: self[2].eq(&rhs[2]), 
            w: self[3].eq(&rhs[3]) 
        }).into()
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
        Float4 {
            x: self[0].abs(), 
            y: self[1].abs(), 
            z: self[2].abs(), 
            w: self[3].abs() 
        }.into()
    }
    
    /// Return a vector filled by adding all the elements of the vector.
    #[inline]
    pub fn sum(self) -> Self {
        Float4 {
            x: self[0] + self[1] + self[2] + self[3], 
            y: self[0] + self[1] + self[2] + self[3], 
            z: self[0] + self[1] + self[2] + self[3], 
            w: self[0] + self[1] + self[2] + self[3] 
        }.into()
    }

    /// Dot product of tow two-element vectors.
    #[inline]
    pub fn vec2_dot(self, rhs: Self) -> Vector {
        let mul = self * rhs;
        Float4 {
            x: mul[0] + mul[1], 
            y: mul[0] + mul[1], 
            z: mul[0] + mul[1], 
            w: mul[0] + mul[1] 
        }.into()
    }

    /// Dot product of tow three-element vectors.
    #[inline]
    pub fn vec3_dot(self, rhs: Self) -> Vector {
        let mul = self * rhs;
        Float4 {
            x: mul[0] + mul[1] + mul[2], 
            y: mul[0] + mul[1] + mul[2], 
            z: mul[0] + mul[1] + mul[2], 
            w: mul[0] + mul[1] + mul[2] 
        }.into()
    }

    /// Dot product of two four-element vectors.
    #[inline]
    pub fn vec4_dot(self, rhs: Self) -> Vector {
        let mul = self * rhs;
        Float4 {
            x: mul[0] + mul[1] + mul[2] + mul[3], 
            y: mul[0] + mul[1] + mul[2] + mul[3], 
            z: mul[0] + mul[1] + mul[2] + mul[3], 
            w: mul[0] + mul[1] + mul[2] + mul[3] 
        }.into()
    }

    /// Cross product of two three-element vectors.
    #[inline]
    pub fn vec3_cross(self, rhs: Self) -> Vector {
        // self=[ax, ay, az], rhs=[bx, by, bz]
        // x: ay*bz - az*by
        // y: az*bx - ax*bz
        // z: ax*by - ay*bx
        // w: 0.0
        //
        Float4 {
            x: self[1] * rhs[2] - self[2] * rhs[1], 
            y: self[2] * rhs[0] - self[0] * rhs[2], 
            z: self[0] * rhs[1] - self[1] * rhs[0], 
            w: 0.0
        }.into()
    }

    /// Length squared of a two-element vector.
    #[inline]
    pub fn vec2_len_sq(self) -> f32 {
        self.vec2_dot(self).x
    }

    /// Length of a two-element vector.
    #[inline]
    pub fn vec2_len(self) -> f32 {
        self.vec2_len_sq().sqrt()
    }

    /// Length squared of a three-element vector.
    #[inline]
    pub fn vec3_len_sq(self) -> f32 {
        self.vec3_dot(self).x
    }

    /// Length of a three-element vector.
    #[inline]
    pub fn vec3_len(self) -> f32 {
        self.vec3_len_sq().sqrt()
    }

    /// Length squared of a four-element vector.
    #[inline]
    pub fn vec4_len_sq(self) -> f32 {
        self.vec4_dot(self).x
    }

    /// Length of a four-element vector.
    #[inline]
    pub fn vec4_len(self) -> f32 {
        self.vec4_len_sq().sqrt()
    }

    /// Returns `true` if it is a unit vector.
    #[inline]
    pub fn is_vec2_normalized(self) -> bool {
        (self.vec2_len_sq() - 1.0).abs() <= f32::EPSILON
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

    /// Returns `true` if it is a unit vector.
    #[inline]
    pub fn is_vec3_normalized(self) -> bool {
        (self.vec3_len_sq() - 1.0).abs() <= f32::EPSILON
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

    /// Returns `true` if it is a unit vector.
    #[inline]
    pub fn is_vec4_normalized(self) -> bool {
        (self.vec4_len_sq() - 1.0).abs() <= f32::EPSILON
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
        use core::mem;
        unsafe { 
            Self(Float4 { 
                x: mem::transmute(value[0]), 
                y: mem::transmute(value[1]), 
                z: mem::transmute(value[2]), 
                w: mem::transmute(value[3]) 
            })
        }
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
        Float2::from(*self)
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
        Float3::from(*self)
    }
}

impl From<Float4> for Vector {
    #[inline]
    fn from(value: Float4) -> Self {
        Self(value)
    }
}

impl Into<Float4> for Vector {
    #[inline]
    fn into(self) -> Float4 {
        *self
    }
}

impl ops::Deref for Vector {
    type Target = Float4;
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
        Vector(*self + *rhs)
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
        Vector(*self - *rhs)
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
        Vector(-*self)
    }
}

impl ops::Mul<Self> for Vector {
    type Output = Self;
    /// Element-wise multiplication of two vectors.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vector(*self * *rhs)
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
        Vector(*self / *rhs)
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
