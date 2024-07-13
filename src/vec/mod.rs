//! # Vector types
//! Vector types are used to perform calculations.
//! This has a similar purpose to `XMVECTOR` or `XMMATRIX` in the `DirectXMath` library.
//! 
//! It is not appropriate to use vector types to load or store data.
//! If you want to load or store data, you should use a data type.
//! 
//! ### Supports SIMD operations
//! - `aarch64` - Supports SIMD operations using `neon`.
//! - `x86`, `x86_64` - Supports SIMD operations using `sse2`.
//! 

#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))] 
mod neon;

#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))]
pub use self::neon::*;
