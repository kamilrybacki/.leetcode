/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
 */

use std::borrow::Borrow;
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
use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn measure_branch(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(existing_node) => {
                    1 + measure_branch(existing_node.borrow().left.clone())
                        .max(measure_branch(existing_node.borrow().right.clone()))
                }
            }
        }
        match root {
            None => 0,
            Some(valid_root) => {
                1 + match (
                    measure_branch(valid_root.borrow().left.clone()),
                    measure_branch(valid_root.borrow().right.clone()),
                ) {
                    (0, h) | (h, 0) => h,
                    (h_left, h_right) => h_left.min(h_right),
                }
            }
        }
    }
}
// @lc code=end
