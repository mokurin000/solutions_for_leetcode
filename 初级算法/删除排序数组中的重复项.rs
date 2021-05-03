impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let result = nums.iter()
            .fold((0, Vec::new(), None),
                |(count, mut out, last), num| {
                    if let Some(last_num) = last {
                        if last_num == *num {
                            (count, out, last)
                        } else {
                            out.push(*num);
                            (count + 1, out, Some(*num))
                        }
                    } else {
                        out.push(*num);
                        (count + 1, out, Some(*num))
                    }
                }
            );

        nums.clone_from(&result.1);
        result.0
    }
}

/*

给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。

不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。

提示：

    0 <= nums.length <= 3 * 104
    -104 <= nums[i] <= 104
    nums 已按升序排列

作者：力扣 (LeetCode)
链接：https://leetcode-cn.com/leetbook/read/top-interview-questions-easy/x2gy9m/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

*/
