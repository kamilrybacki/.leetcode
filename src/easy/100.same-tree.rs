/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn compare_nodes(
            node_1: Option<Rc<RefCell<TreeNode>>>,
            node_2: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (node_1, node_2) {
                (None, None) => true,
                (Some(_), None) | (None, Some(_)) => false,
                (Some(first), Some(second)) => {
                    return first.borrow().val == second.val
                        && compare_nodes(first.borrow().left.clone(), second.borrow().left.clone())
                        && compare_nodes(
                            first.borrow().right.clone(),
                            second.borrow().right.clone(),
                        )
                }
            }
        }
        compare_nodes(p, q)
    }
}
// @lc code=end
