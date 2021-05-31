pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();
    let mut j = 1;
    loop {
        let mut strs_iter = strs.iter();
        let tmp = match strs_iter.next() {
            Some(s) => s,
            None => return result
        };
        loop {
            let i = match strs_iter.next() {
                Some(s) => s,
                None => break
            };
            if match tmp.get(0..j) {
                Some(s) => s,
                None => return result
            } != match i.get(0..j) {
                Some(s) => s,
                None => return result
            } {
                return result;
            }
        }
        result = match tmp.get(0..j) {
            Some(s) => s,
            None => return result
        }.parse().unwrap();
        j += 1;
    }
}