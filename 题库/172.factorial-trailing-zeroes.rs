impl Solution {
    fn trailing_zeroes(n: i32) -> i32 {
        (1u32..).map(|i|n / 5i32.pow(i))
            .take_while(|&r| r != 0)
            .sum()
    }
}
