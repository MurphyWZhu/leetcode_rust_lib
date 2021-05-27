pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut tmp = vec![0, 0];
    let (nums1_len, nums2_len) = (nums1.len(), nums2.len());
    let median = (nums1_len + nums2_len) / 2;
    let (mut p1, mut p2): (usize, usize) = (0, 0);
    for _i in 0..median + 1 {
        match nums1.get(p1) {
            Some(n1) => {
                match nums2.get(p2) {
                    Some(n2) => {
                        if n2 > n1 {
                            tmp.push(n1.clone());
                            tmp.remove(0);
                            p1 += 1;
                        } else {
                            tmp.push(n2.clone());
                            tmp.remove(0);
                            p2 += 1;
                        }
                    }
                    None => {
                        tmp.push(n1.clone());
                        tmp.remove(0);
                        p1 += 1;
                    }
                }
            }
            None => {
                match nums2.get(p2) {
                    Some(n2) => {
                        tmp.push(n2.clone());
                        tmp.remove(0);
                        p2 += 1;
                    }
                    None => return 0.0
                }
            }
        }
    }
    if (nums1_len + nums2_len) % 2 != 0 {
        tmp[1] as f64
    } else {
        (tmp[0] as f64 + tmp[1] as f64) / 2.0
    }
}