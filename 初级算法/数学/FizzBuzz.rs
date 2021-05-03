impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {

        fn fb(n: i32) -> String {
            match (n%3, n%5) {
                (0,0) => "FizzBuzz".to_string(),
                (0,_) => "Fizz".to_string(),
                (_,0) => "Buzz".to_string(),
                (_,_) => n.to_string(),
            }
        }
        
        (1..=n).map(|n| fb(n)).collect()
    }
}
