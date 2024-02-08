/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn traverse_branch(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut found_nodes: Vec<i32> = Vec::new();
            match node {
                None => (),
                Some(valid_node) => {
                    found_nodes.push(valid_node.borrow().val.clone());
                    found_nodes.extend(traverse_branch(valid_node.borrow().left.clone()));
                    found_nodes.extend(traverse_branch(valid_node.borrow().right.clone()));
                }
            }
            found_nodes
        }
        traverse_branch(root)
    }
}
// @lc code=end
