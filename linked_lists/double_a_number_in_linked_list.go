/*
  You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.
  Return the head of the linked list after doubling it.

  Input: head = [1,8,9]
  Output: [3,7,8]
  Explanation: The figure above corresponds to the given linked list which represents the number 189. Hence, the returned linked list represents the number 189 * 2 = 378.
*/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func doubleIt(head *ListNode) *ListNode {
    carry := doubleVal(head);
    if carry != 0 {
        return &ListNode{carry, head};
    }

    return head;
}

func doubleVal(head *ListNode) int {
    if head == nil {
        return 0;
    }

    carry := doubleVal(head.Next);

    doubledVal := head.Val * 2;
    reminder := doubledVal % 10;

    head.Val = reminder + carry;

    return (doubledVal - reminder) / 10;
}
