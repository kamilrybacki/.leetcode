/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */


// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        use std::cmp::Ordering;
        use std::mem::swap;
        if m == 0 {
          swap(nums1, nums2);
          return
        }
        let mut second_number_index = n - 1;
        let mut first_number_index = m - 1;
        let mut right_side_pointer = n + m - 1;
        while right_side_pointer >= 0 {
          println!("{} | {} | {} ", first_number_index, right_side_pointer, second_number_index);
          if first_number_index < 0 {
            first_number_index = 0;
          }
          if second_number_index < 0 {
            second_number_index = 0;
          }
          match nums1[first_number_index as usize].cmp(
            &nums2[second_number_index as usize]
          ) {
            Ordering::Equal | Ordering::Greater => {
              nums1[right_side_pointer as usize] = nums1[first_number_index as usize].clone();
              first_number_index -= 1;
            },
            Ordering::Less => {
              nums1[right_side_pointer as usize] = nums2[second_number_index as usize].clone();
              second_number_index -= 1;
            }
          }
          right_side_pointer -= 1;
        }
    }
}
// @lc code=end

