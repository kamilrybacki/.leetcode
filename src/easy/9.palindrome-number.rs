/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
      let mut digits = Vec::new();
      if x < 0 { return false; };
      let mut rest = x;
      while rest != 0 {
        digits.push(rest % 10);
        rest /= 10 as i32;
      }
      for index in 0..digits.len() / 2 as usize {
        if digits[index] != digits[digits.len() - 1 - index] { return false; }
      }
      true
    }
}
// @lc code=end
