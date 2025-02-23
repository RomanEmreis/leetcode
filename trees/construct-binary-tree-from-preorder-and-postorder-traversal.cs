/*
  889. Construct Binary Tree from Preorder and Postorder Traversal
  
  Given two integer arrays, preorder and postorder where preorder is the preorder traversal of a binary tree of distinct values and postorder is the postorder traversal of the same tree, reconstruct and return the binary tree.
  If there exist multiple answers, you can return any of them.
  
  Example 1:
  Input: preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
  Output: [1,2,3,4,5,6,7]
  
  Example 2:
  Input: preorder = [1], postorder = [1]
  Output: [1]
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
    public TreeNode ConstructFromPrePost(int[] preorder, int[] postorder) {
        Dictionary<int, int> positions = [];
        int n = postorder.Length;
        for (int i  = 0; i < n; ++i) {
            positions[postorder[i]] = i;
        }

        return Dfs(0, n - 1, 0, n - 1);

        TreeNode Dfs(int a, int b, int c, int d) {
            if (a > b) return null;

            var node = new TreeNode(preorder[a]);
            if (a == b) return node;

            var i = positions[preorder[a + 1]];
            var m = i - c + 1;
            
            node.left = Dfs(a + 1, a + m, c, i);
            node.right = Dfs(a + m + 1, b, i + 1, d - 1);

            return node;
        }
    }
}
