/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit_index in (0..digits.len()).rev() {
            if digit_index == digits.len() - 1 {
                digits[digit_index] += 1;
            }
            if digits[digit_index] >= 10 {
                digits[digit_index] %= 10;
                match digit_index {
                    0 => {
                        digits.splice(0..0, vec![1].iter().cloned());
                    }
                    _ => {
                        digits[digit_index - 1] += 1;
                    }
                }
            }
        }
        digits
    }
}
// @lc code=end
