/*

给定一个整数数组，判断是否存在重复元素。

如果存在一值在数组中出现至少两次，函数返回 true 。如果数组中每个元素都不相同，则返回 false 。

示例 3:

输入: [1,1,1,3,3,4,3,2,4,2]
输出: true

https://leetcode-cn.com/leetbook/read/top-interview-questions-easy/x248f5/

*/


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
