impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        if strs.len() == 0 {
            return "".into();
        }

        let max = strs.iter().max().unwrap();
        let min = strs.iter().min().unwrap();

        let max = max.chars();
        let min = min.chars();

        let mut re = String::new();
        for (a, b) in max.zip(min) {
            if a == b {
                re.push(a);
            } else {
                return re;
            }
        }
        re
    }
} 
