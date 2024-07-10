//! # Independent Management Matrix (IMM)
//! A math library for video games that allow independent management of data.
//! Just like [DirectXMath](https://github.com/microsoft/DirectXMath), data and vectors are divided.
//! 
//! 
//! # Example
//! ```
//! use imm::prelude::*;
//! 
//! let a = Float4::new(1.0, 2.0, 3.0, 4.0);
//! let b = Float4::fill(5.0);
//! 
//! let va = load_float4(a);
//! let vb = load_float4(b);
//! let vc = vector_add(va, vb);
//! 
//! let c = store_float4(vc);
//! 
//! println!("a:{}, b:{}, a + b:{}", a, b, c);
//! 
//! ```
//! 
//! 
//! # Features
//! ### Supports SIMD operations
//! The imm library supports SIMD instructions on the `x86`, `x86_64` and the `aarch64` architecture.
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

pub mod prelude;

pub mod data;
pub mod vec;

pub mod utils;
