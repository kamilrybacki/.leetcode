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
          let first_number = match first_number_index < 0 {
            true => i32::MIN,
            false => nums1[first_number_index as usize].clone()
          };
          let second_number = match second_number_index < 0 {
            true => i32::MIN,
            false => nums2[second_number_index as usize].clone()
          };
          match first_number.cmp(&second_number) {
            Ordering::Equal | Ordering::Greater => {
              nums1[right_side_pointer as usize] = first_number;
              first_number_index -= 1;
            },
            Ordering::Less => {
              nums1[right_side_pointer as usize] = second_number;
              second_number_index -= 1;
            }
          }
          right_side_pointer -= 1;
        }
    }
}
// @lc code=end

