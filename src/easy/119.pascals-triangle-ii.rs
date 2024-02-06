/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row: Vec<i32> = Vec::new();
        let mut counter = 0;
        while counter <= row_index {
            row = match counter {
                0 => vec![1],
                1 => vec![1, 1],
                _ => {
                    let cache = row.clone();
                    let mut new_row: Vec<i32> = cache[0..cache.len() - 1]
                        .iter()
                        .enumerate()
                        .map(|(index, _)| cache[index] + cache[index + 1])
                        .collect();
                    new_row.insert(0, 1);
                    new_row.push(1);
                    new_row
                }
            };
            counter += 1;
        }
        row
    }
}
// @lc code=end
