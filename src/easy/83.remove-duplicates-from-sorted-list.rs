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
        fn delete_duplicate(mut current_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
          match current_head {
            Some(ref mut node) => {
              let mut next_node = node.next.clone();
              while next_node != None {
                println!("{:?} | {:?}", node, next_node);
                if next_node.clone().unwrap().val == node.clone().val {
                  node.as_mut().next = next_node.clone();
                }
                *node = node.next.clone().unwrap();
                next_node = next_node.clone().unwrap().next;
              }
            },
            None => {}
          };
          current_head
        }
        delete_duplicate(head.clone())
    }
}
// @lc code=end

