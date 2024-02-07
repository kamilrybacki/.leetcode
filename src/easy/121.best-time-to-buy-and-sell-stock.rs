/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut current_maximum = 0;
        fn calculate_differences(prices: Vec<i32>, mut cache: &i32) {
            if prices.len() <= 1 {
                return;
            }
            if prices.len() == 2 {
                let difference = prices[1] - prices[0];
                cache = cache.max(&difference.clone());
                return;
            }
            let mut second_forward_index = 0;
            let mut second_backward_index = prices.len();
            while second_forward_index <= second_backward_index {
                second_backward_index -= 1;
                let bigger_value = prices[second_backward_index].max(prices[second_forward_index]);
                let difference = bigger_value - prices[0];
                cache = cache.max(&difference.clone());
                println!("({}) {} - {}", bigger_value, prices[0], cache);
                second_forward_index += 1;
                calculate_differences(prices[second_forward_index..].to_vec(), cache);
                calculate_differences(prices[second_backward_index..].to_vec(), cache);
            }
        }
        calculate_differences(prices, &current_maximum);
        current_maximum
    }
}
// @lc code=end
