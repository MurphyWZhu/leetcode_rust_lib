use leetcode_rust_lib::*;

#[test]
fn my_atoi() {
    let s = String::from("42");
    assert_eq!(42, string_to_integer_atoi::my_atoi(s));
}