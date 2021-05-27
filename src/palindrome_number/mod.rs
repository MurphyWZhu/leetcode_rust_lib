pub fn is_palindrome(x: i32) -> bool {
    let mut t = x;
    let mut t_rev = 0;
    if x < 0 {
        return false;
    }
    while t != 0 {
        t_rev = t_rev * 10 + t % 10;
        t /= 10;
    }
    t_rev == x
}