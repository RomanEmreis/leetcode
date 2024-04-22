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
    public ListNode MergeInBetween(ListNode list1, int a, int b, ListNode list2) {
        ListNode current = list1;

        for (int i = 0; i < a - 1; ++i) current = current.next;
        ListNode tail1 = current;

        for (int i = a - 1; i < b + 1; ++i) current = current.next;
        ListNode tail2 = current;

        tail1.next = list2;
        current = list2;
        while (current.next != null) current = current.next;
        current.next = tail2;

        return list1;
    }
}
