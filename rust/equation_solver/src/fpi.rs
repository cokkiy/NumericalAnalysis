/// Performs fixed-point iteration to find the root of a function.
///
/// # Arguments
///
/// * `f` - The function for which the root is to be found.
/// * `initial_guess` - The initial guess for the root.
/// * `tolerance` - The tolerance value for convergence.
/// * `max_iterations` - The maximum number of iterations to perform.
///
/// # Returns
///
/// * `Some(f64)` - The approximate root if convergence is achieved within the specified tolerance.
/// * `None` - If convergence is not achieved within the specified maximum number of iterations.
pub fn fixed_point_iter(
    f: impl Fn(f64) -> f64,
    initial_guess: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Option<f64> {
    let mut x = initial_guess;
    let mut prev_x = x;

    for _ in 0..max_iterations {
        x = f(x);

        if (x - prev_x).abs() < tolerance {
            return Some(x);
        }

        prev_x = x;
    }

    None
}
