/*
 * @lc app=leetcode id=136 lang=rust
 *
 * [136] Single Number
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut seen: HashSet<i32> = HashSet::new();
        for number in nums {
            if seen.contains(&number) {
                seen.remove(&number);
                continue;
            }
            seen.insert(number);
        }
        *seen.iter().nth(0).unwrap()
    }
}
// @lc code=end
