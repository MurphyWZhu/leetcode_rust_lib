use leetcode_rust_lib::*;

#[test]
fn roman_to_int() {
    assert_eq!(3, roman_to_integer::roman_to_int("III".to_string()));
    assert_eq!(9, roman_to_integer::roman_to_int("IX".to_string()));
    assert_eq!(58, roman_to_integer::roman_to_int("LVIII".to_string()));
    assert_eq!(1994, roman_to_integer::roman_to_int("MCMXCIV".to_string()));
}