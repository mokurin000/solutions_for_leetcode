impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut count = 0;
        while n >= 5 {
            n /= 5;
            count += n;
        }
        count
    }
}
