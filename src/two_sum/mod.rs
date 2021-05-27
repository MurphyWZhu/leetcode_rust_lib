pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for (i, v) in nums.iter().enumerate() {
        for (i1, v1) in nums.iter().enumerate() {
            if v + v1 == target && i != i1 {
                result.push(i as i32);
                result.push(i1 as i32);
                return result;
            }
        }
    }
    return result;
}