use crate::poly_eval::poly_eval_no_simd;

#[test]
fn test_poly_eval_no_simd() {
    let c = vec![1.0, 2.0, 3.0];
    let x = vec![1.0, 2.0, 3.0];
    let result = poly_eval_no_simd(&c, &x, None);
    assert_eq!(result, vec![6.0, 17.0, 34.0]);
}

#[test]
fn test_poly_eval_no_simd_with_only_one_x() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![5.0];
    let result = poly_eval_no_simd(&c, &x, None);
    assert_eq!(result, vec![586.0]);
}

#[test]
fn test_poly_eval_no_simd_with_only_one_c() {
    let c = vec![1.0];
    let x = vec![5.0, 6.0, 7.0];
    let result = poly_eval_no_simd(&c, &x, None);
    assert_eq!(result, vec![1.0, 1.0, 1.0]);
}

#[test]
fn test_poly_eval_no_simd_with_empty_c() {
    let c = vec![];
    let x = vec![5.0, 6.0, 7.0];
    let result = poly_eval_no_simd(&c, &x, None);
    assert_eq!(result, vec![]);
}

#[test]
fn test_poly_eval_no_simd_with_empty_x() {
    let c = vec![1.0, 2.0, 3.0];
    let x = vec![];
    let result = poly_eval_no_simd(&c, &x, None);
    assert_eq!(result, vec![]);
}

#[test]
fn test_poly_eval_no_simd_with_empty_c_and_x() {
    let c = vec![];
    let x = vec![];
    let result = poly_eval_no_simd(&c, &x, None);
    assert_eq!(result, vec![]);
}

#[test]
fn test_poly_eval_no_simd_with_b() {
    let c = vec![1.0, 2.0, 3.0];
    let x = vec![1.0, 2.0, 3.0];
    let b = vec![1.0, 2.0];
    let result = poly_eval_no_simd(&c, &x, Some(&b));
    assert_eq!(result, vec![1.0, 3.0, 11.0]);
}

#[test]
#[should_panic]
fn test_poly_eval_no_simd_should_panic() {
    let c = vec![1.0, 2.0, 3.0];
    let x = vec![1.0, 2.0, 3.0];
    let b = vec![1.0, 2.0, 3.0, 4.0];
    poly_eval_no_simd(&c, &x, Some(&b));
}

#[test]
#[should_panic]
fn test_poly_eval_no_simd_should_panic_when_b_length_equal_c() {
    let c = vec![1.0, 2.0, 3.0];
    let x = vec![1.0, 2.0, 3.0];
    let b = vec![1.0, 2.0, 4.0];
    poly_eval_no_simd(&c, &x, Some(&b));
}

#[test]
fn test_poly_eval_no_simd_with_less_b() {
    let c = vec![1.0, 2.0, 3.0, 5.0];
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let b = vec![3.0];
    let result = poly_eval_no_simd(&c, &x, Some(&b));
    assert_eq!(result, vec![-19.0, -27.0, 1.0, 95.0]);
}

#[test]
fn test_poly_eval_no_simd_with_full_b() {
    let c = vec![1.0, 2.0, 3.0, 5.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 8.0];
    let b = vec![3.0, 4.0, 5.0];
    let result = poly_eval_no_simd(&c, &x, Some(&b));
    assert_eq!(result, vec![-105.0, -25.0, 1.0, 3.0, 11.0, 371.0]);
}

#[test]
fn test_poly_eval_no_simd_with_cx8() {
    let c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let result = poly_eval_no_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![36.0, 1793.0, 24604.0, 167481.0, 756836.0, 2620201.0, 7526268.0, 18831569.]
    );
}
