impl Solution {
    pub fn is_valid(ref s: String) -> bool {
        let mut filo = Vec::with_capacity(s.len() / 2);
        for ch in s.chars() {
            match ch {
                '{' => filo.push('}'),
                '(' => filo.push(')'),
                '[' => filo.push(']'),
                _ => if Some(ch) != filo.pop(){
                    return false
                }
            }
        }
        filo.is_empty()
    }
}
