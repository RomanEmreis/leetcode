/*
  Given a binary tree root and a linked list with head as the first node. 
  Return True if all the elements in the linked list starting from the head correspond to some downward path connected in the binary tree otherwise return False.
  
  In this context downward path means a path that starts at some node and goes downwards.

  Example 1:
    Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
    Output: true
    Explanation: Nodes in blue form a subpath in the binary Tree. 
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
/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     public int val;
 *     public TreeNode left;
 *     public TreeNode right;
 *     public TreeNode(int val=0, TreeNode left=null, TreeNode right=null) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
public class Solution {
    public bool IsSubPath(ListNode head, TreeNode root) {
        if (root == null) return false;

        return Dfs(head, root) || IsSubPath(head, root.left) ||IsSubPath(head, root.right);
    }

    private bool Dfs(ListNode head, TreeNode root) {
        if (head == null) return true;
        if (root == null) return false;

        return head.val == root.val && (Dfs(head.next, root.left) || Dfs(head.next, root.right));
    }
}
