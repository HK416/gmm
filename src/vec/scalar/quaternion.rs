use core::fmt;
use core::ops;
use crate::{ 
    Vector, VectorInt, 
    Boolean4, UInteger4, Float4 
};



/// This is a quaternion data type that uses the `SIMD` instruction.
/// 
/// Using the `scalar-math` feature disables the use of `SIMD` instructions.
/// 
/// It is recommended not to use this data types as a member of a structure.
/// 
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Quaternion(pub(crate) Float4);

impl Quaternion {
    /// Checks if the elements of two quaternions are eqaul.
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

    /// Checks if the elements of two quaternions are not eqaul.
    /// 
    /// This function does not use [`f32::EPSILON`].
    /// 
    #[inline]
    pub fn ne(self, rhs: Self) -> VectorInt {
        !self.eq(rhs)
    }
}

impl Quaternion {
    /// Dot product of two quaternions.
    #[inline]
    pub fn dot(self, rhs: Self) -> Vector {
        let mul = *self * *rhs;
        Float4 {
            x: mul[0] + mul[1] + mul[2] + mul[3], 
            y: mul[0] + mul[1] + mul[2] + mul[3], 
            z: mul[0] + mul[1] + mul[2] + mul[3], 
            w: mul[0] + mul[1] + mul[2] + mul[3] 
        }.into()
    }

    /// Length squared of a quaternion.
    #[inline]
    pub fn len_sq(self) -> f32 {
        self.dot(self).x
    }

    /// Length of a quaternion.
    #[inline]
    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
    }

    /// Normalizes a quaternion.
    /// If normalization fails, `None`is returned.
    #[inline]
    pub fn normalize(self) -> Option<Self> {
        let len = self.len();
        match len <= f32::EPSILON {
            false => Some(Quaternion(*self / len)), 
            true => None,
        }
    }

    /// Returns the conjugate of the quaternion.
    #[inline]
    pub fn conjugate(self) -> Self {
        Quaternion(Float4 { 
            x: -self.x, 
            y: -self.y, 
            z: -self.z, 
            w: self.w 
        })
    }

    /// Returns the inverse of the quaternion.
    /// If normalization fails, `None` is returned.
    #[inline]
    pub fn inverse(self) -> Option<Self> {
        self.normalize().map(|norm| {
            norm.conjugate()
        })
    }
}

impl ops::Deref for Quaternion {
    type Target = Float4;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Quaternion {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[f32; 4]> for Quaternion {
    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self::from(Float4::from(value))
    }
}

impl Into<[f32; 4]> for Quaternion {
    #[inline]
    fn into(self) -> [f32; 4] {
        let value: Float4 = self.into();
        value.into()
    }
}

impl From<Vector> for Quaternion {
    #[inline]
    fn from(value: Vector) -> Self {
        Quaternion(value.0)
    }
}

impl Into<Vector> for Quaternion {
    #[inline]
    fn into(self) -> Vector {
        Vector(*self)
    }
}

impl From<Float4> for Quaternion {
    #[inline]
    fn from(value: Float4) -> Self {
        Self(value)
    }
}

impl Into<Float4> for Quaternion {
    #[inline]
    fn into(self) -> Float4 {
        *self
    }
}

impl ops::Mul<Self> for Quaternion {
    type Output = Self;
    /// Multiplies two quaternions.
    fn mul(self, rhs: Self) -> Self::Output {
        // self: a, rhs: b
        // i: aw*bx + ax*bw + ay*bz - az*by
        // j: aw*by - ax*bz + ay*bw + az*bx
        // k: aw*bz + ax*by - ay*bx + az*bw
        // w: aw*bw - ax*bx - ay*by - az*bz
        //
        Float4 {
            x: self[3]*rhs[0] + self[0]*rhs[3] + self[1]*rhs[2] - self[2]*rhs[1], 
            y: self[3]*rhs[1] - self[0]*rhs[2] + self[1]*rhs[3] + self[2]*rhs[0], 
            z: self[3]*rhs[2] + self[0]*rhs[1] - self[1]*rhs[0] + self[2]*rhs[3], 
            w: self[3]*rhs[3] - self[0]*rhs[0] - self[1]*rhs[1] - self[2]*rhs[2] 
        }.into()
    }
}

impl ops::MulAssign<Self> for Quaternion {
    /// Multiplies two quaternions. (assign)
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl fmt::Debug for Quaternion {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Quaternion))
            .field(&*self)
            .finish()
    }
}
