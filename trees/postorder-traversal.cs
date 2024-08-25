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
    public IList<int> PostorderTraversal(TreeNode root) {
        List<int> result = [];
        Postorder(root, result);
        return result;
    }

    private static void Postorder(TreeNode node, List<int> result) {
        if (node == null) return;

        Postorder(node.left, result);
        Postorder(node.right, result);

        result.Add(node.val);
    }
}
