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
            match branch {
                None => shift - 1,
                Some(valid_root) => {
                    let minimum_depth = match (
                        valid_root.borrow().left.clone(),
                        valid_root.borrow().right.clone(),
                    ) {
                        (None, None) => shift,
                        (Some(left), Some(right)) => {
                            branch_min_depth(Some(left.clone()), shift + 1)
                                .min(branch_min_depth(Some(right.clone()), shift + 1))
                        }
                        (Some(single_branch), None) | (None, Some(single_branch)) => {
                            branch_min_depth(Some(single_branch.clone()), shift + 1)
                        }
                    };
                    minimum_depth
                }
            }
        }
        branch_min_depth(root, 1)
    }
}
// @lc code=end
