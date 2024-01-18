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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
      use std::cmp::Ordering;
      let mut list1_head = list1.clone();
      let mut list2_head = list2.clone();
      let mut results: Vec<Option<Box<ListNode>>> = Vec::new();
      while list1_head != None && list2_head != None {
        let mut new_node = Box::new(ListNode::new(0));
        match list1_head.clone().unwrap().val.cmp(&list2_head.clone().unwrap().val) {
          Ordering::Less | Ordering::Equal => {
            new_node.val = list1_head.clone().unwrap().val;
            new_node.next = Some(Box::new(ListNode::new(
              list2_head.clone().unwrap().val
            )));
          }
          Ordering::Greater => {
            new_node.val = list2_head.clone().unwrap().val;
            new_node.next = Some(Box::new(ListNode::new(
              list1_head.clone().unwrap().val
            )));
          }
        }
        println!("{:?}", new_node);
        results.push(Some(new_node));
        list1_head = list1_head.unwrap().next;
        list2_head = list2_head.unwrap().next;
      }
      results.iter().fold(results.first().unwrap(), |acc, value| {
        acc.unwrap().next = *value;
        acc
      })
    }
}
// @lc code=end
