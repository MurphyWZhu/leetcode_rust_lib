pub fn my_atoi(s: String) -> i32 {
    let mut space = false;
    let mut result = 0;
    let mut symbol = 1;
    let mut tmp: i32;
    for c in s.chars() {
        match c {
            ' ' => {
                if space {
                    break;
                }
            }
            '-' => {
                if space {
                    break;
                }
                space = true;
                symbol = -1;
            }
            '+' => {
                if space {
                    break;
                }
                space = true;
            }
            '0'..='9' => {
                space = true;
                tmp = c as i32 - '0' as i32;
                if symbol == 1 {
                    if result > 214748364 || (result == 214748364 && tmp >= 7) {
                        return 2147483647;
                    }
                } else {
                    if result > 214748364 || (result == 214748364 && tmp >= 8) {
                        return -2147483648;
                    }
                }
                result = result * 10 + tmp;
            }
            _ => {
                if space {
                    break;
                } else {
                    return 0;
                }
            }
        }
    }
    result * symbol
}