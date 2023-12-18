use num_traits::pow;
use poly_eval::poly_eval;
fn main() {
    let coefficients = (0..=50).map(|_i| 1.0).collect::<Vec<_>>();
    let x = vec![1.00001];
    let result = poly_eval(&coefficients, &x, None);
    let dir_result = (pow(x[0], 51) - 1.0) / (x[0] - 1.0);
    println!(
        "Result: {}, Directly Result {}, Error {}",
        result[0],
        dir_result,
        result[0] - dir_result
    );

    let coefficients = (0..=99)
        .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
        .collect::<Vec<_>>();
    let x = vec![1.00001];
    let result = poly_eval(&coefficients, &x, None);
    let dir_result = 1.0 - pow(x[0], 99);
    println!(
        "Result: {}, Directly Result {}, Error {}",
        result[0],
        dir_result,
        result[0] - dir_result
    );
}
