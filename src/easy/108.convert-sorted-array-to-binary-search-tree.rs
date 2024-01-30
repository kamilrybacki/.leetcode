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
        fn branch_out(vals: &Vec<i32>) -> () {
            println!("{:?}", vals);
            let middle_index = vals.len() / 2;
            let root = TreeNode::new(vals[middle_index]);
            root.left
            Some(Rc::new(RefCell::new(root)))
        }
        match nums.len() {
            0 => None,
            _ => branch_out(&nums),
        }
    }
}
// @lc code=end
