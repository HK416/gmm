use core::fmt;

use super::float3::Float3;

/// A structure that stores 3x3 column major matrix data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Float3x3 {
    pub x_axis: Float3,
    pub y_axis: Float3,
    pub z_axis: Float3
}

impl fmt::Debug for Float3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Float3x3))
            .field(&self.x_axis)
            .field(&self.y_axis)
            .field(&self.z_axis)
            .finish()
    }
}

impl fmt::Display for Float3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", &self.x_axis, &self.y_axis, &self.z_axis)
    }
}
