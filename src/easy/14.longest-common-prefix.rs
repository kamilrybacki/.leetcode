/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut index = 0;
        loop {
            let target_char = match strs[0].chars().nth(index) {
                Some(char) => char,
                None => break,
            };
            match strs[1..]
                .iter()
                .filter(|current_string| match current_string.chars().nth(index) {
                  Some(character) => character == target_char,
                  None => false
                })
                .count() == strs.len() - 1
            {
                false => break,
                true => index += 1,
            }
        }
        String::from(&strs[0][0..index])
    }
}
// @lc code=end
