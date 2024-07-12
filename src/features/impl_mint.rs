use crate::{
    Boolean2, Boolean3, Boolean4, 
    Float2, Float3, Float3x3, Float4, Float4x4, 
    Integer2, Integer3, Integer4, UInteger2, UInteger3, UInteger4, 
};


macro_rules! impl_vector2_type {
    ($t: ty, $v: ty) => {
        impl From<mint::Point2<$t>> for $v {
            fn from(v: mint::Point2<$t>) -> Self {
                Self { x: v.x, y: v.y }
            }
        }

        impl From<$v> for mint::Point2<$t> {
            fn from(v: $v) -> Self {
                Self { x: v.x, y: v.y }
            }
        }

        impl From<mint::Vector2<$t>> for $v {
            fn from(v: mint::Vector2<$t>) -> Self {
                Self { x: v.x, y: v.y }
            }
        }

        impl From<$v> for mint::Vector2<$t> {
            fn from(v: $v) -> Self {
                Self { x: v.x, y: v.y }
            }
        }

        impl mint::IntoMint for $v {
            type MintType = mint::Vector2<$t>;
        }
    };
}

macro_rules! impl_vector3_type {
    ($t: ty, $v: ty) => {
        impl From<mint::Point3<$t>> for $v {
            fn from(v: mint::Point3<$t>) -> Self {
                Self { x: v.x, y: v.y, z: v.z }
            }
        }

        impl From<$v> for mint::Point3<$t> {
            fn from(v: $v) -> Self {
                Self { x: v.x, y: v.y, z: v.z }
            }
        }

        impl From<mint::Vector3<$t>> for $v {
            fn from(v: mint::Vector3<$t>) -> Self {
                Self { x: v.x, y: v.y, z: v.z }
            }
        }

        impl From<$v> for mint::Vector3<$t> {
            fn from(v: $v) -> Self {
                Self { x: v.x, y: v.y, z: v.z }
            }
        }

        impl mint::IntoMint for $v {
            type MintType = mint::Vector3<$t>;
        }
    };
}

macro_rules! impl_vector4_type {
    ($t: ty, $v: ty) => {
        impl From<mint::Vector4<$t>> for $v {
            fn from(v: mint::Vector4<$t>) -> Self {
                Self { x: v.x, y: v.y, z: v.z, w: v.w }
            }
        }

        impl From<$v> for mint::Vector4<$t> {
            fn from(v: $v) -> Self {
                Self { x: v.x, y: v.y, z: v.z, w: v.w }
            }
        }

        impl mint::IntoMint for $v {
            type MintType = mint::Vector4<$t>;
        }
    };
}



impl_vector2_type!(bool, Boolean2);
impl_vector3_type!(bool, Boolean3);
impl_vector4_type!(bool, Boolean4);

impl_vector2_type!(f32, Float2);
impl_vector3_type!(f32, Float3);
impl_vector4_type!(f32, Float4);

impl_vector2_type!(i32, Integer2);
impl_vector3_type!(i32, Integer3);
impl_vector4_type!(i32, Integer4);

impl_vector2_type!(u32, UInteger2);
impl_vector3_type!(u32, UInteger3);
impl_vector4_type!(u32, UInteger4);



impl From<mint::Quaternion<f32>> for Float4 {
    fn from(value: mint::Quaternion<f32>) -> Self {
        Self { x: value.v.x, y: value.v.y, z: value.v.z, w: value.s }
    }
}

impl From<Float4> for mint::Quaternion<f32> {
    fn from(value: Float4) -> Self {
        Self { 
            s: value.w, 
            v: mint::Vector3 { 
                x: value.x, 
                y: value.y, 
                z: value.z 
            }, 
        }
    }
}



impl From<mint::RowMatrix3<f32>> for Float3x3 {
    fn from(value: mint::RowMatrix3<f32>) -> Self {
        Self { 
            x_axis: Float3 { x: value.x.x, y: value.y.x, z: value.z.x }, 
            y_axis: Float3 { x: value.x.y, y: value.y.y, z: value.z.y }, 
            z_axis: Float3 { x: value.x.z, y: value.y.z, z: value.z.z } 
        }
    }
}

impl From<mint::ColumnMatrix3<f32>> for Float3x3 {
    fn from(value: mint::ColumnMatrix3<f32>) -> Self {
        Self { 
            x_axis: Float3 { x: value.x.x, y: value.x.y, z: value.x.z }, 
            y_axis: Float3 { x: value.y.x, y: value.y.y, z: value.y.z }, 
            z_axis: Float3 { x: value.z.x, y: value.z.y, z: value.z.z } 
        }
    }
}

impl From<Float3x3> for mint::RowMatrix3<f32> {
    fn from(value: Float3x3) -> Self {
        Self { 
            x: mint::Vector3 { x: value.x_axis.x, y: value.y_axis.x, z: value.z_axis.x }, 
            y: mint::Vector3 { x: value.x_axis.y, y: value.y_axis.y, z: value.z_axis.y }, 
            z: mint::Vector3 { x: value.x_axis.z, y: value.y_axis.z, z: value.z_axis.z } 
        }
    }
}

impl From<Float3x3> for mint::ColumnMatrix3<f32> {
    fn from(value: Float3x3) -> Self {
        Self { 
            x: mint::Vector3 { x: value.x_axis.x, y: value.x_axis.y, z: value.x_axis.z }, 
            y: mint::Vector3 { x: value.y_axis.x, y: value.y_axis.y, z: value.y_axis.z }, 
            z: mint::Vector3 { x: value.z_axis.x, y: value.z_axis.y, z: value.z_axis.z } 
        }
    }
}

impl mint::IntoMint for Float3x3 {
    type MintType = mint::ColumnMatrix3<f32>;
}



impl From<mint::RowMatrix4<f32>> for Float4x4 {
    fn from(value: mint::RowMatrix4<f32>) -> Self {
        Self { 
            x_axis: Float4 { x: value.x.x, y: value.y.x, z: value.z.x, w: value.w.x }, 
            y_axis: Float4 { x: value.x.y, y: value.y.y, z: value.z.y, w: value.w.y }, 
            z_axis: Float4 { x: value.x.z, y: value.y.z, z: value.z.z, w: value.w.z }, 
            w_axis: Float4 { x: value.x.w, y: value.y.w, z: value.z.w, w: value.w.w } 
        }
    }
}

impl From<mint::ColumnMatrix4<f32>> for Float4x4 {
    fn from(value: mint::ColumnMatrix4<f32>) -> Self {
        Self { 
            x_axis: Float4 { x: value.x.x, y: value.x.y, z: value.x.z, w: value.x.w }, 
            y_axis: Float4 { x: value.y.x, y: value.y.y, z: value.y.z, w: value.y.w }, 
            z_axis: Float4 { x: value.z.x, y: value.z.y, z: value.z.z, w: value.z.w }, 
            w_axis: Float4 { x: value.w.x, y: value.w.y, z: value.w.z, w: value.w.w } 
        }
    }
}

impl From<Float4x4> for mint::RowMatrix4<f32> {
    fn from(value: Float4x4) -> Self {
        Self { 
            x: mint::Vector4 { x: value.x_axis.x, y: value.y_axis.x, z: value.z_axis.x, w: value.w_axis.x }, 
            y: mint::Vector4 { x: value.x_axis.y, y: value.y_axis.y, z: value.z_axis.y, w: value.w_axis.y }, 
            z: mint::Vector4 { x: value.x_axis.z, y: value.y_axis.z, z: value.z_axis.z, w: value.w_axis.z }, 
            w: mint::Vector4 { x: value.x_axis.w, y: value.y_axis.w, z: value.z_axis.w, w: value.w_axis.w } 
        }
    }
}

impl From<Float4x4> for mint::ColumnMatrix4<f32> {
    fn from(value: Float4x4) -> Self {
        Self { 
            x: mint::Vector4 { x: value.x_axis.x, y: value.x_axis.y, z: value.x_axis.z, w: value.x_axis.w }, 
            y: mint::Vector4 { x: value.y_axis.x, y: value.y_axis.y, z: value.y_axis.z, w: value.y_axis.w }, 
            z: mint::Vector4 { x: value.z_axis.x, y: value.z_axis.y, z: value.z_axis.z, w: value.z_axis.w }, 
            w: mint::Vector4 { x: value.w_axis.x, y: value.w_axis.y, z: value.w_axis.z, w: value.w_axis.w } 
        }
    }
}

impl mint::IntoMint for Float4x4 {
    type MintType = mint::ColumnMatrix4<f32>;
}
