/**
多项式求值

根据给定的多项式系数和自变量值，计算多项式的值。
多项式形如：`c0 + (x-b0)(c1 + (x-b1)(c2 + (x-b2)(c3 + ...)))`，其中`c`为多项式系数，`b`为多项式基点（插值点），`x`为自变量.
如果目标平台支持SIMD指令集（如x86、x86_64、aarch64），并且编译器开启了AVX2、SSE2或NEON特性，
则使用SIMD加速的算法进行计算；否则，使用普通的算法进行计算。

# 参数

- `c`: 多项式的系数，按照从高到低的顺序排列。
- `x`: 自变量的值。
- `b`: 可选参数，多项式的基点（插值点）。如果提供了基点（插值点），则使用提供的基点（插值点）进行计算；
否则，使用默认的基点（插值点）（0.0）进行计算。基点个数必须小于等于多项式系数个数减1，不足的自动以0补充，
多余多项式系数个数减1则会`panic`。

# 返回值

计算得到的多项式值。

# 示例

```
use ploy_eval::poly_eval;
let mut c = vec![1.0, 2.0, 3.0]; // 多项式系数为 [1.0, 2.0, 3.0]
let x = vec![1.0, 2.0, 3.0]; // 自变量值为 [1.0, 2.0, 3.0]
let result = poly_eval(&c, &x, None); // 计算多项式的值
println!("{:?}", result); // 输出计算结果
```
*/
#[allow(dead_code)]
pub fn poly_eval(c: &Vec<f64>, x: &Vec<f64>, b: Option<&Vec<f64>>) -> Vec<f64> {
    if c.len() == 0 || x.len() == 0 {
        // if no c or no x, return empty
        return vec![];
    }

    if cfg!(all(
        any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64"
        ),
        any(
            target_feature = "avx2",
            target_feature = "sse2",
            target_feature = "neon",
        )
    )) {
        if x.len() >= 8 {
            // use simd only when x length >= 8
            let remain = x.len() % 8;
            if remain != 0 {
                // if x length % 8 != 0, then use no simd for the last few elements
                let result1 = poly_eval_simd(c, &x[0..x.len() - remain].to_vec(), b);
                let result2 = poly_eval_no_simd(c, &x[x.len() - remain..].to_vec(), b);
                return [&result1[..], &result2[..]].concat();
            } else {
                return poly_eval_simd(c, x, b);
            }
        } else {
            // if x length < 8, then use no simd
            return poly_eval_no_simd(c, x, b);
        }
    } else {
        poly_eval_no_simd(c, x, b)
    }
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    any(
        target_feature = "avx2",
        target_feature = "sse2",
        target_feature = "neon",
    )
))]
#[inline]
pub(crate) fn poly_eval_simd(c: &Vec<f64>, x: &Vec<f64>, b: Option<&Vec<f64>>) -> Vec<f64> {
    if c.len() == 0 {
        // if no c, return empty
        return vec![];
    }
    // reverse c
    let mut c = c.to_vec();
    if c.len() % 4 != 0 {
        // make sure c is 4x aligned
        c.resize(c.len() + 4 - c.len() % 4, 0.0);
    }
    c.reverse();

    // if b is not provided, use default
    let mut base: Vec<f64>;
    if let Some(b) = b {
        // if b is provided, use it
        base = b.to_vec();
        if base.len() >= c.len() {
            panic!("total basis numbers must be less than coefficients' numbers-1.");
        }
        if base.len() < c.len() - 1 {
            // base size must equal to c size and 4x aligned
            base.resize(c.len() - 1, 0.0);
        }
        // reverse base
        base.reverse();
        base.push(0.0);
    } else {
        base = vec![0.0; c.len()];
    }

    let mut result: Vec<f64> = vec![0.0; x.len()];
    #[cfg(target_arch = "aarch64")]
    use std::arch::aarch64::*;
    #[cfg(target_arch = "x86")]
    use std::arch::x86::float64x2x4_t;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::float64x2x4_t;

    unsafe {
        // each time process 4 elements
        for i in (0..x.len()).step_by(8) {
            // load 4 x to reg
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
    result
}

#[inline]
pub(crate) fn poly_eval_no_simd(c: &Vec<f64>, x: &Vec<f64>, b: Option<&Vec<f64>>) -> Vec<f64> {
    // if no c, return empty
    if c.len() == 0 {
        return vec![];
    }

    let mut base: Vec<f64>;
    if let Some(b) = b {
        base = b.to_vec();
        if base.len() >= c.len() {
            panic!("total basis numbers must be less than coefficients' numbers-1.");
        }
        if base.len() < c.len() - 1 {
            base.resize(c.len() - 1, 0.0);
        }
    } else {
        base = vec![0.0; c.len() - 1];
    }

    let mut result = vec![];
    for x_i in x {
        let mut y_i = 0.0;
        c.iter()
            .rev()
            .zip(base.iter().rev())
            .for_each(|(c_i, b_i)| {
                y_i = (x_i - b_i) * (c_i + y_i);
            });
        y_i = y_i + c[0];
        result.push(y_i);
    }
    result
}
