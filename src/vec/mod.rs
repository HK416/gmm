#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))] 
mod neon;

#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))]
pub use self::neon::*;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod sse2;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
pub use self::sse2::*;



#[cfg(test)]
mod tests;
