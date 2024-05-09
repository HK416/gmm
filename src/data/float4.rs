use core::fmt;
use core::ops;
use core::f32;

use super::bool4::Boolean4;
use super::float2::Float2;
use super::float3::Float3;

/// A structure that stores four-dimensional vector data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Float4 {
    /// All elements are zero.
    pub const ZERO: Self = Self::fill(0.0);

    /// All elements are one.
    pub const ONE: Self = Self::fill(1.0);

    /// All elements are negative one.
    pub const NEG_ONE: Self = Self::fill(-1.0);

    /// positive unit vector on x-axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    /// positive unit vector on y-axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);

    /// positive unit vector on z-axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);

    /// positive unit vector on w-axis.
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// negative unit vector on x-axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);

    /// negative unit vector on y-axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);

    /// negative unit vector on z-axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);

    /// negative unit vector on w-axis.
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

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

    /// Creates with given elements.
    #[must_use]
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Fills all elements with the given values.
    #[must_use]
    #[inline(always)]
    pub const fn fill(val: f32) -> Self {
        Self { x: val, y: val, z: val, w: val }
    }

    /// Returns `true` if at least one element of the vector is [`f32::NAN`].
    #[inline]
    pub fn is_nan(&self) -> bool {
        Boolean4 {
            x: self.x.is_nan(),
            y: self.y.is_nan(),
            z: self.z.is_nan(),
            w: self.w.is_nan(),
        }.any()
    }

    /// Returns `true` if at least one element of the vector is [`f32::INFINITY`].
    #[inline]
    pub fn is_infinite(&self) -> bool {
        Boolean4 {
            x: self.x.is_infinite(),
            y: self.y.is_infinite(),
            z: self.z.is_infinite(),
            w: self.w.is_infinite(),
        }.any()
    }
}

// Vector swizzle code implementation.
impl Float4 {
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
    pub const fn xw(self) -> Float2 {
        Float2 { x: self.x, y: self.w }
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
    pub const fn yw(self) -> Float2 {
        Float2 { x: self.y, y: self.w }
    }

    #[inline]
    pub const fn zx(self) -> Float2 {
        Float2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(self) -> Float2 {
        Float2 { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zz(self) -> Float2 {
        Float2 { x: self.z, y: self.z }
    }

    #[inline]
    pub const fn zw(self) -> Float2 {
        Float2 { x: self.z, y: self.w }
    }

    #[inline]
    pub const fn wx(self) -> Float2 {
        Float2 { x: self.w, y: self.x }
    }

    #[inline]
    pub const fn wy(self) -> Float2 {
        Float2 { x: self.w, y: self.y }
    }

    #[inline]
    pub const fn wz(self) -> Float2 {
        Float2 { x: self.w, y: self.z }
    }

    #[inline]
    pub const fn ww(self) -> Float2 {
        Float2 { x: self.w, y: self.w }
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
    pub const fn xxw(self) -> Float3 {
        Float3 { x: self.x, y: self.x, z: self.w }
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
    pub const fn xyw(self) -> Float3 {
        Float3 { x: self.x, y: self.y, z: self.w }
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
    pub const fn xzw(self) -> Float3 {
        Float3 { x: self.x, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn xwx(self) -> Float3 {
        Float3 { x: self.x, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn xwy(self) -> Float3 {
        Float3 { x: self.x, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn xwz(self) -> Float3 {
        Float3 { x: self.x, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn xww(self) -> Float3 {
        Float3 { x: self.x, y: self.w, z: self.w }
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
    pub const fn yxw(self) -> Float3 {
        Float3 { x: self.y, y: self.x, z: self.w }
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
    pub const fn yyw(self) -> Float3 {
        Float3 { x: self.y, y: self.y, z: self.w }
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
    pub const fn yzw(self) -> Float3 {
        Float3 { x: self.y, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn ywx(self) -> Float3 {
        Float3 { x: self.y, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn ywy(self) -> Float3 {
        Float3 { x: self.y, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn ywz(self) -> Float3 {
        Float3 { x: self.y, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn yww(self) -> Float3 {
        Float3 { x: self.y, y: self.w, z: self.w }
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
    pub const fn zxw(self) -> Float3 {
        Float3 { x: self.z, y: self.x, z: self.w }
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
    pub const fn zyw(self) -> Float3 {
        Float3 { x: self.z, y: self.y, z: self.w }
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
    pub const fn zzw(self) -> Float3 {
        Float3 { x: self.z, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn zwx(self) -> Float3 {
        Float3 { x: self.z, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn zwy(self) -> Float3 {
        Float3 { x: self.z, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn zwz(self) -> Float3 {
        Float3 { x: self.z, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn zww(self) -> Float3 {
        Float3 { x: self.z, y: self.w, z: self.w }
    }

    #[inline]
    pub const fn wxx(self) -> Float3 {
        Float3 { x: self.w, y: self.x, z: self.x }
    }

    #[inline]
    pub const fn wxy(self) -> Float3 {
        Float3 { x: self.w, y: self.x, z: self.y }
    }

    #[inline]
    pub const fn wxz(self) -> Float3 {
        Float3 { x: self.w, y: self.x, z: self.z }
    }

    #[inline]
    pub const fn wxw(self) -> Float3 {
        Float3 { x: self.w, y: self.x, z: self.w }
    }

    #[inline]
    pub const fn wyx(self) -> Float3 {
        Float3 { x: self.w, y: self.y, z: self.x }
    }

    #[inline]
    pub const fn wyy(self) -> Float3 {
        Float3 { x: self.w, y: self.y, z: self.y }
    }

    #[inline]
    pub const fn wyz(self) -> Float3 {
        Float3 { x: self.w, y: self.y, z: self.z }
    }

    #[inline]
    pub const fn wyw(self) -> Float3 {
        Float3 { x: self.w, y: self.y, z: self.w }
    }

    #[inline]
    pub const fn wzx(self) -> Float3 {
        Float3 { x: self.w, y: self.z, z: self.x }
    }

    #[inline]
    pub const fn wzy(self) -> Float3 {
        Float3 { x: self.w, y: self.z, z: self.y }
    }

    #[inline]
    pub const fn wzz(self) -> Float3 {
        Float3 { x: self.w, y: self.z, z: self.z }
    }

    #[inline]
    pub const fn wzw(self) -> Float3 {
        Float3 { x: self.w, y: self.z, z: self.w }
    }

    #[inline]
    pub const fn wwx(self) -> Float3 {
        Float3 { x: self.w, y: self.w, z: self.x }
    }

    #[inline]
    pub const fn wwy(self) -> Float3 {
        Float3 { x: self.w, y: self.w, z: self.y }
    }

    #[inline]
    pub const fn wwz(self) -> Float3 {
        Float3 { x: self.w, y: self.w, z: self.z }
    }

    #[inline]
    pub const fn www(self) -> Float3 {
        Float3 { x: self.w, y: self.w, z: self.w }
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
    pub const fn xxxw(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.x, w: self.w }
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
    pub const fn xxyw(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.y, w: self.w }
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
    pub const fn xxzw(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xxwx(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xxwy(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xxwz(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xxww(self) -> Float4 {
        Float4 { x: self.x, y: self.x, z: self.w, w: self.w }
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
    pub const fn xyxw(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.x, w: self.w }
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
    pub const fn xyyw(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.y, w: self.w }
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
    pub const fn xyzw(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xywx(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xywy(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xywz(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xyww(self) -> Float4 {
        Float4 { x: self.x, y: self.y, z: self.w, w: self.w }
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
    pub const fn xzxw(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.x, w: self.w }
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
    pub const fn xzyw(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.y, w: self.w }
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
    pub const fn xzzw(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xzwx(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xzwy(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xzwz(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xzww(self) -> Float4 {
        Float4 { x: self.x, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn xwxx(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn xwxy(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn xwxz(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn xwxw(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn xwyx(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn xwyy(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn xwyz(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn xwyw(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn xwzx(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn xwzy(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn xwzz(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn xwzw(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn xwwx(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn xwwy(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn xwwz(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn xwww(self) -> Float4 {
        Float4 { x: self.x, y: self.w, z: self.w, w: self.w }
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
    pub const fn yxxw(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.x, w: self.w }
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
    pub const fn yxyz(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn yxyw(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.y, w: self.w }
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
    pub const fn yxzw(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yxwx(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yxwy(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yxwz(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yxww(self) -> Float4 {
        Float4 { x: self.y, y: self.x, z: self.w, w: self.w }
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
    pub const fn yyxw(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.x, w: self.w }
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
    pub const fn yyyw(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.y, w: self.w }
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
    pub const fn yyzw(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yywx(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yywy(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yywz(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yyww(self) -> Float4 {
        Float4 { x: self.y, y: self.y, z: self.w, w: self.w }
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
    pub const fn yzxw(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.x, w: self.w }
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
    pub const fn yzyw(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.y, w: self.w }
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
    pub const fn yzzw(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn yzwx(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn yzwy(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn yzwz(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn yzww(self) -> Float4 {
        Float4 { x: self.y, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn ywxx(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn ywxy(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn ywxz(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn ywxw(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn ywyx(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn ywyy(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn ywyz(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn ywyw(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn ywzx(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn ywzy(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn ywzz(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn ywzw(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn ywwx(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn ywwy(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn ywwz(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn ywww(self) -> Float4 {
        Float4 { x: self.y, y: self.w, z: self.w, w: self.w }
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
    pub const fn zxxw(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.x, w: self.w }
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
    pub const fn zxyw(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.y, w: self.w }
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
    pub const fn zxzw(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zxwx(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zxwy(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zxwz(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zxww(self) -> Float4 {
        Float4 { x: self.z, y: self.x, z: self.w, w: self.w }
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
    pub const fn zyxw(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.x, w: self.w }
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
    pub const fn zyyw(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.y, w: self.w }
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
    pub const fn zyzw(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zywx(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zywy(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zywz(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zyww(self) -> Float4 {
        Float4 { x: self.z, y: self.y, z: self.w, w: self.w }
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
    pub const fn zzxw(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.x, w: self.w }
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
    pub const fn zzyw(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.y, w: self.w }
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

    #[inline]
    pub const fn zzzw(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zzwx(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zzwy(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zzwz(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zzww(self) -> Float4 {
        Float4 { x: self.z, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn zwxx(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn zwxy(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn zwxz(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn zwxw(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn zwyx(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn zwyy(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn zwyz(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn zwyw(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn zwzx(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn zwzy(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn zwzz(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn zwzw(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn zwwx(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn zwwy(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn zwwz(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn zwww(self) -> Float4 {
        Float4 { x: self.z, y: self.w, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wxxx(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wxxy(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wxxz(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wxxw(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wxyx(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wxyy(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wxyz(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wxyw(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wxzx(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wxzy(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wxzz(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wxzw(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wxwx(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wxwy(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wxwz(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wxww(self) -> Float4 {
        Float4 { x: self.w, y: self.x, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wyxx(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wyxy(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wyxz(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wyxw(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wyyx(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wyyy(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wyyz(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wyyw(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wyzx(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wyzy(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wyzz(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wyzw(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wywx(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wywy(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wywz(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wyww(self) -> Float4 {
        Float4 { x: self.w, y: self.y, z: self.w, w: self.w }
    }

    #[inline]
    pub const fn wzxx(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wzxy(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wzxz(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wzxw(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wzyx(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wzyy(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wzyz(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wzyw(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wzzx(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wzzy(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wzzz(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wzzw(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wzwx(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wzwy(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wzwz(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wzww(self) -> Float4 {
        Float4 { x: self.w, y: self.z, z: self.w, w: self.w }
    }
    
    #[inline]
    pub const fn wwxx(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.x, w: self.x }
    }

    #[inline]
    pub const fn wwxy(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.x, w: self.y }
    }

    #[inline]
    pub const fn wwxz(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.x, w: self.z }
    }

    #[inline]
    pub const fn wwxw(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.x, w: self.w }
    }

    #[inline]
    pub const fn wwyx(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.y, w: self.x }
    }

    #[inline]
    pub const fn wwyy(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.y, w: self.y }
    }

    #[inline]
    pub const fn wwyz(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.y, w: self.z }
    }

    #[inline]
    pub const fn wwyw(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.y, w: self.w }
    }

    #[inline]
    pub const fn wwzx(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.z, w: self.x }
    }

    #[inline]
    pub const fn wwzy(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.z, w: self.y }
    }

    #[inline]
    pub const fn wwzz(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.z, w: self.z }
    }

    #[inline]
    pub const fn wwzw(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.z, w: self.w }
    }

    #[inline]
    pub const fn wwwx(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.w, w: self.x }
    }

    #[inline]
    pub const fn wwwy(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.w, w: self.y }
    }

    #[inline]
    pub const fn wwwz(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.w, w: self.z }
    }

    #[inline]
    pub const fn wwww(self) -> Float4 {
        Float4 { x: self.w, y: self.w, z: self.w, w: self.w }
    }
}

impl Default for Float4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<Float2> for Float4 {
    #[inline]
    fn from(value: Float2) -> Self {
        Float4 { x: value.x, y: value.y, z: 0.0, w: 0.0 }
    }
}

impl From<Float3> for Float4 {
    #[inline]
    fn from(value: Float3) -> Self {
        Float4 { x: value.x, y: value.y, z: value.z, w: 0.0 }
    }
}

impl AsRef<[f32; 4]> for Float4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Float4 as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Float4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Float4 as *mut [f32; 4]) }
    }
}

impl ops::Index<usize> for Float4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for Float4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of range!")
        }
    }
}

impl fmt::Debug for Float4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Float4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl fmt::Display for Float4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {} }}", &self.x, &self.y, &self.z, &self.w)
    }
}
