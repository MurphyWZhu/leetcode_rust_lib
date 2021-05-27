pub fn max_area(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    let (mut tmp_l, mut tmp_r): (i32, i32);
    while left < right {
        tmp_l = height[left];
        tmp_r = height[right];
        result = result.max((right - left) as i32 * tmp_l.min(tmp_r));
        if tmp_l < tmp_r {
            left += 1;
        } else {
            right -= 1;
        }
    }
    result
}