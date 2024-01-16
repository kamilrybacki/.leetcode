/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() == 1 {
            return false;
        }
        use std::collections::HashMap;
        let pairs: HashMap<String, String> = HashMap::from([
            (")".to_string(), "(".to_string()),
            ("}".to_string(), "{".to_string()),
            ("]".to_string(), "[".to_string()),
        ]);
        let mut stack: Vec<String> = Vec::new();
        for bracket in s.chars() {
            match pairs.get(&bracket.to_string()) {
                Some(opening_bracket) => match stack.last() {
                    Some(last_registered_bracket) => {
                        if last_registered_bracket == opening_bracket {
                            stack.pop();
                        } else {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                None => stack.push(bracket.to_string()),
            }
        }
        stack.len() == 0
    }
}
// @lc code=end
