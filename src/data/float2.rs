use crate::macros::impl_element2;
use crate::macros::impl_element2_op;
use super::bool2::Boolean2;
use super::float3::Float3;
use super::float4::Float4;





/// A structure that stores two-dimensional vector data.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0.0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1.0);

    /// All elements are negative one.
    pub const NEG_ONE: Self = Self::fill(-1.0);
    
    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1.0, 0.0);
    
    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0.0, 1.0);

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0);

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0);

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
        Boolean2 {
            x: self.x.is_nan(),
            y: self.y.is_nan(),
        }.any()
    }

    /// Returns `true` if at least one element of the vector is [`f32::INFINITY`].
    #[inline]
    pub fn is_infinite(&self) -> bool {
        Boolean2 {
            x: self.x.is_infinite(),
            y: self.y.is_infinite(),
        }.any()
    }
}

// Vector swizzle code implementation.
impl Float2 {
    #[inline]
    pub const fn xx(self) -> Float2 {
        Float2 { x: self.x, y: self.x }
    }

    #[inline]
    pub const fn xy(self) -> Float2 {
        Float2 { x: self.x, y: self.y }
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
    pub const fn xxx(self) -> Float3 {
        Float3 { x: self.x, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn xxy(self) -> Float3 {
        Float3 { x: self.x, y: self.x, z: self.y }
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
    pub const fn yxx(self) -> Float3 {
        Float3 { x: self.y, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn yxy(self) -> Float3 {
        Float3 { x: self.y, y: self.x, z: self.y }
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
    pub const fn xxxx(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xxxy(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.x, w: self.y }
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
    pub const fn xyxx(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xyxy(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.x, w: self.y }
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
    pub const fn yxxx(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yxxy(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.x, w: self.y }
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
    pub const fn yyxx(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn yyxy(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn yyyx(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn yyyy(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.y, w: self.y }
    }
}

impl_element2!(f32, Float2);

impl_element2_op!(f32, Float2);

impl Default for Float2 {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<Float3> for Float2 {
    #[inline]
    fn from(value: Float3) -> Self {
        Float2 { x: value.x, y: value.y }
    }
}

impl From<Float4> for Float2 {
    #[inline]
    fn from(value: Float4) -> Self {
        Float2 { x: value.x, y: value.y }
    }
}

impl core::ops::Neg for Float2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x, 
            y: -self.y, 
        }
    }
}
