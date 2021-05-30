use leetcode_rust_lib::*;

#[test]
fn int_to_roman() {
    assert_eq!("III".to_string(), integer_to_roman::int_to_roman(3));
    assert_eq!("IV".to_string(), integer_to_roman::int_to_roman(4));
    assert_eq!("MCMXCIV".to_string(), integer_to_roman::int_to_roman(1994));
}