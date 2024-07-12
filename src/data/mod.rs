//! # Data types
//! Data types are used to read and write data.
//! This has a similar purpose to `XMFloat*` or `XMInt*` in the `DirectXMath` library.
//! 
//! All instructions used for calculations on data types do not use SIMD instructions.
//! To perform calculations using SIMD instructions, conversion to vector type is required.
//! 

pub mod bool2;
pub mod bool3;
pub mod bool4;

pub mod float2;
pub mod float3;
pub mod float4;

pub mod float3x3;
pub mod float4x4;

pub mod int2;
pub mod int3;
pub mod int4;

pub mod uint2;
pub mod uint3;
pub mod uint4;
