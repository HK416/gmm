use crate::macros::impl_element3;
use crate::macros::impl_element3_op;
use super::bool3::Boolean3;
use super::float2::Float2;
use super::float4::Float4;



/// A structure that stores three-dimensional vector data.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0.0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1.0);

    /// All elements are negative one.
    pub const NEG_ONE: Self = Self::fill(-1.0);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);

    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);

    /// positive unit vector on z-axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);

    /// negative unit vector on z-axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    /// All elements are [`f32::MIN`].
    pub const MIN: Self = Self::fill(f32::MIN);

    /// All elements are [`f32::MAX`].
    pub const MAX: Self = Self::fill(f32::MAX);

    /// All elements are [`f32::NAN`].
    pub const NAN: Self = Self::fill(f32::NAN);

    /// All elements are [`f32::INFINITY`].
    pub const INFINITY: Self = Self::fill(f32::INFINITY);

    /// All elements are [`f32::NEG_INFINITY`].
    pub const NEG_INFINITY: Self = Self::fill(f32::NEG_INFINITY);

    /// Returns `true` if at least one element of the vector is [`f32::NAN`].
    #[inline]
    pub fn is_nan(&self) -> bool {
        Boolean3 {
            x: self.x.is_nan(),
            y: self.y.is_nan(),
            z: self.z.is_nan(),
        }.any()
    }

    /// Returns `true` if at least one element of the vector is [`f32::INFINITY`].
    #[inline]
    pub fn is_infinite(&self) -> bool {
        Boolean3 {
            x: self.x.is_infinite(),
            y: self.y.is_infinite(),
            z: self.z.is_infinite(),
        }.any()
    }
}

// Vector swizzle code implementation.
impl Float3 {
    #[inline]
    pub const fn xx(self) -> Float2 {
        Float2 { x: self.x, y: self.x }
    }

    #[inline]
    pub const fn xy(self) -> Float2 {
        Float2 { x: self.x, y: self.y }
    }

    #[inline]
    pub const fn xz(self) -> Float2 {
        Float2 { x: self.x, y: self.z }
    }

    #[inline]
    pub const fn yx(self) -> Float2 {
        Float2 { x: self.y, y: self.x }
    }

    #[inline]
    pub const fn yy(self) -> Float2 {
        Float2 { x: self.y, y: self.y }
    }

    #[inline]
    pub const fn yz(self) -> Float2 {
        Float2 { x: self.y, y: self.z }
    }

    #[inline]
    pub const fn zx(self) -> Float2 {
        Float2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(self) -> Float2 {
        Float2 { x: self.z, y: self.y }
    }

    #[inline]
    pub const fn zz(self) -> Float2 {
        Float2 { x: self.z, y: self.z }
    }

    #[inline]
    pub const fn xxx(self) -> Float3 {
        Float3 { x: self.x, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn xxy(self) -> Float3 {
        Float3 { x: self.x, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn xxz(self) -> Float3 {
        Float3 { x: self.x, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn xyx(self) -> Float3 {
        Float3 { x: self.x, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn xyy(self) -> Float3 {
        Float3 { x: self.x, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn xyz(self) -> Float3 {
        Float3 { x: self.x, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn xzx(self) -> Float3 {
        Float3 { x: self.x, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn xzy(self) -> Float3 {
        Float3 { x: self.x, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn xzz(self) -> Float3 {
        Float3 { x: self.x, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn yxx(self) -> Float3 {
        Float3 { x: self.y, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn yxy(self) -> Float3 {
        Float3 { x: self.y, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn yxz(self) -> Float3 {
        Float3 { x: self.y, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn yyx(self) -> Float3 {
        Float3 { x: self.y, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn yyy(self) -> Float3 {
        Float3 { x: self.y, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn yyz(self) -> Float3 {
        Float3 { x: self.y, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn yzx(self) -> Float3 {
        Float3 { x: self.y, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn yzy(self) -> Float3 {
        Float3 { x: self.y, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn yzz(self) -> Float3 {
        Float3 { x: self.y, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn zxx(self) -> Float3 {
        Float3 { x: self.z, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn zxy(self) -> Float3 {
        Float3 { x: self.z, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn zxz(self) -> Float3 {
        Float3 { x: self.z, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn zyx(self) -> Float3 {
        Float3 { x: self.z, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn zyy(self) -> Float3 {
        Float3 { x: self.z, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn zyz(self) -> Float3 {
        Float3 { x: self.z, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn zzx(self) -> Float3 {
        Float3 { x: self.z, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn zzy(self) -> Float3 {
        Float3 { x: self.z, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn zzz(self) -> Float3 {
        Float3 { x: self.z, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn xxxx(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xxxy(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xxxz(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xxyx(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xxyy(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xxyz(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xxzx(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xxzy(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xxzz(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.z, w: self.z }
    }
    
    #[inline]
    pub const fn xyxx(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xyxy(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xyxz(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xyyx(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xyyy(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xyyz(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xyzx(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xyzy(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xyzz(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xzxx(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xzxy(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xzxz(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xzyx(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xzyy(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xzyz(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xzzx(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xzzy(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xzzz(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yxxx(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yxxy(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yxxz(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yxyx(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yxyy(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yxzx(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yxzy(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yxzz(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yyxx(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yyxy(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yyxz(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yyyx(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yyyy(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yyyz(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yyzx(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yyzy(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yyzz(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn yzxx(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yzxy(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yzxz(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn yzyx(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yzyy(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn yzyz(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yzzx(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn yzzy(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn yzzz(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zxxx(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zxxy(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zxxz(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zxyx(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zxyy(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zxyz(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zxzx(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zxzy(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zxzz(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zyxx(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zyxy(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zyxz(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zyyx(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zyyy(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zyyz(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zyzx(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zyzy(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zyzz(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zzxx(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zzxy(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zzxz(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zzyx(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zzyy(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zzyz(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zzzx(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zzzy(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.z, w: self.y }
    }
    
    #[inline]
    pub const fn zzzz(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.z, w: self.z }
    }
}

impl_element3!(f32, Float3);

impl_element3_op!(f32, Float3);

impl Default for Float3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<Float2> for Float3 {
    #[inline]
    fn from(value: Float2) -> Self {
        Float3 { x: value.x, y: value.y, z: 0.0 }
    }
}

impl From<Float4> for Float3 {
    #[inline]
    fn from(value: Float4) -> Self {
        Float3 { x: value.x, y: value.y, z: value.z }
    }
}

impl core::ops::Neg for Float3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x, 
            y: -self.y, 
            z: -self.z, 
        }
    }
}
