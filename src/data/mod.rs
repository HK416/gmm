//! # Data types
//! Data types are used to load and store data.
//! This has a similar purpose to `XMFloat*` or `XMInt*` in the `DirectXMath` library.
//! 
//! All instructions used for calculations on data types do not use SIMD instructions.
//! To perform calculations using SIMD instructions, conversion to vector type is required.
//! 

mod bool2;
pub use self::bool2::Boolean2;
mod bool3;
pub use self::bool3::Boolean3;
mod bool4;
pub use self::bool4::Boolean4;

mod float2;
pub use self::float2::Float2;
mod float3;
pub use self::float3::Float3;
mod float4;
pub use self::float4::Float4;

mod float3x3;
pub use self::float3x3::Float3x3;
mod float4x4;
pub use self::float4x4::Float4x4;

mod int2;
pub use self::int2::Integer2;
mod int3;
pub use self::int3::Integer3;
mod int4;
pub use self::int4::Integer4;

mod uint2;
pub use self::uint2::UInteger2;
mod uint3;
pub use self::uint3::UInteger3;
mod uint4;
pub use self::uint4::UInteger4;
