impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        for i in (0..nums.len()).rev() {
            if nums[i] == val {
                nums.swap_remove(i);
            }
        }
        nums.len() as i32
    }
}
// 0ms 100%
// 1.9MiB 97&