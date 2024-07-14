/// Number of tests.
const NUM_TEST: usize = 10_000_000;

/// Some SIMD instructions do not conform to IEEE-754. (for performance benefits)
/// 
/// So we compare using a separate Epsilon constant.
/// 
const EPSILON: f32 = 1.192092896e-6;



mod vector_op;
mod vector_base_op;
mod vector_cmp;

mod quaternion_op;

mod matrix_op;
mod matrix_base_op;
