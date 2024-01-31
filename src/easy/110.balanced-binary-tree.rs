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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn measure_branch(node: &Option<Rc<RefCell<TreeNode>>>, last_height: i32) -> i32 {
          match node {
            None => last_height,
            Some(existing_node) =>  {
              let left_height = measure_branch(&existing_node.borrow().left, last_height + 1);
              let right_height = measure_branch(&existing_node.borrow().right, last_height + 1);
              left_height - right_height
            }
          }
        }
        measure_branch(&root, 0) == 0
    }
}
// @lc code=end

