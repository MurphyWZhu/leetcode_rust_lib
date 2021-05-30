pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut result = String::new();
    let int_v = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1, 0];
    let roman_v = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I", ""];
    let mut int_iter = int_v.iter();
    let mut roman_iter = roman_v.iter();
    let mut int_cur = match int_iter.next() {
        Some(b) => b,
        None => panic!()
    };
    let mut roman_curr = match roman_iter.next() {
        Some(&s) => s,
        None => panic!()
    };
    while int_cur != &0 {
        if num / int_cur == 0 {
            int_cur = match int_iter.next() {
                Some(b) => b,
                None => panic!()
            };
            roman_curr = match roman_iter.next() {
                Some(&s) => s,
                None => panic!()
            };
        } else {
            result.push_str(roman_curr);
            num -= int_cur;
        }
    }
    result
}