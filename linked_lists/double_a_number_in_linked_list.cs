/*
  You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.
  Return the head of the linked list after doubling it.

  Input: head = [1,8,9]
  Output: [3,7,8]
  Explanation: The figure above corresponds to the given linked list which represents the number 189. Hence, the returned linked list represents the number 189 * 2 = 378.
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
    public ListNode DoubleIt(ListNode head) {
        int carry = Double(head);
        if (carry != 0) {
            return new(carry, head);
        }

        return head;
    }

    private static int Double(ListNode head) {
        if (head == null) return 0;

        int carry = Double(head.next);

        int doubledLastVal = head.val * 2;
        int reminder = doubledLastVal % 10;

        head.val = reminder + carry;

        return (doubledLastVal - reminder) / 10;
    }
}
