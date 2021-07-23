impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut re = 0;
        let mut vote = 0;
        for &num in nums.iter() {
            if vote == 0 {
                re = num;
            }
            
            if re == num {
                vote += 1;
            } else {
                vote -= 1;
            }
        }
        re
    }
}
