//! # Vector types
//! Perform operations such as addition or subtraction.
//! 
//! # Example
//! ```
//! use gmm::Float4;
//! use gmm::vec::*;
//! 
//! let data = Float4::new(1.0, 2.0, 3.0, 4.0);
//! let vec = load_float4(data);
//! 
//! /* ...vector operation... */
//! 
//! let data = store_float4(vec);
//! 
//! ```
//! 
//! ### Supports SIMD operations
//! - `aarch64` - Supports SIMD operations using `neon`.
//! - `x86`, `x86_64` - Supports SIMD operations using `sse2`.
//! 

#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))] 
mod neon;

#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))]
pub use self::neon::*;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod sse2;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
pub use self::sse2::*;


#[cfg(any(feature = "scalar-math", not(any(target_feature = "neon", target_feature = "sse2"))))]
mod scalar;

#[cfg(any(feature = "scalar-math", not(any(target_feature = "neon", target_feature = "sse2"))))]
pub use self::scalar::*;



#[cfg(test)]
mod tests;
