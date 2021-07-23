impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        if x == 0 { return true; }
        if x % 10 == 0 { return false; }

        let reversed = {
            let mut temp = 0;
            let mut x = x;
            while x != 0 {
                temp = x % 10 + temp * 10;
                x /= 10;
            }
            temp
        };

        reversed == x
    }
}
