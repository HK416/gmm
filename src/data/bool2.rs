use core::fmt;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean2 {
    pub x: bool,
    pub y: bool,
}

impl fmt::Debug for Boolean2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Boolean2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl fmt::Display for Boolean2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {} }}", &self.x, &self.y)
    }
}
