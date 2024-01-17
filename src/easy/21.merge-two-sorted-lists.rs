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
        let mut list1_head = list1.clone();
        let mut list2_head = list2.clone();
        let mut nodes: Vec<Option<Box<ListNode>>> = Vec::new();
        loop {
            match (list1_head.clone(), list2_head.clone()) {
                (None, None) => break,
                (None, node2) => {
                    nodes.push(node2.clone());
                    list2_head = node2.unwrap().next;
                    continue;
                }
                (node1, None) => {
                  nodes.push(node1.clone());
                  list1_head = node1.unwrap().next;
                  continue;
                }
                (node1, node2) => {
                    nodes.push(node1.clone());
                    list1_head = node1.clone().unwrap().next;
                    if node1.clone().unwrap().val <= node2.clone().unwrap().val {
                        nodes.push(node2.clone());
                        list2_head = node2.unwrap().next;
                    }
                    continue;
                }
            }
        }

    }
}
// @lc code=end
