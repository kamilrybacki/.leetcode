/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */


// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
      for index in 0..nums.len() {
        let difference = nums[index] - target;
        if difference >= 0 {
          return index as i32;
        }
      }
      nums.len() as i32
    }
}
// @lc code=end

