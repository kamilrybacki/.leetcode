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
            None => true,
            Some(root_node) => {
                Self::is_balanced(root_node.borrow().left.clone())
                    && Self::is_balanced(root_node.borrow().right.clone())
                    && (measure_branch(root_node.borrow().left.clone())
                        - measure_branch(root_node.borrow().right.clone()))
                    .abs()
                        <= 1
            }
        }
    }
}
// @lc code=end
