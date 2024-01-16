/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
      for first_pointer in 0..nums.len() {
        for second_pointer in first_pointer + 1..nums.len() {
          if nums[first_pointer] + nums[second_pointer] == target {
            return Vec::from([
              first_pointer as i32,
              second_pointer as i32
            ])
          }
        }
      }
      Vec::new()
    }
}
// @lc code=end
