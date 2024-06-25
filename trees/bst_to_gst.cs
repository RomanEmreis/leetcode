/*
  Given the root of a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original BST is changed to the original key plus the sum of all keys greater than the original key in BST.
  As a reminder, a binary search tree is a tree that satisfies these constraints:

  The left subtree of a node contains only nodes with keys less than the node's key.
  The right subtree of a node contains only nodes with keys greater than the node's key.
  Both the left and right subtrees must also be binary search trees.

Example 1:
  Input: root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
  Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
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
    private int _sum = 0;
    public TreeNode BstToGst(TreeNode root) {
        if (root != null) {
            BstToGst(root.right);
            _sum += root.val;
            root.val = _sum;
            BstToGst(root.left);
        }

        return root;
    }
}
