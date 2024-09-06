/*
  You are given an array of integers nums and the head of a linked list. Return the head of the modified linked list after removing all nodes from the linked list that have a value that exists in nums.
   
  Example 1:
    Input: nums = [1,2,3], head = [1,2,3,4,5]
    Output: [4,5]
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
    public ListNode ModifiedList(int[] nums, ListNode head) {
        HashSet<int> toRemove = [.. nums];

        ListNode guard = new(0, head);
        ListNode current = guard;

        while (current != null && current.next != null) {
            if (toRemove.Contains(current.next.val)) {
                current.next = current.next.next;
            } else {
                current = current.next;
            }
        }

        return guard.next;
    }
}
