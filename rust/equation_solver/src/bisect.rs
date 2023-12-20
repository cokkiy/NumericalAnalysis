/// Performs the bisection method to find the root of a function within a given interval.
///
/// The bisection method is an iterative numerical method that repeatedly bisects an interval
/// and selects a sub-interval in which a root of the function is guaranteed to exist.
///
/// # Arguments
///
/// * `f` - The function for which the root is to be found.
/// * `a` - The lower bound of the interval.
/// * `b` - The upper bound of the interval.
/// * `tol` - The tolerance value that determines the accuracy of the root approximation.
///
/// # Returns
///
/// * `Some(c)` - The approximate root of the function within the specified tolerance.
/// * `None` - If the function does not have a root within the given interval.
pub fn bisect(f: impl Fn(f64) -> f64, a: f64, b: f64, tol: f64) -> Option<f64> {
    let mut a = a;
    let mut b = b;
    let mut c = (a + b) / 2.0;

    if f(a) * f(b) > 0.0 {
        return None;
    }

    while (b - a) / 2.0 > tol {
        if f(c) == 0.0 {
            return Some(c);
        } else if f(a) * f(c) < 0.0 {
            b = c;
        } else {
            a = c;
        }
        c = (a + b) / 2.0;
    }

    Some(c)
}
