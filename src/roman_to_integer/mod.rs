pub fn roman_to_int(s: String) -> i32 {
    let mut s = s;
    let mut result = 0;
    s = s.replace("IV", "a");
    s = s.replace("IX", "b");
    s = s.replace("XL", "c");
    s = s.replace("XC", "d");
    s = s.replace("CD", "e");
    s = s.replace("CM", "f");
    for i in s.chars() {
        match i {
            'a' => result += 4,
            'b' => result += 9,
            'c' => result += 40,
            'd' => result += 90,
            'e' => result += 400,
            'f' => result += 900,
            'I' => result += 1,
            'V' => result += 5,
            'X' => result += 10,
            'L' => result += 50,
            'C' => result += 100,
            'D' => result += 500,
            'M' => result += 1000,
            _ => {}
        }
    }
    result
}