/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

use std::borrow::BorrowMut;
use std::ops::Deref;
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      fn traverse(node: Option<Rc<RefCell<TreeNode>>>, last_parent: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values: Vec<i32> = Vec::new();
        if node.is_none() {
          return vec![]
        }
        values.append(
          &mut traverse(
            node.as_ref().unwrap().borrow().left.clone(),
            node.clone()
          )
        );
        values.push(node.as_ref().unwrap().borrow().val);
        values.append(
          &mut traverse(
            node.as_ref().unwrap().borrow().right.clone(),
            node.clone()
          )
        );
        values
      }
      traverse(root, None)
    }
}
// @lc code=end

