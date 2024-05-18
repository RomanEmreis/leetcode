/*
  You are given the root of a binary tree with n nodes where each node in the tree has node.val coins. There are n coins in total throughout the whole tree.
  In one move, we may choose two adjacent nodes and move one coin from one node to another. A move may be from parent to child, or from child to parent.

  Return the minimum number of moves required to make every node have exactly one coin.

  Example 1:
    Input: root = [3,0,0]
    Output: 2
    Explanation: From the root of the tree, we move one coin to its left child, and one coin to its right child.
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
    public int DistributeCoins(TreeNode root, TreeNode parent = null) {
        if (root == null) return 0;

        int result = DistributeCoins(root.left, root) + DistributeCoins(root.right, root);
        int reminder = root.val - 1;
        if (parent != null) parent.val += reminder;

        result += Math.Abs(reminder);

        return result;
    }
}
