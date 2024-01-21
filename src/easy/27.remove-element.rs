/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */


// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        use std::mem::swap;
        match nums.len() {
          0 => 0,
          1 => (nums[0] != val) as i32,
          _ => {
            let mut right_pointer: usize = nums.len() - 1;
            let mut left_pointer: usize = 0;
            while right_pointer != left_pointer {
              if nums[left_pointer] == val {
                while nums[right_pointer] == val {
                  right_pointer -= 1;
                  if left_pointer == right_pointer {
                    return left_pointer as i32;
                  }
                  if right_pointer == 0 {
                    return 0;
                  }
                }
                nums.swap(left_pointer, right_pointer);
              }
              left_pointer += 1;
            };
            if nums[left_pointer] != val {
              left_pointer += 1;
            }
            left_pointer as i32
          }
        }
    }
}
// @lc code=end
