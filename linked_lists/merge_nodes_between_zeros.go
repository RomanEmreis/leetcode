/*
  You are given the head of a linked list, which contains a series of integers separated by 0's. The beginning and end of the linked list will have Node.val == 0.
  For every two consecutive 0's, merge all the nodes lying in between them into a single node whose value is the sum of all the merged nodes. The modified list should not contain any 0's.

  Return the head of the modified linked list.

  Example:
    Input: head = [0,3,1,0,4,5,2,0]
    Output: [4,11]
    Explanation: 
    The above figure represents the given linked list. The modified list contains
    - The sum of the nodes marked in green: 3 + 1 = 4.
    - The sum of the nodes marked in red: 4 + 5 + 2 = 11.
*/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeNodes(head *ListNode) *ListNode {
    current, tail := head.Next, head;
    for  current != nil {
        tail.Val += current.Val;
        current = current.Next;

        if current == nil { break; }

        if current.Val == 0 {
            if current.Next == nil {
                tail.Next = nil;
            } else {
                tail.Next = current;
            }

            tail = current;
            current = current.Next;
        }
    }

    return head;
}
