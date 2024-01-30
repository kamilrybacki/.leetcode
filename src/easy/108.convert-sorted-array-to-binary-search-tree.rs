/*
 * @lc app=leetcode id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
 */

use core::num;
use std::borrow::BorrowMut;
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
        fn make_branch(
            parent_node: &mut Option<Rc<RefCell<TreeNode>>>,
            vals: &mut Vec<i32>,
        ) -> () {
            println!("{:?} | {:?}", parent_node, vals);
            if parent_node.is_some() {

                let middle_point = vals.len() / 2;
                parent_node.clone().unwrap().clone().borrow_mut().left = match middle_point - 1 >= 0 {
                  true => Some(Rc::new(
                    RefCell::new(TreeNode::new(vals[middle_point - 1])),
                  )),
                  false => None
                };
                parent_node.clone().unwrap().clone().borrow_mut().right = match middle_point + 1 < vals.len() {
                  true => Some(Rc::new(
                    RefCell::new(TreeNode::new(vals[middle_point + 1])),
                  )),
                  false => None
                };
                vals.drain(middle_point - 1..middle_point + 2);
                make_branch(
                    &mut parent_node.clone().unwrap().clone().borrow_mut().left,
                    &mut vals,
                );
                make_branch(
                    &mut parent_node.clone().unwrap().clone().borrow_mut().right,
                    &mut vals,
                );
            }
        }
        match nums.len() {
            0 => None,
            _ => {
                let root = TreeNode::new(nums[nums.len() / 2]);
                let mut root_node = Some(Rc::new(RefCell::new(root)));
                make_branch(&mut root_node, &mut nums);
                root_node
            }
        }
    }
}
// @lc code=end
