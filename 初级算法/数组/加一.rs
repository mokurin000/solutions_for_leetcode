/*

给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。

最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。

你可以假设除了整数 0 之外，这个整数不会以零开头。

 

作者：力扣 (LeetCode)
链接：https://leetcode-cn.com/leetbook/read/top-interview-questions-easy/x2cv1c/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

示例 3：

输入：digits = [0]
输出：[1]

*/

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut prev = true;

        for digit in digits.iter_mut().rev() {
            if prev { // 前一项进位
                *digit += 1;
                if *digit > 9 { // 这一项进位
                    *digit -= 10;
                } else { // 否则prev为false
                    prev = false;
                }
            }
        }

        if prev {
            digits.insert(0, 1);
        }

        digits
    }
}
