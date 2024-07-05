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
