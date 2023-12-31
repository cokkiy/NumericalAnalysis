//#![feature(stdsimd)]
//#![feature(avx512_target_feature)]

mod aarch64_neon;
mod poly_eval;
mod x86_64;
pub use poly_eval::poly_eval;

#[cfg(test)]
mod poly_eval_tests;

#[cfg(test)]
mod poly_eval_no_simd_tests;

#[cfg(test)]
mod poly_eval_simd_tests;
