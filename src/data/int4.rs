use core::fmt;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Integer4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl fmt::Debug for Integer4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Integer4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl fmt::Display for Integer4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {} }}", &self.x, &self.y, &self.z, &self.w)
    }
}
