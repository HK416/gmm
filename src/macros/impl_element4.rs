macro_rules! impl_element4 {
    ($s:ty, $t:ty) => {
        impl $t {
            /// Creates with given elements.
            #[inline]
            #[must_use]
            pub const fn new(x: $s, y: $s, z: $s, w: $s) -> Self {
                Self { x, y, z, w }
            }

            /// Fills all elements with the given values.
            #[inline]
            #[must_use]
            pub const fn fill(val: $s) -> Self {
                Self { x: val, y: val, z: val, w: val }
            }

            /// Creates with given array.
            #[inline]
            #[must_use]
            pub const fn from_array(arr: [$s; 4]) -> Self {
                Self { x: arr[0], y: arr[1], z: arr[2], w: arr[3] }
            }

            /// Convert to array.
            #[inline]
            #[must_use]
            pub const fn to_array(self) -> [$s; 4] {
                [self.x, self.y, self.z, self.w]
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
                Self { x: slice[0], y: slice[1], z: slice[2], w: slice[3] }
            }

            /// Creates with given tuple.
            #[inline]
            #[must_use]
            pub const fn from_tuple(tuple: ($s, $s, $s, $s)) -> Self {
                Self { x: tuple.0, y: tuple.1, z: tuple.2, w: tuple.3 }
            }

            /// Convert to tuple.
            #[inline]
            #[must_use]
            pub const fn to_tuple(self) -> ($s, $s, $s, $s) {
                (self.x, self.y, self.z, self.w)
            }

            /// Sets the value of the x element.
            #[inline]
            #[must_use]
            pub fn set_x(mut self, x: $s) -> $t {
                self.x = x;
                self
            }

            /// Sets the value of the y element.
            #[inline]
            #[must_use]
            pub fn set_y(mut self, y: $s) -> $t {
                self.y = y;
                self
            }

            /// Sets the value of the z element.
            #[inline]
            #[must_use]
            pub fn set_z(mut self, z: $s) -> $t {
                self.z = z;
                self
            }

            /// Sets the value of the w element.
            #[inline]
            #[must_use]
            pub fn set_w(mut self, w: $s) -> $t {
                self.w = w;
                self
            }
        }

        impl From<[$s; 4]> for $t {
            #[inline]
            fn from(value: [$s; 4]) -> Self {
                Self::from_array(value)
            }
        }

        impl Into<[$s; 4]> for $t {
            #[inline]
            fn into(self) -> [$s; 4] {
                self.to_array()
            }
        }

        impl From<($s, $s, $s, $s)> for $t {
            #[inline]
            fn from(value: ($s, $s, $s, $s)) -> Self {
                Self::from_tuple(value)
            }
        }

        impl Into<($s, $s, $s, $s)> for $t {
            #[inline]
            fn into(self) -> ($s, $s, $s, $s) {
                self.to_tuple()
            }
        }

        impl AsRef<[$s; 4]> for $t {
            #[inline]
            fn as_ref(&self) -> &[$s; 4] {
                unsafe { &*(self as *const $t as *const [$s; 4]) }
            }
        }

        impl AsMut<[$s; 4]> for $t {
            #[inline]
            fn as_mut(&mut self) -> &mut [$s; 4] {
                unsafe { &mut *(self as *mut $t as *mut [$s; 4]) }
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
                    3 => &self.w,
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
                    3 => &mut self.w,
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
                    .field(&self.w)
                    .finish()
            }
        }

        impl core::fmt::Display for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{{ {}, {}, {}, {} }}", &self.x, &self.y, &self.z, &self.w)
            }
        }
    };
}

pub(crate) use impl_element4;
