/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::HashMap;
        fn measure_branch(node: &Option<Rc<RefCell<TreeNode>>>, last_height: i32) -> Vec<i32> {
            match node {
                Some(parent_node) => {
                    match (
                        node.as_ref().unwrap().borrow().left.clone(),
                        node.as_ref().unwrap().borrow().right.clone(),
                    ) {
                        (None, None) => vec![last_height, last_height],
                        (Some(left), None) => {
                            let mut left_height = measure_branch(&Some(left), last_height + 1);
                            left_height.push(last_height);
                            left_height
                        }
                        (None, Some(right)) => {
                            let mut right_height = measure_branch(&Some(right), last_height + 1);
                            right_height.insert(0, last_height);
                            right_height
                        }
                        (Some(left), Some(right)) => {
                            let mut left_height = measure_branch(&Some(left), last_height + 1);
                            let right_height = measure_branch(&Some(right), last_height + 1);
                            left_height.extend(right_height);
                            left_height
                        }
                    }
                }
                None => vec![last_height, last_height],
            }
        }
        let counts = measure_branch(&root, 0)
            .iter()
            .fold(HashMap::new(), |mut counters, value| {
                counters.insert(
                    *value,
                    match counters.get_key_value(value) {
                        Some((_, current_counter)) => current_counter + 1,
                        None => 1,
                    },
                );
                counters
            });
        counts.values().fold(0, |sum, count| (sum + count) as i32) / counts.len() as i32
            == *counts.values().nth(0).unwrap()
    }
}
// @lc code=end
