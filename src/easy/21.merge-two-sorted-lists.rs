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

use std::cmp::Ordering;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: Vec<Option<Box<ListNode>>> = Vec::new();
        while list1 != None {
            match list2 {
                Some(ref list2_node) => match list1.clone().unwrap().val.cmp(&list2_node.val) {
                    Ordering::Greater | Ordering::Equal => {
                        result.push(list2.clone());
                        list2 = match list2 {
                            Some(head) => head.next,
                            None => None,
                        };
                    }
                    Ordering::Less => {
                        result.push(list1.clone());
                        list1 = match list1 {
                            Some(head) => head.next,
                            None => None,
                        }
                    }
                },
                None => {
                    result.push(list1.clone());
                    list1 = match list1 {
                        Some(head) => head.next,
                        None => None,
                    }
                }
            }
        }
        while list2 != None {
            result.push(list2.clone());
            list2 = match list2 {
                Some(head) => head.next,
                None => None,
            };
        }
        for index in (1..result.len()).rev() {
            result[index - 1].as_mut().unwrap().next = result[index].clone();
        }
        match result.first() {
            Some(node) => node.clone(),
            None => None,
        }
    }
}
// @lc code=end
