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
        let mut uniques: Vec<i32> = Vec::new();
        while head != None {
            if !uniques.contains(&head.as_ref().unwrap().val) {
                uniques.push(head.as_ref().unwrap().val);
            }
            head = head.unwrap().next;
        }
        head = None;
        for value in uniques.iter().rev() {
            let mut new_node = Box::new(ListNode::new(*value));
            new_node.next = head.clone();
            head = Some(new_node)
        }
        head
    }
}
// @lc code=end
