/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

// @lc code=start

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let length_difference = a.len() as i32 - b.len() as i32;
        if length_difference != 0 {
            match length_difference > 0 {
                true => {
                    b = "0".repeat(length_difference.abs() as usize).to_owned() + b.as_str();
                }
                false => {
                    a = "0".repeat(length_difference.abs() as usize).to_owned() + a.as_str();
                }
            };
        }
        let mut carry_over = "0";
        for index in (0..a.len()).rev() {
            a.replace_range(
                index..index + 1,
                match format!(
                    "{}{}{}",
                    a[index..index + 1].to_owned(),
                    b[index..index + 1].to_owned(),
                    carry_over
                )
                .as_str()
                {
                    "110" | "101" | "011" => {
                        carry_over = "1";
                        "0"
                    }
                    "010" | "100" | "001" => {
                        carry_over = "0";
                        "1"
                    }
                    "111" => {
                        carry_over = "1";
                        "1"
                    }
                    "000" => {
                        carry_over = "0";
                        "0"
                    }
                    _ => {
                        panic!("Unsupported option!")
                    }
                },
            );
        }
        if carry_over == "1" {
            a = [carry_over.to_owned(), a[..].to_owned()].concat()
        }
        a
    }
}
// @lc code=end
