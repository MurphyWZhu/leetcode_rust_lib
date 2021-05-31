use leetcode_rust_lib::*;

#[test]
fn longest_common_prefix() {
    assert_eq!("".to_string(), longest_common_prefix::longest_common_prefix(vec!["".to_string()]));
}