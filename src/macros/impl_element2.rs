macro_rules! impl_element2 {
    ($s:ty, $t:ty) => {
        impl $t {
            /// Creates with given elements.
            #[inline]
            #[must_use]
            pub const fn new(x: $s, y: $s) -> Self {
                Self { x, y }
            }

            /// Fills all elements with the given values.
            #[inline]
            #[must_use]
            pub const fn fill(val: $s) -> Self {
                Self { x: val, y: val }
            }

            /// Creates with given array.
            #[inline]
            #[must_use]
            pub const fn from_array(arr: [$s; 2]) -> Self {
                Self { x: arr[0], y: arr[1] }
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
                Self { x: slice[0], y: slice[1] }
            }

            /// Creates with given tuple.
            #[inline]
            #[must_use]
            pub const fn from_tuple(tuple: ($s, $s)) -> Self {
                Self { x: tuple.0, y: tuple.1 }
            }
        }

        impl From<[$s; 2]> for $t {
            #[inline]
            fn from(value: [$s; 2]) -> Self {
                Self { x: value[0], y: value[1] }
            }
        }

        impl Into<[$s; 2]> for $t {
            #[inline]
            fn into(self) -> [$s; 2] {
                [self.x, self.y]
            }
        }

        impl From<($s, $s)> for $t {
            #[inline]
            fn from(value: ($s, $s)) -> Self {
                Self { x: value.0, y: value.1 }
            }
        }

        impl Into<($s, $s)> for $t {
            #[inline]
            fn into(self) -> ($s, $s) {
                (self.x, self.y)
            }
        }

        impl AsRef<[$s; 2]> for $t {
            #[inline]
            fn as_ref(&self) -> &[$s; 2] {
                unsafe { &*(self as *const $t as *const [$s; 2]) }
            }
        }

        impl AsMut<[$s; 2]> for $t {
            #[inline]
            fn as_mut(&mut self) -> &mut [$s; 2] {
                unsafe { &mut *(self as *mut $t as *mut [$s; 2]) }
            }
        }

        impl core::ops::Index<usize> for $t {
            type Output = $s;
            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!("index out of range!"),
                }
            }
        }
        
        impl core::ops::IndexMut<usize> for $t {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!("index out of range!")
                }
            }
        }

        impl core::fmt::Debug for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($t))
                    .field(&self.x)
                    .field(&self.y)
                    .finish()
            }
        }

        impl core::fmt::Display for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{{ {}, {} }}", &self.x, &self.y)
            }
        }
    };
}

pub(crate) use impl_element2;
