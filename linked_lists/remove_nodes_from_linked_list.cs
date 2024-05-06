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
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int val=0, ListNode next=null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */
public class Solution {
    public ListNode RemoveNodes(ListNode head) {
        if (head.next == null) return head;

        Stack<ListNode> s = [];
        s.Push(head);

        ListNode current = head.next;
        while (current != null) {
            s.Push(current);
            current = current.next;
        }

        ListNode max = null;
        while (s.TryPop(out ListNode node)) {
            if (max == null || node.val >= max.val) {
                node.next = max;
                max = node;
            }
        }

        return max;
    }
}
