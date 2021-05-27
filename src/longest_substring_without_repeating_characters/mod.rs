pub fn length_of_longest_substring(s: String) -> i32 {
    let s_len = s.len();
    let mut tmp: Vec<&str> = Vec::new();
    let (mut result, mut left, mut right): (usize, usize, usize) = (0, 0, 0);
    let mut c;
    while right < s_len {
        c = match s.get(right..right + 1) {
            Some(c) => c,
            None => panic!()
        };
        if !tmp.contains(&c) {
            tmp.push(&c);
            right += 1;
            result = result.max(right - left);
        } else {
            while tmp[0] != c {
                tmp.remove(0);
                left += 1;
            }
            tmp.remove(0);
            left += 1;
        }
    }
    result as i32
}