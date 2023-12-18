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
use poly_eval::poly_eval;
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
            // base size must equal to c size - 1
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
    {
        use crate::aarch64_neon::neon_eval;
        unsafe {
            neon_eval(x, c, base, &mut result);
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        use crate::x86_64::avx2_eval;
        use crate::x86_64::avx512_eval;
        use crate::x86_64::sse2_eval;
        unsafe {
            if is_x86_feature_detected!("avx512f") {
                avx512_eval(x, c, base, &mut result);
            } else if is_x86_feature_detected!("avx2") {
                avx2_eval(x, c, base, &mut result);
            } else if is_x86_feature_detected!("sse3") {
                sse2_eval(x, c, base, &mut result);
            }
        }
    }

    #[cfg(target_arch = "x86")]
    {
        use crate::x86::sse2_eval;
        unsafe {
            if is_x86_feature_detected!("sse3") {
                sse2_eval(x, c, base, &mut result);
            } else {
                ploy_eval_no_simd(x, c, base, &mut result);
            }
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
