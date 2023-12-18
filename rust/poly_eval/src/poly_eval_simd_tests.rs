use std::vec;

use crate::poly_eval::poly_eval_simd;

#[test]
fn test_poly_eval_simd_8x() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![10.0, 49.0, 142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0]
    );
}

#[test]
fn test_poly_eval_simd_next_8x() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0, 3178.0, 4321.0]
    );
}

#[test]
fn test_poly_eval_simd_16x() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![
            10.0, 49.0, 142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0, 3178.0, 4321.0, 5710.0, 7369.0,
            9322.0, 11593.0, 14206.0, 17185.0
        ]
    );
}

#[test]
fn test_poly_eval_simd_next_16x() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![
        3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16., 17.0, 18.0,
    ];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![
            142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0, 3178.0, 4321.0, 5710.0, 7369.0, 9322.0,
            11593.0, 14206.0, 17185.0, 20554.0, 24337.0
        ]
    );
}

#[test]
fn test_poly_eval_simd_24x() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6., 7.0, 8.0, 9.0, 10., 11.0, 12.0, 13.0, 14.0, 15.0, 16., 17.0,
        18.0, 19.0, 20., 21.0, 22.0, 23.0, 24.,
    ];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![
            10.0, 49.0, 142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0, 3178.0, 4321.0, 5710.0, 7369.0,
            9322.0, 11593.0, 14206.0, 17185.0, 20554.0, 24337.0, 28558.0, 33241.0, 38410.0,
            44089.0, 50302.0, 57073.0
        ]
    );
}

#[test]
fn test_poly_eval_simd_with_cx8() {
    let c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![36.0, 1793.0, 24604.0, 167481.0, 756836.0, 2620201.0, 7526268.0, 18831569.]
    );
}

#[test]
fn test_poly_eval_simd_with_cx12() {
    let c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9., 10., 11., 12.];
    let x = vec![1.0; 8];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(result, vec![78.0, 78.0, 78.0, 78.0, 78.0, 78.0, 78.0, 78.0]);
}

#[test]
fn test_poly_eval_simd_zero_x() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![0.0; 8];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(result, vec![1.0; 8]);
}

#[test]
fn test_poly_eval_simd_with_b() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.,
    ];
    let b = vec![1.0, 1.0, 1.0];
    let result = poly_eval_simd(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![
            1.0, 10.0, 49.0, 142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0, 3178.0, 4321.0, 5710.0,
            7369.0, 9322.0, 11593.0, 14206.0
        ]
    );
}

#[test]
fn test_poly_eval_simd_with_diff_b() {
    let c = vec![1.0, 2.0, 3.0, 4.0];
    let x = vec![
        3.0, 4.0, 5.0, 6.0, 7.0, 8., 9.0, 10.0, 11.0, 12., 13.0, 14.0, 15.0, 16.0, 17.0, 18.,
    ];
    let b = vec![3.0, 4.0, 5.0];
    let result = poly_eval_simd(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![
            1.0, 3.0, 11.0, 49.0, 141.0, 311.0, 583.0, 981.0, 1529.0, 2251.0, 3171.0, 4313.0,
            5701.0, 7359.0, 9311.0, 11581.0,
        ]
    );
}

#[test]
fn test_poly_eval_simd_with_less_b() {
    let c = vec![1.0, 2.0, 3.0, 5.0];
    let x = vec![
        3.0, 4.0, 5.0, 6.0, 7.0, 8., 9.0, 10.0, 11.0, 12., 13.0, 14.0, 15.0, 16.0, 17.0, 18.,
    ];
    let b = vec![3.0];
    let result = poly_eval_simd(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![
            1.0, 95.0, 285.0, 601.0, 1073.0, 1731.0, 2605.0, 3725.0, 5121.0, 6823.0, 8861.0,
            11265.0, 14065.0, 17291.0, 20973.0, 25141.0,
        ]
    );
}

#[test]
fn test_ploy_eval_simd_c_no_4_align() {
    let c = vec![1.0, 2.0, 3.0];
    let x = vec![1.0; 8];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(result, vec![6.0; 8]);
}

#[test]
fn test_ploy_eval_simd_c_no_4_align2() {
    let c = vec![1.0, 2.0];
    let x = vec![1.0; 8];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(result, vec![3.0; 8]);
}

#[test]
fn test_ploy_eval_simd_c_no_4_align3() {
    let c = vec![1.0, 2.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(result, vec![3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0,]);
}

#[test]
fn test_ploy_eval_simd_c_no_4_align4() {
    let c = vec![1.0, 2.0, 3.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![6.0, 17.0, 34.0, 57.0, 86.0, 121.0, 162.0, 209.0,]
    );
}

#[test]
fn test_ploy_eval_simd_c_no_4_align5() {
    let c = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let x = vec![1.0; 8];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(result, vec![15.0; 8]);
}

#[test]
fn test_ploy_eval_simd_c_no_4_align6() {
    let c = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let x = vec![
        3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11., 12., 13., 14., 15.0, 16.0, 17.0, 20.0,
    ];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(
        result,
        vec![
            547.0, 1593.0, 3711.0, 7465.0, 13539.0, 22737.0, 35983.0, 54321.0, 78915.0, 111049.0,
            152127.0, 203673.0, 267331.0, 344865.0, 438159.0, 833241.0,
        ]
    );
}

#[test]
fn test_ploy_eval_simd_c_no_4_align_and_b() {
    let c = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let x = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8., 9.0, 10.0, 11.0, 12., 13.0, 14.0, 15.0, 16.0,
    ];
    let b = vec![3.0, 4.0, 5.0];
    let result = poly_eval_simd(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![
            -201.0, -79.0, 1.0, 3.0, 11.0, 229.0, 981.0, 2711.0, 5983.0, 11481.0, 20009.0, 32491.0,
            49971.0, 73613.0, 104701.0, 144639.0,
        ]
    );
}

#[test]
fn test_ploy_eval_simd_c_no_4_align_and_full_b() {
    let c = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let x = vec![
        3.0, 4.0, 5.0, 6.0, 7.0, 8., 9.0, 10.0, 11.0, 12., 13.0, 14.0, 15.0, 16.0, 17., 18.0,
    ];
    let b = vec![3.0, 4.0, 5.0, 6.0];
    let result = poly_eval_simd(&c, &x, Some(&b));
    assert_eq!(
        result,
        vec![
            1.0, 3.0, 11.0, 49.0, 261.0, 911.0, 2383.0, 5181.0, 9929.0, 17371.0, 28371.0, 43913.0,
            65101.0, 93159.0, 129431.0, 175381.0,
        ]
    );
}

#[test]
fn test_poly_eval_simd_with_empty_c() {
    let c = vec![];
    let x = vec![5.0, 6.0, 7.0];
    let result = poly_eval_simd(&c, &x, None);
    assert_eq!(result, vec![]);
}
