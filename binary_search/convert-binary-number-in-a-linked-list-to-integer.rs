/*
  1290. Convert Binary Number in a Linked List to Integer
  
  Given head which is a reference node to a singly-linked list. The value of each node in the linked list is either 0 or 1. The linked list holds the binary representation of a number.
  Return the decimal value of the number in the linked list.
  The most significant bit is at the head of the linked list.
  
  Example 1:
  Input: head = [1,0,1]
  Output: 5
  Explanation: (101) in base 2 = (5) in base 10
  
  Example 2:
  Input: head = [0]
  Output: 0
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
impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        while let Some(h) = head {
            result = (result << 1) | h.val;
            head = h.next;
        }
        result
    }
}
