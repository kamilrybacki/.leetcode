/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check_branch(
            branch_1: &Option<Rc<RefCell<TreeNode>>>,
            branch_2: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (branch_1, branch_2) {
                (None, None) => true,
                (None, Some(_)) | (Some(_), None) => false,
                (Some(node_1), Some(node_2)) => {
                    node_1.as_ref().borrow().val == node_2.as_ref().borrow().val
                        && check_branch(
                            &node_1.as_ref().borrow().left,
                            &node_2.as_ref().borrow().right,
                        )
                        && check_branch(
                            &node_2.as_ref().borrow().left,
                            &node_1.as_ref().borrow().right,
                        )
                }
            }
        }

        let root_node = root.as_ref().unwrap().borrow();
        let left_root_node = &root_node.left;
        let right_root_node = &root_node.right;

        check_branch(&left_root_node, &right_root_node)
    }
}
// @lc code=end
