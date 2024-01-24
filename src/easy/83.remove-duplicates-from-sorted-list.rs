/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

use std::ops::Deref;

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn delete_duplicate(current_node: &mut Box<ListNode>) -> () {
            println!("{:?}", current_node);
            match current_node.next.clone() {
                Some(ref mut child_node) => {
                    while child_node.val == current_node.val {
                      *child_node = child_node.next.as_ref().unwrap().clone();
                    }
                    current_node.next = Some(child_node.clone());
                    if current_node.next != None {
                        delete_duplicate(current_node.next.as_mut().unwrap());
                    }
                }
                None => {
                    println!("Child node is None!");
                }
            };
        }
        match head {
            None => None,
            _ => {
                delete_duplicate(&mut head.as_mut().unwrap());
                head
            }
        }
    }
}
// @lc code=end
