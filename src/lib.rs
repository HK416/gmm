//! # Game Math for Me
//! A math library for video games that allow independent management of data.
//! Just like [DirectXMath](https://github.com/microsoft/DirectXMath), data and vectors are divided.
//! 
//! # Features
//! ### Supports SIMD operations
//! The gmm library supports SIMD instructions on the `x86`, `x86_64` and the `aarch64` architecture.
//! 
//! 
//! ### Compile Features
//! - `bytemuck` - Enables the bytemuck library implementation.
//! - `mint` - Enables the mint library implementation.
//! - `scalar-math` - Disable the simd instruction in the library.
//! 
//! 
//! # License
//! MIT license (LICENSE or http://opensource.org/licenses/MIT)
//! 

#[cfg(test)]
mod tests;

mod macros;
mod features;

pub mod data;
pub mod vec;
// pub mod utils;



pub use self::data::Boolean2;
pub use self::data::Boolean3;
pub use self::data::Boolean4;

pub use self::data::Float2;
pub use self::data::Float3;
pub use self::data::Float4;

pub use self::data::Float3x3;
pub use self::data::Float4x4;

pub use self::data::Integer2;
pub use self::data::Integer3;
pub use self::data::Integer4;

pub use self::data::UInteger2;
pub use self::data::UInteger3;
pub use self::data::UInteger4;

pub use self::vec::Vector;
pub use self::vec::VectorInt;
