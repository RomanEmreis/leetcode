/*
  Given a binary tree root and an integer target, delete all the leaf nodes with value target.
  Note that once you delete a leaf node with value target, if its parent node becomes a leaf node and has the value target, it should also be deleted (you need to continue doing that until you cannot).

  Example 1:
    Input: root = [1,2,3,2,null,2,4], target = 2
    Output: [1,null,3,null,4]
    Explanation: Leaf nodes in green with value (target = 2) are removed (Picture in left). 
    After removing, new nodes become leaf nodes with value (target = 2) (Picture in center).
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
    public TreeNode RemoveLeafNodes(TreeNode root, int target) {
        RemoveLeafNodes(ref root, ref target);
        return root;
    }

    private static void RemoveLeafNodes(ref TreeNode root, ref int target) {
        if (root == null) return;

        if (root.left == null && root.right == null && root.val == target) {
            root = null;
        } else {
            RemoveLeafNodes(ref root.left, ref target);
            RemoveLeafNodes(ref root.right, ref target);

            if (root.left == null && root.right == null && root.val == target) root = null;
        }
    }
}
