/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut valid_backward_character_index = s.len();
        let mut characters: Vec<char> = s.chars().collect();
        for valid_forward_character_index in 0..characters.len() - 1 / 2 {
            if !characters[valid_forward_character_index].is_alphanumeric() {
                continue;
            }
            valid_backward_character_index -= 1;
            while !characters[valid_backward_character_index].is_alphanumeric() {
                valid_backward_character_index -= 1;
            }
            if characters[valid_forward_character_index].to_ascii_lowercase()
                != characters[valid_backward_character_index].to_ascii_lowercase()
            {
                return false;
            }
        }
        true
    }
}
// @lc code=end
