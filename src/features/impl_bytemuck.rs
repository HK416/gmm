use bytemuck::{Pod, Zeroable};
use crate::{
    Boolean2, Boolean3, Boolean4, 
    Float2, Float3, Float3x3, Float4, Float4x4, 
    Integer2, Integer3, Integer4, UInteger2, UInteger3, UInteger4, 
};



unsafe impl Pod for Boolean2 {}
unsafe impl Zeroable for Boolean2 {}

unsafe impl Pod for Boolean3 {}
unsafe impl Zeroable for Boolean3 {}

unsafe impl Pod for Boolean4 {}
unsafe impl Zeroable for Boolean4 {}


unsafe impl Pod for Float2 {}
unsafe impl Zeroable for Float2 {}

unsafe impl Pod for Float3 {}
unsafe impl Zeroable for Float3 {}

unsafe impl Pod for Float4 {}
unsafe impl Zeroable for Float4 {}


unsafe impl Pod for Float3x3 {}
unsafe impl Zeroable for Float3x3 {}

unsafe impl Pod for Float4x4 {}
unsafe impl Zeroable for Float4x4 {}


unsafe impl Pod for Integer2 {}
unsafe impl Zeroable for Integer2 {}

unsafe impl Pod for Integer3 {}
unsafe impl Zeroable for Integer3 {}

unsafe impl Pod for Integer4 {}
unsafe impl Zeroable for Integer4 {}


unsafe impl Pod for UInteger2 {}
unsafe impl Zeroable for UInteger2 {}

unsafe impl Pod for UInteger3 {}
unsafe impl Zeroable for UInteger3 {}

unsafe impl Pod for UInteger4 {}
unsafe impl Zeroable for UInteger4 {}

// TODO: Add test function...
#[cfg(test)]
mod tests {
    use core::mem;
    use crate::{
        Boolean2, Boolean3, Boolean4, 
        Float2, Float3, Float3x3, Float4, Float4x4, 
        Integer2, Integer3, Integer4, UInteger2, UInteger3, UInteger4
    };

    macro_rules! test_pod_impl {
        ($name:ident, $t:ty) => {
            #[test]
            fn $name() {
                let t = <$t>::default();
                let bytes = bytemuck::bytes_of(&t);
                for byte in bytes {
                    assert_eq!(byte, byte);
                }

                assert_eq!(&t as *const _ as usize, bytes.as_ptr() as usize);
                assert_eq!(bytes.len(), mem::size_of_val(&t));
            }
        }
    }

    test_pod_impl!(impl_bytemuck_boolean2, Boolean2);
    test_pod_impl!(impl_bytemuck_boolean3, Boolean3);
    test_pod_impl!(impl_bytemuck_boolean4, Boolean4);

    test_pod_impl!(impl_bytemuck_float2, Float2);
    test_pod_impl!(impl_bytemuck_float3, Float3);
    test_pod_impl!(impl_bytemuck_float4, Float4);
    test_pod_impl!(impl_bytemuck_float3x3, Float3x3);
    test_pod_impl!(impl_bytemuck_float4x4, Float4x4);

    test_pod_impl!(impl_bytemuck_integer2, Integer2);
    test_pod_impl!(impl_bytemuck_integer3, Integer3);
    test_pod_impl!(impl_bytemuck_integer4, Integer4);

    test_pod_impl!(impl_bytemuck_uinteger2, UInteger2);
    test_pod_impl!(impl_bytemuck_uinteger3, UInteger3);
    test_pod_impl!(impl_bytemuck_uinteger4, UInteger4);
}
