impl Solution {
    fn zeros(n: i32) -> i32 {
        (1u32..).map(|i|n / 5u64.pow(i))
            .take_while(|&r| r != 0)
            .sum()
    }
}
