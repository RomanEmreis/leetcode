/*
  You are given the root of a binary tree with n nodes. Each node is assigned a unique value from 1 to n. You are also given an array queries of size m.
  
  You have to perform m independent queries on the tree where in the ith query you do the following:
  
  Remove the subtree rooted at the node with the value queries[i] from the tree. It is guaranteed that queries[i] will not be equal to the value of the root.
  Return an array answer of size m where answer[i] is the height of the tree after performing the ith query.
  
  Note:
  
  The queries are independent, so the tree returns to its initial state after each query.
  The height of a tree is the number of edges in the longest simple path from the root to some node in the tree.
  
  Example 1:
    Input: root = [1,3,4,2,null,6,5,null,null,null,null,null,7], queries = [4]
    Output: [2]
    Explanation: The diagram above shows the tree after removing the subtree rooted at node with value 4.
    The height of the tree is 2 (The path 1 -> 3 -> 2).
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
    public int[] TreeQueries(TreeNode root, int[] queries) {
        Dictionary<int, int> d = [];
        _ = CalcDepth(root);

        int[] res = new int[d.Count + 1];
        Dfs(root, -1, 0);

        int[] result = new int[queries.Length];
        for (int i = 0; i < queries.Length; ++i) {
            result[i] = res[queries[i]];
        }

        return result;

        int CalcDepth(TreeNode root) {
            if (root is null) return 0;

            int l = CalcDepth(root.left);
            int r = CalcDepth(root.right);

            d[root.val] = 1 + Math.Max(l, r);
            return d[root.val];
        }

        void Dfs(TreeNode root, int depth, int rest) {
            if (root is null) return;

            ++depth;
            res[root.val] = rest;

            Dfs(root.left, depth, Math.Max(rest, depth + (root.right is null ? 0 : d[root.right.val])));
            Dfs(root.right, depth, Math.Max(rest, depth + (root.left is null ? 0 : d[root.left.val])));
        }
    }
}
