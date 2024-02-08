/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy_price = prices[0];
        let mut max_profit = 0;
        for index in 1..prices.len() {
            if prices[index] < buy_price {
                buy_price = prices[index];
                continue;
            }
            max_profit = max_profit.max(prices[index] - buy_price);
        }
        max_profit
    }
}
// @lc code=end
