/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn search_sum(node: Option<Rc<RefCell<TreeNode>>>, target: i32, sum: i32) -> bool {
            match node {
                None => sum == target,
                Some(existing_node) => {
                    match (
                        existing_node.borrow().left.clone(),
                        existing_node.borrow().right.clone(),
                    ) {
                        (None, None) => sum == target,
                        (Some(branch), None) | (None, Some(branch)) => {
                            let new_sum = sum + branch.borrow().val.clone();
                            search_sum(Some(branch), target, new_sum)
                        }
                        (Some(left), Some(right)) => {
                            let left_sum = sum + left.borrow().val.clone();
                            let right_sum = sum + right.borrow().val.clone();
                            search_sum(Some(left), target, left_sum)
                                || search_sum(Some(right), target, right_sum)
                        }
                    }
                }
            }
        }
        match root {
            None => false,
            Some(valid_root) => search_sum(
                Some(valid_root.clone()),
                target_sum,
                valid_root.borrow().val.clone(),
            ),
        }
    }
}
// @lc code=end
