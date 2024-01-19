/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut uniques: Vec<i32> = Vec::new();
        nums.clone()
            .iter_mut()
            .enumerate()
            .for_each(|(index, number)| {
                if !uniques.contains(number) {
                    uniques.push(*number);
                }
            });
        let mut counter = 0;
        uniques.into_iter().enumerate().for_each(|(index, number)| {
            nums[index] = number;
            counter += 1;
        });
        counter
    }
}
// @lc code=end
