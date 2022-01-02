impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if let Some(re) = s.split_whitespace().last() {
                re.len() as i32
        } else {
            0
        }
    }
}
