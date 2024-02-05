/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
 */

use core::borrow;
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn branch_min_depth(branch: Option<Rc<RefCell<TreeNode>>>, shift: i32) -> i32 {
            println!("{:?}: {}", branch, shift);
            let mut minimal_depth = shift.clone();
            match branch {
                None => minimal_depth - 1,
                Some(node) => {
                    let left_depth = branch_min_depth(node.borrow().left.clone(), shift + 1);
                    let right_depth = branch_min_depth(node.borrow().right.clone(), shift + 1);
                    left_depth.min(right_depth)
                }
            }
        }
        match root {
            None => 0,
            Some(valid_root) => {
                let minimum_depth = match (
                    valid_root.borrow().left.clone(),
                    valid_root.borrow().right.clone(),
                ) {
                    (Some(_), Some(_)) => branch_min_depth(Some(valid_root.clone()), 1),
                    (None, None) => 1,
                    (Some(single_branch), None) | (None, Some(single_branch)) => {
                        branch_min_depth(Some(single_branch.clone()), 1)
                    }
                };
                minimum_depth
            }
        }
    }
}
// @lc code=end
