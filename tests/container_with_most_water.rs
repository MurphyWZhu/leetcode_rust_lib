use leetcode_rust_lib::*;

#[test]
fn max_area() {
    assert_eq!(49, container_with_most_water::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    assert_eq!(1, container_with_most_water::max_area(vec![1, 1]));
    assert_eq!(16, container_with_most_water::max_area(vec![4, 3, 2, 1, 4]));
}