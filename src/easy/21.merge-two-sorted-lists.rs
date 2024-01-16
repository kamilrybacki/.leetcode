/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

// @lc code=start

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        pub fn unpack_node(current_node: Option<Box<ListNode>>) -> Box<ListNode> {
          match current_node {
            Some(node) => node,
            None => Box::new(ListNode::new(
              -101
            ))
          }
        }
        let mut new_list = ListNode::new(-101);
        loop {
          let current_nodes: (Box<ListNode>, Box<ListNode>) = (
            unpack_node(list1), unpack_node(list2)
          );
        }
    }
}
// @lc code=end

