/*
  61. Rotate List
  
  Given the head of a linked list, rotate the list to the right by k places.
  
  Example 1:
  Input: head = [1,2,3,4,5], k = 2
  Output: [4,5,1,2,3]
  
  Example 2:
  Input: head = [0,1,2], k = 4
  Output: [2,0,1]
*/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::collections::VecDeque;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {        
        let mut len = 0;
        let mut curr = head.as_ref();

        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }

        if len <= 1 {
            return head;
        }

        let k = k as usize % len;
        if k == 0 {
            return head;
        }

        let split_at = len - k;

        let mut curr = &mut head;
        for _ in 0..split_at - 1 {
            curr = &mut curr.as_mut().unwrap().next;
        }

        let mut new_head = curr.as_mut().unwrap().next.take();

        let mut tail = &mut new_head;
        while tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }

        tail.as_mut().unwrap().next = head;

        new_head
    }
}
