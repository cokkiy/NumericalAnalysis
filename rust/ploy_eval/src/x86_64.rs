#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse3")]
pub(crate) unsafe fn sse2_eval(x: &Vec<f64>, c: Vec<f64>, base: Vec<f64>, result: &mut Vec<f64>) {
    use std::arch::x86_64::*;

    for i in (0..x.len()).step_by(2) {
        // load 2 x to reg
        let x_ix2 = _mm_loadu_pd(x.as_ptr().add(i));
        let mut y_i_0 = _mm_setzero_pd();

        (0..c.len()).for_each(|j| {
            // load next  element of c and b to reg and duplicate it, notice that c and b are reversed
            let c_jx1 = _mm_loaddup_pd(c.as_ptr().add(j));
            let base_jx1 = _mm_loaddup_pd(base.as_ptr().add(j));
            let sub_0 = _mm_sub_pd(x_ix2, base_jx1);
            let add_0 = _mm_add_pd(c_jx1, y_i_0);
            if j + 1 < c.len() {
                y_i_0 = _mm_mul_pd(sub_0, add_0);
            } else {
                y_i_0 = add_0;
            }
        });

        let ptr = result.as_mut_ptr().add(i);
        _mm_storeu_pd(ptr, y_i_0);
    }
}

#[cfg(all(target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn avx2_eval(x: &Vec<f64>, c: Vec<f64>, base: Vec<f64>, result: &mut Vec<f64>) {
    use std::arch::x86_64::*;

    for i in (0..x.len()).step_by(4) {
        // load 4 x to reg
        let x_ix4 = _mm256_loadu_pd(x.as_ptr().add(i));
        let mut y_ix4 = _mm256_setzero_pd();

        for j in 0..c.len() {
            // load next  element of c and b to reg and duplicate it, notice that c and b are reversed
            let c_jx1 = _mm_loaddup_pd(c.as_ptr().add(j));
            let c_jx4 = _mm256_broadcast_pd(&c_jx1);
            let base_jx1 = _mm_loaddup_pd(base.as_ptr().add(j));
            let base_jx4 = _mm256_broadcast_pd(&base_jx1);
            let sub_0 = _mm256_sub_pd(x_ix4, base_jx4);
            let add_0 = _mm256_add_pd(c_jx4, y_ix4);
            if j + 1 < c.len() {
                y_ix4 = _mm256_mul_pd(sub_0, add_0);
            } else {
                y_ix4 = add_0;
            }
        }

        let ptr = result.as_mut_ptr().add(i);
        _mm256_storeu_pd(ptr, y_ix4);
    }
}

#[cfg(all(target_arch = "x86_64"))]
#[target_feature(enable = "avx512f")]
pub(crate) unsafe fn avx512_eval(x: &Vec<f64>, c: Vec<f64>, base: Vec<f64>, result: &mut Vec<f64>) {
    use std::arch::x86_64::*;

    for i in (0..x.len()).step_by(8) {
        // load 4 x to reg
        let x_ix8 = _mm512_loadu_pd(x.as_ptr().add(i));
        let mut y_ix8 = _mm512_setzero_pd();

        for j in 0..c.len() {
            // load next 4 elements of c and b to reg and duplicate it, notice that c and b are reversed
            let c_j_1x = _mm_loaddup_pd(c.as_ptr().add(j));
            let c_j_8x = _mm512_broadcastsd_pd(c_j_1x);
            let base_jx1 = _mm_loaddup_pd(base.as_ptr().add(j));
            let base_jx8 = _mm512_broadcastsd_pd(base_jx1);
            // calc xi_0 with 4 elements c and b, (xi_0-b3)((xi_0-b2)((xi_0-b1)((xi_0-b0)c0+c1)+c2)+c3)
            let sub_0 = _mm512_sub_pd(x_ix8, base_jx8);
            let add_0 = _mm512_add_pd(c_j_8x, y_ix8);
            y_ix8 = _mm512_mul_pd(sub_0, add_0);
        }
        let ptr = result.as_mut_ptr().add(i);
        _mm512_storeu_pd(ptr, y_ix8)
    }
}
