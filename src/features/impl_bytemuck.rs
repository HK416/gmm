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
