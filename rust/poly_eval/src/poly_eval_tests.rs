use super::poly_eval;

#[test]
fn test_ploy_eval_x_number_less_8() {
    let c = vec![1.0, 2.0, 3.0];
    let x = vec![1.0, 2.0, 3.0];
    let result = poly_eval(&c, &x, None);
    assert_eq!(result, vec![6.0, 17.0, 34.0]);
}

#[test]
fn test_ploy_eval_x_number_equal_8() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let b = vec![1.0, 1.0, 1.0];
    let result = poly_eval(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![1.0, 10.0, 49.0, 142.0, 313.0, 586.0, 985.0, 1534.0]
    );
}

#[test]
fn test_ploy_eval_x_number_equal_16() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.,
    ];
    let b = vec![1.0, 1.0, 1.0];
    let result = poly_eval(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![
            1.0, 10.0, 49.0, 142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0, 3178.0, 4321.0, 5710.0,
            7369.0, 9322.0, 11593.0, 14206.0
        ]
    );
}

#[test]
fn test_ploy_eval_x_number_larger_8_no_aligned() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let b = vec![1.0, 1.0, 1.0];
    let result = poly_eval(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![1.0, 10.0, 49.0, 142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0, 3178.0]
    );
}

#[test]
fn test_ploy_eval_x_number_larger_8_no_aligned_with_diff_b() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![
        3.0, 4.0, 5.0, 6.0, 7.0, 8., 9.0, 10.0, 11.0, 12., 13.0, 14.0, 15.0, 16.0, 17.0,
    ];
    let b = vec![3.0, 4.0, 5.0];
    let result = poly_eval(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![
            1.0, 3.0, 11.0, 49.0, 141.0, 311.0, 583.0, 981.0, 1529.0, 2251.0, 3171.0, 4313.0,
            5701.0, 7359.0, 9311.0,
        ]
    );
}
