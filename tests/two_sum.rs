use leetcode_rust_lib::*;

#[test]
fn two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(vec![0, 1], two_sum::two_sum(nums, target));
}