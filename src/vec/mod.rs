#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))] 
mod neon;

#[cfg(all(target_feature = "neon", not(feature = "scalar-math")))]
pub use self::neon::*;



#[cfg(test)]
mod tests;
