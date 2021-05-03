impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        for nums in nums.windows(2) {
            if nums[0] == nums[1] {
                return true
            }
        }
        false
    }
}
      
// 似乎是因为不需要额外内存，比HashSet快许多
