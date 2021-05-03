// 注意必须至少有一个元素。
// 速度与内存占用超过100% 于 2021.05.04 07:46

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let re = nums.iter()
            .fold((0, 0, 0, 0),
                |(mut min, mut max, mut re, mut sum), num| {
                    sum += num;
                    if sum < min {
                        min = sum;
                    }
                    if sum - min > re {
                        re = sum - min;
                    }
                    (min, max, re, sum)
                })
            .2;
        if re == 0 {
            *nums.iter().max().unwrap()
        } else {
            re
        }
    }
}
