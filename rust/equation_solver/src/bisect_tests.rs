use crate::bisect::bisect;
#[test]
fn bisect_test() {
    // Define the function f(x) = x^2 - 4
    let f = |x: f64| x.powi(2) - 4.0;

    // Test case 1: Root exists within the interval
    let a1 = 1.0;
    let b1 = 3.0;
    let tol1 = 0.0001;
    let result1 = bisect(f, a1, b1, tol1);
    assert!(result1.is_some());
    assert!((result1.unwrap() - 2.0).abs() < tol1);
}
#[test]
fn bisect_is_none_test() {
    // Define the function f(x) = x^2 - 4
    let f = |x: f64| x.powi(2) - 4.0;
    // Test case 2: Root does not exist within the interval
    let a2 = -1.5;
    let b2 = -1.0;
    let tol2 = 0.0001;
    let result2 = bisect(f, a2, b2, tol2);
    assert!(result2.is_none());
}

#[test]
fn bisect_cubic_equation_test() {
    // Define the function f(x) = x^3 - 2x^2 + 3x - 5
    let f = |x: f64| x.powi(3) - 2.0 * x.powi(2) + 3.0 * x - 5.0;

    // Test case 3: Root exists within the interval
    let a3 = 0.0;
    let b3 = 3.0;
    let tol3 = 0.00005;
    let result3 = bisect(f, a3, b3, tol3);
    assert!(result3.is_some());
    assert!((result3.unwrap() - 1.84373427789807).abs() < tol3);
}
