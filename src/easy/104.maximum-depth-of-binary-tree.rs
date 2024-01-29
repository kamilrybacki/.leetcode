/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn branch_depth(node: &Option<Rc<RefCell<TreeNode>>>, current_level: i32) -> Vec<i32> {
            let mut depths = Vec::new();
            match node {
                Some(present_node) => {
                    depths.append(&mut branch_depth(
                        &present_node.borrow().left,
                        current_level + 1,
                    ));
                    depths.append(&mut branch_depth(
                        &present_node.borrow().right,
                        current_level + 1,
                    ));
                }
                None => depths.push(current_level),
            }
            depths
        }
        branch_depth(&root, 0).into_iter().max().unwrap()
    }
}
// @lc code=end
