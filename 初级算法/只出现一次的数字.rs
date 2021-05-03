use std::ops::BitXor;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(0, BitXor::bitxor)
    }
}
      
// 相较于闭包 内存占用更小……
