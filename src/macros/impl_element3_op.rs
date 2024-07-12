macro_rules! impl_element3_op {
    ($s:ty, $t:ty) => {
        impl core::ops::Add<$t> for $s {
            type Output = $t;
            /// Adds a vector elements to scalar value.
            #[inline]
            fn add(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self + rhs.x, 
                    y: self + rhs.y, 
                    z: self + rhs.z, 
                }
            }
        }
        
        impl core::ops::Add<$s> for $t {
            type Output = $t;
            /// Adds a scalar value to vector elements.
            #[inline]
            fn add(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self.x + rhs, 
                    y: self.y + rhs, 
                    z: self.z + rhs, 
                }
            }
        }

        impl core::ops::AddAssign<$s> for $t {
            /// Adds a scalar value to vector elements. (assign)
            #[inline]
            fn add_assign(&mut self, rhs: $s) {
                *self = *self + rhs
            }
        }

        impl core::ops::Add<$t> for $t {
            type Output = $t;
            /// Adds two vectors. 
            #[inline]
            fn add(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x, 
                    y: self.y + rhs.y, 
                    z: self.z + rhs.z, 
                }
            }
        }

        impl core::ops::AddAssign<$t> for $t {
            /// Adds two vectors. (assign)
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                *self = *self + rhs
            }
        }



        impl core::ops::Sub<$t> for $s {
            type Output = $t;
            /// Subtracts a vector elements to scalar value.
            #[inline]
            fn sub(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self - rhs.x, 
                    y: self - rhs.y, 
                    z: self - rhs.z, 
                }
            }
        }

        impl core::ops::Sub<$s> for $t {
            type Output = $t;
            /// Subtracts a scalar value to vector elements.
            #[inline]
            fn sub(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self.x - rhs, 
                    y: self.y - rhs, 
                    z: self.z - rhs, 
                }
            }
        }

        impl core::ops::SubAssign<$s> for $t {
            /// Subtracts a scalar value to vector elements. (assign)
            #[inline]
            fn sub_assign(&mut self, rhs: $s) {
                *self = *self - rhs
            }
        }

        impl core::ops::Sub<$t> for $t {
            type Output = $t;
            /// Subtracts two vectors.
            #[inline]
            fn sub(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x, 
                    y: self.y - rhs.y, 
                    z: self.z - rhs.z, 
                }
            }
        }

        impl core::ops::SubAssign<$t> for $t {
            /// Subtracts two vectors. (assign)
            #[inline]
            fn sub_assign(&mut self, rhs: $t) {
                *self = *self - rhs
            }
        }



        impl core::ops::Mul<$t> for $s {
            type Output = $t;
            /// Multiply a vector elements to scalar value.
            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self * rhs.x, 
                    y: self * rhs.y, 
                    z: self * rhs.z, 
                }
            }
        }

        impl core::ops::Mul<$s> for $t {
            type Output = $t;
            /// Multiply a scalar value to vector elements.
            #[inline]
            fn mul(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self.x * rhs, 
                    y: self.y * rhs, 
                    z: self.z * rhs, 
                }
            }
        }

        impl core::ops::MulAssign<$s> for $t {
            /// Multiply a scalar value to vector elements. (assign)
            #[inline]
            fn mul_assign(&mut self, rhs: $s) {
                *self = *self * rhs
            }
        }

        impl core::ops::Mul<$t> for $t {
            type Output = $t;
            /// Element-wise multiplication of two vectors.
            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self.x * rhs.x, 
                    y: self.y * rhs.y, 
                    z: self.z * rhs.z, 
                }
            }
        }

        impl core::ops::MulAssign<$t> for $t {
            /// Element-wise multiplication of two vectors. (assign)
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs
            }
        }



        impl core::ops::Div<$t> for $s {
            type Output = $t;
            /// Divide a scalar value to a vector elements.
            #[inline]
            fn div(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self / rhs.x, 
                    y: self / rhs.y, 
                    z: self / rhs.z, 
                }
            }
        }

        impl core::ops::Div<$s> for $t {
            type Output = $t;
            /// Divide a vector elements to a scalar value.
            #[inline]
            fn div(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self.x / rhs, 
                    y: self.y / rhs, 
                    z: self.z / rhs, 
                }
            }
        }

        impl core::ops::DivAssign<$s> for $t {
            /// Divide a vector elements to a scalar value. (assign)
            #[inline]
            fn div_assign(&mut self, rhs: $s) {
                *self = *self / rhs
            }
        }

        impl core::ops::Div<$t> for $t {
            type Output = $t;
            /// Element-wise division of two vectors. 
            #[inline]
            fn div(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self.x / rhs.x, 
                    y: self.y / rhs.y, 
                    z: self.z / rhs.z, 
                }
            }
        }

        impl core::ops::DivAssign<$t> for $t {
            /// Element-wise division of two vectors. (assign)
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs
            }
        }



        impl core::ops::Rem<$t> for $s {
            type Output = $t;
            /// Remainders a vector elements to a scalar value.
            #[inline]
            fn rem(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self % rhs.x, 
                    y: self % rhs.y, 
                    z: self % rhs.z, 
                }
            }
        }

        impl core::ops::Rem<$s> for $t {
            type Output = $t;
            /// Remainders a scalar value to a vector elements.
            #[inline]
            fn rem(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self.x % rhs, 
                    y: self.y % rhs, 
                    z: self.z % rhs, 
                }
            }
        }

        impl core::ops::RemAssign<$s> for $t {
            /// Remainders a scalar value to a vector elements. (assign)
            #[inline]
            fn rem_assign(&mut self, rhs: $s) {
                *self = *self % rhs
            }
        }

        impl core::ops::Rem<$t> for $t {
            type Output = $t;
            /// Element-wise remaindation of two vectors.
            fn rem(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self.x % rhs.x, 
                    y: self.y % rhs.y, 
                    z: self.z % rhs.z, 
                }
            }
        }

        impl core::ops::RemAssign<$t> for $t {
            /// Element-wise remaindation of two vectors. (assign)
            #[inline]
            fn rem_assign(&mut self, rhs: $t) {
                *self = *self % rhs
            }
        }
    };
}

pub(crate) use impl_element3_op;
