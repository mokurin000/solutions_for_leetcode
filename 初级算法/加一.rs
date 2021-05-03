impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut prev = true;

        for digit in digits.iter_mut().rev() {
            if prev {
                *digit += 1
            }
            if *digit > 9 {
                *digit -= 10;
            } else {
                prev = false;
            }
        }

        if prev {
            digits.insert(0, 1);
        }

        digits
    }
}
