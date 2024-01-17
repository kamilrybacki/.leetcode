/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut list1_head = list1.clone();
      let list2_head = list2.clone();
      loop {
        list1_head = list1_head.unwrap().next;
        if list1_head.unwrap().val <= list2_head.unwrap().val {
          list1_head.unwrap().next = list2_head;
          list2_head = list2_head.unwrap().next;
        }
        if list1_head == None && list2_head == None {
          break;
        }
      }
      list1
    }
}
// @lc code=end

