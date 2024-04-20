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
    public string SmallestFromLeaf(TreeNode root) {
        string result = null;

        FindSmallest(root, string.Empty, ref result);

        return result;
    }

    private void FindSmallest(TreeNode node, string path, ref string result) {
        if (node == null) return;

        path = ((char)('a' + node.val)) + path;

        if (node.left == null && node.right == null) {
            if (result == null || path.CompareTo(result) < 0) result = path;
        }

        FindSmallest(node.left, path, ref result);
        FindSmallest(node.right, path, ref result);
    }
}
