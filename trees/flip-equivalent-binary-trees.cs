/*
  For a binary tree T, we can define a flip operation as follows: choose any node, and swap the left and right child subtrees.
  A binary tree X is flip equivalent to a binary tree Y if and only if we can make X equal to Y after some number of flip operations.
  
  Given the roots of two binary trees root1 and root2, return true if the two trees are flip equivalent or false otherwise.
  
  Example 1:
    Flipped Trees Diagram
    Input: root1 = [1,2,3,4,5,6,null,null,null,7,8], root2 = [1,3,2,null,6,4,5,null,null,null,null,8,7]
    Output: true
    Explanation: We flipped at nodes with values 1, 3, and 5.

  Example 2:
    Input: root1 = [], root2 = []
    Output: true
  
  Example 3:
    Input: root1 = [], root2 = [1]
    Output: false
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
    public bool FlipEquiv(TreeNode root1, TreeNode root2) {
        if (root1 is null && root2 is null) return true;
        if (root1 is not null && root2 is null) return false;
        if (root1 is null && root2 is not null) return false;
        if (root1.val != root2.val) return false;

        bool leftEq = FlipEquiv(root1.left, root2.left) || FlipEquiv(root1.left, root2.right);
        bool rightEq = FlipEquiv(root1.right, root2.right) || FlipEquiv(root1.right, root2.left);

        return leftEq && rightEq;
    }
}
