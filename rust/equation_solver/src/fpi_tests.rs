use crate::fpi::fixed_point_iter;

#[test]
fn fpi_cosx_test() {
    let f = |x: f64| x.cos();
    let tol = 0.00001;
    let r = fixed_point_iter(f, 0.0, 0.00001, 50);
    assert!(r.is_some());
    assert!(r.unwrap() - 0.7390851332151607 < tol)
}

#[test]
fn fpi_x2_test() {
    let f = |x: f64| x * x;
    let tol = 0.00001;
    let r = fixed_point_iter(f, 0.5, tol, 50);
    assert!(r.is_some());
    assert!(r.unwrap() - 1.0 < tol)
}

#[test]
fn fpi_1_minus_x3_test() {
    let f = |x: f64| 1.0 - x.powi(3);
    let tol = 0.00001;
    let r = fixed_point_iter(f, 0.5, tol, 20);
    assert!(r.is_none());
}

#[test]
fn fpi_1_minus_x_test() {
    let f = |x: f64| (1.0 - x).powf(1.0 / 3.0);
    let tol = 0.00001;
    let r = fixed_point_iter(f, 0.5, tol, 50);
    assert!(r.is_some());
    assert!(r.unwrap() - 0.68236807 < tol);
}
