impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        if strs.len() == 0 {
            return "".into();
        }

        let max = strs.iter().max().unwrap();
        let min = strs.iter().min().unwrap();

        let max = max.bytes();
        let min = min.bytes();

        let mut re = String::with_capacity(strs.len()/2);
        for (a, b) in max.zip(min).take_while(|(a,b)|a==b) {
            re.push(a as char);
        }
        re
    }
} 
