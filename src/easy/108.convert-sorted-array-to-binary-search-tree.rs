/*
 * @lc app=leetcode id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
      use std::cmp::Ordering;
        fn make_branch(
            parent_node: &mut Option<Rc<RefCell<TreeNode>>>,
            branch_vals: &mut Vec<i32>,
        ) -> () {
            if parent_node.is_some() {
                match branch_vals.len() {
                    0 => (),
                    1 => {
                        parent_node.as_mut().unwrap().borrow_mut().right =
                            Some(Rc::new(RefCell::new(TreeNode::new(branch_vals[0]))));
                    }
                    _ => {
                        let middle_point = branch_vals.len() / 2;
                        let children_values = match branch_vals[middle_point - 1].cmp(&branch_vals[middle_point+1]) {
                          Ordering::Less => (
                            branch_vals[middle_point-1],
                            branch_vals[middle_point+1]
                          ),
                          Ordering::Greater => (
                            branch_vals[middle_point+1],
                            branch_vals[middle_point-1]
                          ),
                          Ordering::Equal => (
                            branch_vals[middle_point+1],
                            branch_vals[middle_point+1],
                          )
                        };
                        parent_node.unwrap().as_mut().left = Some(
                          Rc::new(
                            RefCell::new(
                              TreeNode::new(
                                children_values.0
                              )
                            )
                          )
                        );
                        make_branch(
                            &mut parent_node.unwrap().as_mut().left,
                            &mut Vec::from(branch_vals[0..middle_point].to_owned()),
                        );
                        make_branch(
                            &mut parent_node.unwrap().as_mut().right,
                            &mut Vec::from(branch_vals[middle_point + 1..].to_owned()),
                        );
                    }
                };
            }
        }
        match nums.len() {
            0 => None,
            _ => {
                let root = TreeNode::new();
                let mut root_node = Some(Rc::new(RefCell::new(root)));
                make_branch(&mut root_node, &mut Vec::from(nums));
                root_node
            }
        }
    }
}
// @lc code=end
