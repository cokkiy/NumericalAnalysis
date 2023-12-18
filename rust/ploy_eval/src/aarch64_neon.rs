#[cfg(target_arch = "aarch64")]
#[inline]
pub(crate) unsafe fn neon_eval(x: &Vec<f64>, c: Vec<f64>, base: Vec<f64>, result: &mut Vec<f64>) {
    use std::arch::aarch64::*;

    for i in (0..x.len()).step_by(8) {
        // load 8 x to reg
        let x_ix8 = vld1q_f64_x4(x.as_ptr().add(i));
        let mut y_i_0 = vmovq_n_f64(0.0);
        let mut y_i_1 = vmovq_n_f64(0.0);
        let mut y_i_2 = vmovq_n_f64(0.0);
        let mut y_i_3 = vmovq_n_f64(0.0);

        (0..c.len()).step_by(4).for_each(|j| {
            // load next 4 elements of c and b to reg and duplicate it, notice that c and b are reversed
            let c_jx4 = vld4q_dup_f64(c.as_ptr().add(j));
            let base_jx4 = vld4q_dup_f64(base.as_ptr().add(j));
            // calc xi_0 with 4 elements c and b, (xi_0-b3)((xi_0-b2)((xi_0-b1)((xi_0-b0)c0+c1)+c2)+c3)
            let sub_0 = vsubq_f64(x_ix8.0, base_jx4.0);
            let add_0 = vaddq_f64(c_jx4.0, y_i_0);
            y_i_0 = vmulq_f64(sub_0, add_0);

            let sub_0 = vsubq_f64(x_ix8.0, base_jx4.1);
            let add_0 = vaddq_f64(c_jx4.1, y_i_0);
            y_i_0 = vmulq_f64(sub_0, add_0);

            let sub_0 = vsubq_f64(x_ix8.0, base_jx4.2);
            let add_0 = vaddq_f64(c_jx4.2, y_i_0);
            y_i_0 = vmulq_f64(sub_0, add_0);

            let sub_0 = vsubq_f64(x_ix8.0, base_jx4.3);
            let add_0 = vaddq_f64(c_jx4.3, y_i_0);
            if j + 4 < c.len() {
                y_i_0 = vmulq_f64(sub_0, add_0);
            } else {
                y_i_0 = add_0;
            }

            // calc xi_1 with 4 elements c and b, (xi_0-b3)((xi_0-b2)((xi_0-b1)((xi_0-b0)c0+c1)+c2)+c3)
            let sub_1 = vsubq_f64(x_ix8.1, base_jx4.0);
            let add_1 = vaddq_f64(c_jx4.0, y_i_1);
            y_i_1 = vmulq_f64(sub_1, add_1);

            let sub_1 = vsubq_f64(x_ix8.1, base_jx4.1);
            let add_1 = vaddq_f64(c_jx4.1, y_i_1);
            y_i_1 = vmulq_f64(sub_1, add_1);

            let sub_1 = vsubq_f64(x_ix8.1, base_jx4.2);
            let add_1 = vaddq_f64(c_jx4.2, y_i_1);
            y_i_1 = vmulq_f64(sub_1, add_1);

            let sub_1 = vsubq_f64(x_ix8.1, base_jx4.3);
            let add_1 = vaddq_f64(c_jx4.3, y_i_1);
            if j + 4 < c.len() {
                y_i_1 = vmulq_f64(sub_1, add_1);
            } else {
                y_i_1 = add_1;
            }

            // calc xi_2 with 4 elements c and b, (xi_0-b3)((xi_0-b2)((xi_0-b1)((xi_0-b0)c0+c1)+c2)+c3)
            let sub_2 = vsubq_f64(x_ix8.2, base_jx4.0);
            let add_2 = vaddq_f64(c_jx4.0, y_i_2);
            y_i_2 = vmulq_f64(sub_2, add_2);

            let sub_2 = vsubq_f64(x_ix8.2, base_jx4.1);
            let add_2 = vaddq_f64(c_jx4.1, y_i_2);
            y_i_2 = vmulq_f64(sub_2, add_2);

            let sub_2 = vsubq_f64(x_ix8.2, base_jx4.2);
            let add_2 = vaddq_f64(c_jx4.2, y_i_2);
            y_i_2 = vmulq_f64(sub_2, add_2);

            let sub_2 = vsubq_f64(x_ix8.2, base_jx4.3);
            let add_2 = vaddq_f64(c_jx4.3, y_i_2);
            if j + 4 < c.len() {
                y_i_2 = vmulq_f64(sub_2, add_2);
            } else {
                y_i_2 = add_2;
            }

            // calc xi_3 with 4 elements c and b, (xi_0-b3)((xi_0-b2)((xi_0-b1)((xi_0-b0)c0+c1)+c2)+c3)
            let sub_3 = vsubq_f64(x_ix8.3, base_jx4.0);
            let add_3 = vaddq_f64(c_jx4.0, y_i_3);
            y_i_3 = vmulq_f64(sub_3, add_3);

            let sub_3 = vsubq_f64(x_ix8.3, base_jx4.1);
            let add_3 = vaddq_f64(c_jx4.1, y_i_3);
            y_i_3 = vmulq_f64(sub_3, add_3);

            let sub_3 = vsubq_f64(x_ix8.3, base_jx4.2);
            let add_3 = vaddq_f64(c_jx4.2, y_i_3);
            y_i_3 = vmulq_f64(sub_3, add_3);

            let sub_3 = vsubq_f64(x_ix8.3, base_jx4.3);
            let add_3 = vaddq_f64(c_jx4.3, y_i_3);
            if j + 4 < c.len() {
                y_i_3 = vmulq_f64(sub_3, add_3);
            } else {
                y_i_3 = add_3;
            }
        });

        let y = float64x2x4_t(y_i_0, y_i_1, y_i_2, y_i_3);
        let ptr = result.as_mut_ptr().add(i);
        vst1q_f64_x4(ptr, y);
    }
}
