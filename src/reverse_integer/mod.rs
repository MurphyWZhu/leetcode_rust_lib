pub fn reverse(x: i32) -> i32 {
    let mut x: i64 = x as i64;
    let mut result: i64 = 0;
    let mut symbol = 1;
    if x < 0 {
        symbol = -1;
        x = -x;
    }
    while x != 0 {
        result = result * 10 + x % 10;
        x /= 10;
    }
    result *= symbol;
    if result > 2147483647 || result < -2147483648 {
        return 0;
    } else {
        result as i32
    }
}