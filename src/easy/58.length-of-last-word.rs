/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut last_word_count = 0;
        let mut counter = 0;
        s.chars().into_iter().enumerate().for_each(|(index, character)| {
            if !character.is_whitespace() {
                if index == s.len() - 1 {
                  last_word_count = counter + 1;
                };
                counter += 1;
            }
            if character.is_whitespace() && counter > 0 {
                last_word_count = counter;
                counter = 0;
            }
        });
        last_word_count
    }
}
// @lc code=end
