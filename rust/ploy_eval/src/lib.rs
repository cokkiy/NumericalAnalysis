mod poly_eval;
pub use poly_eval::poly_eval;

#[cfg(test)]
mod ploy_eval_tests;

#[cfg(test)]
mod ploy_eval_no_simd_tests;

#[cfg(test)]
mod ploy_eval_simd_tests;
