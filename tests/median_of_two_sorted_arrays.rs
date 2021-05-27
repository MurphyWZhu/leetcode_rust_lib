use leetcode_rust_lib::*;

#[test]
fn find_median_sorted_arrays() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(2 as f64, median_of_two_sorted_arrays::find_median_sorted_arrays(nums1, nums2));
}