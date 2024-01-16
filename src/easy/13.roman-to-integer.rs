/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let possible_combinations: HashMap<&str, i32> = HashMap::from([
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
            ("IV", 4),
            ("IX", 9),
            ("XL", 40),
            ("XC", 90),
            ("CD", 400),
            ("CM", 900),
        ]);
        let mut sum = 0;
        let mut skip = false;
        for mut index in 0..s.len() {
            if skip {
                skip = false;
                continue;
            }
            if index == s.len() - 1 {
                sum += possible_combinations.get(&s[index..]).unwrap();
            } else {
                sum += match possible_combinations.get(&s[index..index + 2]) {
                    Some(pattern_value) => {
                        skip = true;
                        pattern_value
                    }
                    None => possible_combinations.get(&s[index..index + 1]).unwrap(),
                };
            };
        }
        sum
    }
}
// @lc code=end
