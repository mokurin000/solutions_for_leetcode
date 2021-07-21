impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() { return 0 }
        if let Some(i) = haystack.match_indices(&needle).next() {
            i.0 as i32
        } else {
            -1
        }
    }
}