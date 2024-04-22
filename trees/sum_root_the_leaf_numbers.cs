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
    public int SumNumbers(TreeNode root) {
        int result = 0;

        Stack<(TreeNode, int)> s = [];
        s.Push((root, 0));

        while (s.TryPop(out var item)) {
            var (node, val) = item;
            if (node != null) {
                int current = val * 10 + node.val;
                if (node.left == null && node.right == null) result += current;
                else {
                    s.Push((node.left, current));
                    s.Push((node.right, current));
                }
            }
        }

        return result;
    }
}
