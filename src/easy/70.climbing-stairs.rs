/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        use std::collections::HashMap;
        let mut memo: HashMap<i32, i32> = HashMap::new();
        fn calculate_combinations(steps: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            match memo.get(&steps) {
                Some(entry) => *entry,
                None => {
                    let result = match steps {
                        x => match x < 0 {
                            true => 0,
                            false => match x > 2 {
                                true => {
                                    calculate_combinations(steps - 1, memo)
                                        + calculate_combinations(steps - 2, memo)
                                }
                                false => x,
                            },
                        },
                    };
                    memo.insert(steps, result);
                    result
                }
            }
        }
        calculate_combinations(n, &mut memo)
    }
}
// @lc code=end
