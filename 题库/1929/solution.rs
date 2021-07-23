impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .chain(nums.into_iter())
            .map(|&n| n)
            .collect()
    }
}