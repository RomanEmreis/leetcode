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
    public ListNode InsertGreatestCommonDivisors(ListNode head) {
        ListNode current = head;
        while (current?.next != null) {
            int gcd = Gcd(current.val, current.next.val);
            
            var temp = current.next;
            current.next = new ListNode(gcd, current.next);
            current = temp;
        }
        return head;
    }

    private static int Gcd(int a, int b)
    {
        while (b != 0) {
            int temp = b;
            b = a % b;
            a = temp;
        }

        return a;
    }
}
