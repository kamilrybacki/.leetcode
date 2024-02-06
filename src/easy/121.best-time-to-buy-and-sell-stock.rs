/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        fn search_for_max_profit(slice: Vec<i32>) -> i32 {
            let mut indices: HashSet<(usize, usize)> = HashSet::new();
            let mut max_profit = 0;
            for current_day in 0..(slice.len() - 1) {
                let mut forward_pointer = current_day + 1;
                let mut backwards_pointer = slice.len() - 1;
                while forward_pointer <= backwards_pointer {
                    if !indices.contains(&(current_day, forward_pointer)) {
                        max_profit = max_profit.max(slice[forward_pointer] - slice[current_day]);
                        indices.insert((current_day, forward_pointer));
                    }
                    if !indices.contains(&(current_day, backwards_pointer)) {
                        max_profit = max_profit.max(slice[backwards_pointer] - slice[current_day]);
                        indices.insert((current_day, backwards_pointer));
                    }
                    if !indices.contains(&(forward_pointer, backwards_pointer)) {
                        max_profit =
                            max_profit.max(slice[backwards_pointer] - slice[forward_pointer]);
                        indices.insert((forward_pointer, backwards_pointer));
                    }
                    forward_pointer += 1;
                    backwards_pointer -= 1;
                }
            }
            max_profit
        }
        match prices.len() {
            1 => 0,
            2 => 0.max(prices[1] - prices[0]),
            _ => search_for_max_profit(prices),
        }
    }
}
// @lc code=end
