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
