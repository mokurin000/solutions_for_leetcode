impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut re = Vec::with_capacity(nums.len()*2);
        re.extend(nums.iter());
        re.extend(nums.into_iter());
        re
    }
}
