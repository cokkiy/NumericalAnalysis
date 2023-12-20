/// Implements the Newton's method for finding the root of a function.
///
/// # Arguments
///
/// * `f` - The function for which the root needs to be found.
/// * `f_prime` - The derivative of the function `f`.
/// * `initial_guess` - The initial guess for the root.
/// * `tolerance` - The tolerance value for convergence.
///
/// # Returns
///
/// The estimated root of the function.
pub fn newton_method(
    f: fn(f64) -> f64,
    f_prime: fn(f64) -> f64,
    initial_guess: f64,
    tolerance: f64,
    max_steps: usize,
) -> Option<f64> {
    let mut x = initial_guess;
    for _ in 0..max_steps {
        let fx = f(x);
        let fpx = f_prime(x);
        if fpx == 0.0 {
            return None;
        }
        let delta_x = fx / fpx;
        x -= delta_x;
        if delta_x.abs() < tolerance {
            return Some(x);
        }
    }
    None
}

/// Implements the improved Newton's method for finding the multiple roots of order `m` of a function.
///
/// # Arguments
///
/// * `f` - The function for which the root needs to be found.
/// * `f_prime` - The derivative of the function `f`.
/// * `m` - The multiple root order.
/// * `initial_guess` - The initial guess for the root.
/// * `tolerance` - The tolerance value for convergence.
///
/// # Returns
///
/// The estimated root of the function.
pub fn improved_newton_method(
    f: fn(f64) -> f64,
    f_prime: fn(f64) -> f64,
    m: usize,
    initial_guess: f64,
    tolerance: f64,
    max_steps: usize,
) -> Option<f64> {
    let mut x = initial_guess;
    for _ in 0..max_steps {
        let fx = f(x);
        let fpx = f_prime(x);
        if fpx == 0.0 {
            return None;
        }
        let delta_x = (m as f64) * fx / fpx;
        x -= delta_x;
        if delta_x.abs() < tolerance {
            return Some(x);
        }
    }
    None
}
