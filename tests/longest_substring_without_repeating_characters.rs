use leetcode_rust_lib::*;

#[test]
fn length_of_longest_substring() {
    let s = String::from("abcabcbb");
    assert_eq!(3, longest_substring_without_repeating_characters::length_of_longest_substring(s));
}