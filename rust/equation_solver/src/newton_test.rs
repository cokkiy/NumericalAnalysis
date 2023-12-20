use crate::newton::{improved_newton_method, newton_method};

#[test]
fn test_newton_method() {
    // Define the function and its derivative
    fn f(x: f64) -> f64 {
        x.powi(2) - 4.0
    }

    fn f_prime(x: f64) -> f64 {
        2.0 * x
    }

    // Test case 1: Find the root of f(x) = x^2 - 4 using initial guess 1.5, tolerance 0.001, and maximum steps 100
    let root1 = newton_method(f, f_prime, 1.5, 0.001, 100);
    assert!(root1.is_some());
    assert!((root1.unwrap() - 2.0).abs() < 0.001);

    // Test case 2: Find the root of f(x) = x^2 - 4 using initial guess -1.5, tolerance 0.001, and maximum steps 100
    let root2 = newton_method(f, f_prime, -1.5, 0.001, 100);
    assert!(root2.is_some());
    assert!((root2.unwrap() + 2.0).abs() < 0.001);
}

#[test]
fn test_improved_newton_method() {
    // Define the function and its derivative
    fn f(x: f64) -> f64 {
        x.sin() + x.powi(2) * x.cos() - x.powi(2) - x
    }

    fn f_prime(x: f64) -> f64 {
        x.cos() + 2.0 * x * x.cos() - x.powi(2) * x.sin() - 2.0 * x - 1.0
    }

    let tol = 0.5e-6;

    // Test case 1: Find the root of f(x) = x^3 - 8 with multiple root order 1, initial guess 1.0, and tolerance 0.5e-6
    let root1 = improved_newton_method(f, f_prime, 1, 1.0, tol, 30);
    assert!(root1.is_none());

    // Test case 2: Find the root of f(x) = x^3 - 8 with multiple root order 3, initial guess 1.0, and tolerance 0.5e-6
    let root2 = improved_newton_method(f, f_prime, 3, 1.0, tol, 10);
    assert!(root2.is_some());
    assert!((root2.unwrap() - 0.0).abs() < tol);
}

#[test]
fn test_newton_method_is_none() {
    fn f(x: f64) -> f64 {
        4.0 * x.powi(3) - 6.0 * x.powi(2) - 11.0 / 4.0
    }

    fn f_prime(x: f64) -> f64 {
        12.0 * x.powi(2) - 12.0 * x
    }

    let root = newton_method(f, f_prime, 0.5, 0.00001, 100);
    assert!(root.is_none());
}
