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
    public TreeNode AddOneRow(TreeNode root, int val, int depth) {
        if (depth == 1) return new(val, root, null);

        Stack<(TreeNode, int)> s = [];
        s.Push((root, 1));

        while (s.TryPop(out var item)) {
            var (node, curr) = item;

            if (curr == depth - 1) {
                node.left = new(val, node.left, null);
                node.right = new(val, null, node.right);
            } else {
                if (node.left != null) s.Push((node.left, curr + 1));
                if (node.right != null) s.Push((node.right, curr + 1));
            }
        }

        return root;
    }
}
