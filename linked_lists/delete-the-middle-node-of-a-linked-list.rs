/*
  2095. Delete the Middle Node of a Linked List
  
  You are given the head of a linked list. Delete the middle node, and return the head of the modified linked list.
  The middle node of a linked list of size n is the ⌊n / 2⌋th node from the start using 0-based indexing, where ⌊x⌋ denotes the largest integer less than or equal to x.
      For n = 1, 2, 3, 4, and 5, the middle nodes are 0, 1, 1, 2, and 2, respectively.
  
  Example 1:
  Input: head = [1,3,4,7,1,2,6]
  Output: [1,3,4,1,2,6]
  Explanation:
  The above figure represents the given linked list. The indices of the nodes are written below.
  Since n = 7, node 3 with value 7 is the middle node, which is marked in red.
  We return the new list after removing this node. 
  
  Example 2:
  Input: head = [1,2,3,4]
  Output: [1,2,4]
  Explanation:
  The above figure represents the given linked list.
  For n = 4, node 2 with value 3 is the middle node, which is marked in red.
  
  Example 3:
  Input: head = [2,1]
  Output: [2]
  Explanation:
  The above figure represents the given linked list.
  For n = 2, node 1 with value 1 is the middle node, which is marked in red.
  Node 0 with value 2 is the only node remaining after removing node 1.
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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            len += 1;
            cur = &node.next;
        }

        if len <= 1 {
            return None;
        }

        let mid = len / 2;

        let mut cur = &mut head;
        for _ in 0..mid {
            cur = &mut cur.as_mut().unwrap().next;
        }
        
        let next = cur.take().unwrap().next;
        *cur = next;

        head       
    }
}
