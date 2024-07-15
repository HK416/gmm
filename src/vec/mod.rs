//! # Vector types
//! Vector types are used to perform calculations.
//! This has a similar purpose to `XMVECTOR` or `XMMATRIX` in the `DirectXMath` library.
//! 
//! It is not appropriate to use vector types to load or store data.
//! If you want to load or store data, you should use a data type.
//! 
//! Using the `scalar-math` feature disables the use of `SIMD` instructions.
//! 
//! ### Supports SIMD operations
//! - `aarch64` - Supports SIMD operations using `neon`.
//! - `x86`, `x86_64` - Supports SIMD operations using `sse2`.
//! 

#[cfg(any(feature = "scalar-math", not(any(target_feature = "neon", target_feature = "ssec2"))))]
mod scalar;

#[cfg(any(feature = "scalar-math", not(any(target_feature = "neon", target_feature = "ssec2"))))]
pub use self::scalar::*;

#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))] 
mod neon;

#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))]
pub use self::neon::*;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod sse2;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
pub use self::sse2::*;
