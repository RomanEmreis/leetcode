/*
  Given the head of a linked list head, in which each node contains an integer value.
  Between every pair of adjacent nodes, insert a new node with a value equal to the greatest common divisor of them.
  
  Return the linked list after insertion.
  
  The greatest common divisor of two numbers is the largest positive integer that evenly divides both numbers.
  
  Example 1:
    Input: head = [18,6,10,3]
    Output: [18,6,6,2,10,1,3]
    Explanation: The 1st diagram denotes the initial linked list and the 2nd diagram denotes the linked list after inserting the new nodes (nodes in blue are the inserted nodes).
    - We insert the greatest common divisor of 18 and 6 = 6 between the 1st and the 2nd nodes.
    - We insert the greatest common divisor of 6 and 10 = 2 between the 2nd and the 3rd nodes.
    - We insert the greatest common divisor of 10 and 3 = 1 between the 3rd and the 4th nodes.
    There are no more adjacent nodes, so we return the linked list.
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
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|node| {
            let next = node.next.and_then(|next| {
                let gcd = gcd(node.val, next.val);
                let new_next = Self::insert_greatest_common_divisors(Some(next));
                Some(Box::new(ListNode { val: gcd, next: new_next }))
            });

            Some(Box::new(ListNode { val: node.val, next }))
        })
    }
}

fn gcd(left: i32, right: i32) -> i32 {
    if left == 0 { return right; }
    if right == 0 { return left; }
    if left == right { return right; }

    if left > right {
        gcd(left - right, right)
    } else {
        gcd(left, right - left)
    }
}
