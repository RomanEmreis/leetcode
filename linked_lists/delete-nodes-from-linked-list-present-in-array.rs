/*
  You are given an array of integers nums and the head of a linked list. Return the head of the modified linked list after removing all nodes from the linked list that have a value that exists in nums.
   
  Example 1:
    Input: nums = [1,2,3], head = [1,2,3,4,5]
    Output: [4,5]
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
use std::collections::HashSet;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let to_remove: HashSet<i32> = nums.into_iter().collect();

        let mut guard = ListNode { val: 0, next: head };
        let mut current = &mut guard;

        while let Some(nn) = current.next.as_mut() {
            if to_remove.contains(&nn.val) {
                current.next = nn.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        guard.next
    }
}
