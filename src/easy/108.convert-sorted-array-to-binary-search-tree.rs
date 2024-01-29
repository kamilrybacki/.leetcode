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
                let parent_val = parent_node.as_ref().unwrap().borrow().val;
                for _ in 0..2 {
                    match branch_vals.pop() {
                        None => break,
                        Some(leaf_value) => match leaf_value.cmp(&parent_val) {
                            Ordering::Less => {
                                parent_node.as_mut().unwrap().borrow_mut().left =
                                    Some(Rc::new(RefCell::new(TreeNode::new(leaf_value))));
                            }
                            Ordering::Greater => {
                                parent_node.as_mut().unwrap().borrow_mut().right =
                                    Some(Rc::new(RefCell::new(TreeNode::new(leaf_value))));
                            }
                            Ordering::Equal => panic!("Values should be unique!"),
                        },
                    }
                }
                make_branch(
                    &mut parent_node.as_mut().unwrap().borrow_mut().left,
                    branch_vals,
                );
                make_branch(
                    &mut parent_node.as_mut().unwrap().borrow_mut().right,
                    branch_vals,
                );
            }
        }
        match nums.len() {
            0 => None,
            _ => {
                let root = TreeNode::new(nums[0]);
                let mut root_node = Some(Rc::new(RefCell::new(root)));
                make_branch(&mut root_node, &mut Vec::from(nums));
                root_node
            }
        }
    }
}
// @lc code=end
