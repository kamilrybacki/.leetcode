/*
 * @lc app=leetcode id=168 lang=rust
 *
 * [168] Excel Sheet Column Title
 */

// @lc code=start
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        use std::collections::HashMap;
        let quick_lookup_map: HashMap<i32, char> = HashMap::from([
            (0, 'Z'),
            (1, 'A'),
            (2, 'B'),
            (3, 'C'),
            (4, 'D'),
            (5, 'E'),
            (6, 'F'),
            (7, 'G'),
            (8, 'H'),
            (9, 'I'),
            (10, 'J'),
            (11, 'K'),
            (12, 'L'),
            (13, 'M'),
            (14, 'N'),
            (15, 'O'),
            (16, 'P'),
            (17, 'Q'),
            (18, 'R'),
            (19, 'S'),
            (20, 'T'),
            (21, 'U'),
            (22, 'V'),
            (23, 'W'),
            (24, 'X'),
            (25, 'Y'),
            (26, 'Z'),
        ]);
        let mut result = String::new();
        if column_number != 0 {
          result.push(
            *quick_lookup_map.get(&(column_number % 26)).unwrap()
          );
          let remaining_counter = column_number - (column_number % 26);
          if remaining_counter > 0 {
            result.insert_str(
              0,
              &Self::convert_to_title(remaining_counter / 26)
            );
          }
        }
        println!("{}: {:?}", column_number, result);
        result
    }
}
// @lc code=end
