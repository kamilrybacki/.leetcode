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
    pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      fn traverse(mut node: Option<Rc<RefCell<TreeNode>>>, mut last_seen: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values: Vec<i32> = Vec::new();
        println!("Current: {:?}", node);
        if node.is_some() {
            match node.clone().unwrap().borrow().left.clone() {
              Some(left_node) => {
                values.append(&mut traverse(Some(left_node), node));
              },
              None => match node.clone().unwrap().borrow().right.clone() {
                Some(right_node) => {
                  values.append(&mut traverse(Some(right_node), node));
                },
                None => {
                  values.push(node.clone().unwrap().borrow().val.clone());
                  values.append(&mut traverse(last_seen, None));
                }
              }
            }
        };
        values
      }
      traverse(root.clone(), root.clone())
    }
}
// @lc code=end

