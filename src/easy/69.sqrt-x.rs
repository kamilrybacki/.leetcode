/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut root: f64 = 0.0;
        let mut guess = x as f64;
        let tolerance = 0.1;
        loop {
            root = 0.5 * (guess + ((x as f64) / guess));
            if (root - guess).abs() < tolerance {
                break;
            }
            guess = root;
        }
        root as i32
    }
}
// @lc code=end
