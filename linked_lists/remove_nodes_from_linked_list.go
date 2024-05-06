/*
  You are given the head of a linked list.
  Remove every node which has a node with a greater value anywhere to the right side of it.
  Return the head of the modified linked list.

  Input: head = [5,2,13,3,8]
  Output: [13,8]
  Explanation: The nodes that should be removed are 5, 2 and 3.
  - Node 13 is to the right of node 5.
  - Node 13 is to the right of node 2.
  - Node 8 is to the right of node 3.
*/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func removeNodes(head *ListNode) *ListNode {
    if head.Next == nil {
        return head;
    }

    last := removeNodes(head.Next);
    if last.Val > head.Val {
        head.Next = nil;
        return last;
    } else {
        head.Next = last;
    }

    return head;
}
