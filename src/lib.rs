//! # Game Math for Me
//! A math library for video games that allow independent management of data.
//! Just like [DirectXMath](https://github.com/microsoft/DirectXMath), data and vectors are divided.
//! 
//! You can use it like below:
//! 
//! ### Example (without SIMD)
//! ```
//! use gmm::Float4;
//! 
//! let a = Float4::new(1.0, 2.0, 3.0, 4.0);
//! let b = Float4::fill(5.0);
//! let c = a + b;
//! 
//! println!("a = {}, b = {}, a + b = {}", a, b, c);
//! 
//! ```
//! 
//! ### Example (with SIMD)
//! ```
//! use gmm::Float4;
//! use gmm::vec::*;
//! 
//! let a = Float4::new(1.0, 2.0, 3.0, 4.0);
//! let b = Float4::fill(5.0);
//! 
//! let va = load_float4(a);
//! let vb = load_float4(b);
//! let c = store_float4(vector_add(va, vb));
//! 
//! println!("a = {}, b = {}, a + b = {}", a, b, c);
//! ```
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

mod macros;
mod features;

pub mod data;
pub mod vec;
pub mod utils;



pub use self::data::bool2::Boolean2;
pub use self::data::bool3::Boolean3;
pub use self::data::bool4::Boolean4;

pub use self::data::float2::Float2;
pub use self::data::float3::Float3;
pub use self::data::float4::Float4;

pub use self::data::float3x3::Float3x3;
pub use self::data::float4x4::Float4x4;

pub use self::data::int2::Integer2;
pub use self::data::int3::Integer3;
pub use self::data::int4::Integer4;

pub use self::data::uint2::UInteger2;
pub use self::data::uint3::UInteger3;
pub use self::data::uint4::UInteger4;
