/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.clone().iter().enumerate().fold(nums.len() as i32, |mut acc, (index, number)| {
            if *number == val {
                acc -= 1;
                nums.push(*number);
                nums.remove(index);
            }
            acc
        })
    }
}
// @lc code=end
