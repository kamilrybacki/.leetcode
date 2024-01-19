/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.iter().fold(nums.len() as i32, |mut acc, x| {
            if *x == val {
                acc -= 1;
            }
            acc
        })
    }
}
// @lc code=end
