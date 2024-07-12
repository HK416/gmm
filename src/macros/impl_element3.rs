macro_rules! impl_element3 {
    ($s:ty, $t:ty) => {
        impl $t {
            /// Creates with given elements.
            #[inline]
            #[must_use]
            pub const fn new(x: $s, y: $s, z: $s) -> Self {
                Self { x, y, z }
            }

            /// Fills all elements with the given values.
            #[inline]
            #[must_use]
            pub const fn fill(val: $s) -> Self {
                Self { x: val, y: val, z: val }
            }

            /// Creates with given array.
            #[inline]
            #[must_use]
            pub const fn from_array(arr: [$s; 3]) -> Self {
                Self { x: arr[0], y: arr[1], z: arr[2] }
            }

            /// Creates with given slice.
            /// 
            /// # Panics
            /// If the length of the given array is less than the number of elements in the vector,
            /// an index out of range error occurs.
            /// 
            #[inline]
            #[must_use]
            pub fn from_slice(slice: &[$s]) -> Self {
                Self { x: slice[0], y: slice[1], z: slice[2] }
            }

            /// Creates with given tuple.
            #[inline]
            #[must_use]
            pub const fn from_tuple(tuple: ($s, $s, $s)) -> Self {
                Self { x: tuple.0, y: tuple.1, z: tuple.2 }
            }
        }

        impl From<[$s; 3]> for $t {
            #[inline]
            fn from(value: [$s; 3]) -> Self {
                Self { x: value[0], y: value[1], z: value[2] }
            }
        }

        impl Into<[$s; 3]> for $t {
            #[inline]
            fn into(self) -> [$s; 3] {
                [self.x, self.y, self.z]
            }
        }

        impl From<($s, $s, $s)> for $t {
            #[inline]
            fn from(value: ($s, $s, $s)) -> Self {
                Self { x: value.0, y: value.1, z: value.2 }
            }
        }

        impl Into<($s, $s, $s)> for $t {
            #[inline]
            fn into(self) -> ($s, $s, $s) {
                (self.x, self.y, self.z)
            }
        }

        impl AsRef<[$s; 3]> for $t {
            #[inline]
            fn as_ref(&self) -> &[$s; 3] {
                unsafe { &*(self as *const $t as *const [$s; 3]) }
            }
        }
        
        impl AsMut<[$s; 3]> for $t {
            #[inline]
            fn as_mut(&mut self) -> &mut [$s; 3] {
                unsafe { &mut *(self as *mut $t as *mut [$s; 3]) }
            }
        }
        
        impl core::ops::Index<usize> for $t {
            type Output = $s;
            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!("index out of range!"),
                }
            }
        }
        
        impl core::ops::IndexMut<usize> for $t {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!("index out of range!")
                }
            }
        }
        
        impl core::fmt::Debug for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($t))
                    .field(&self.x)
                    .field(&self.y)
                    .field(&self.z)
                    .finish()
            }
        }
        
        impl core::fmt::Display for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{{ {}, {}, {} }}", &self.x, &self.y, &self.z)
            }
        }
    };
}

pub(crate) use impl_element3;
