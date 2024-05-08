use core::fmt;

/// A structure that stores two-dimensional integer data.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Integer2 {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Integer2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Integer2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl fmt::Display for Integer2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {} }}", &self.x, &self.y)
    }
}
