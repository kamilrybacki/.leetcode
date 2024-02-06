/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = Vec::new();
        let mut counter = 0;
        while counter != num_rows {
            let new_row: Vec<i32> = match counter {
                0 => vec![1],
                1 => vec![1, 1],
                _ => {
                    let previous_row = triangle.last().unwrap();
                    let mut row: Vec<i32> = previous_row[0..previous_row.len() - 1]
                        .iter()
                        .enumerate()
                        .map(|(index, _)| previous_row[index] + previous_row[index + 1])
                        .collect();
                    row.insert(0, 1);
                    row.push(1);
                    row
                }
            };
            triangle.push(new_row);
            counter += 1;
        }
        triangle
    }
}
// @lc code=end
