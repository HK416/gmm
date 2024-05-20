use core::fmt;

use super::float4::Float4;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Float4x4 {
    pub x_axis: Float4,
    pub y_axis: Float4,
    pub z_axis: Float4,
    pub w_axis: Float4,
}

impl fmt::Debug for Float4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Float4x4))
            .field(&self.x_axis)
            .field(&self.y_axis)
            .field(&self.z_axis)
            .field(&self.w_axis)
            .finish()
    }
}

impl fmt::Display for Float4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", &self.x_axis, &self.y_axis, &self.z_axis, &self.w_axis)
    }
}
