/*
  You are given the root of a binary tree and an integer distance. A pair of two different leaf nodes of a binary tree is said to be good if the length of the shortest path between them is less than or equal to distance.
  Return the number of good leaf node pairs in the tree.

  Example:
    Input: root = [1,2,3,null,4], distance = 3
    Output: 1
    Explanation: The leaf nodes of the tree are 3 and 4 and the length of the shortest path between them is 3. This is the only good pair.
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
    public int CountPairs(TreeNode root, int distance) {
        Span<int> depths = stackalloc int[distance + 1];
        return Dfs(root, ref distance, ref depths);
    }
    
    private static int Dfs(TreeNode root, ref int distance, ref Span<int> depths) {
        if (root == null) return 0;
        
        if (root.left == null && root.right == null) {
          depths[1] = 1;
          return 0;
        }
        
        Span<int> leftDepths = stackalloc int[distance + 1];
        Span<int> rightDepths = stackalloc int[distance + 1];
        
        var leftPairs = Dfs(root.left, ref distance, ref leftDepths);
        var rightPairs = Dfs(root.right, ref distance, ref rightDepths);
        
        for (var i = 1; i <= distance; ++i) {
          depths[i] = leftDepths[i - 1] + rightDepths[i - 1];
        }
        
        var totalPairs = leftPairs + rightPairs;
        
        for (var i = 1; i < distance; ++i) {
          for (var j = distance - i; j >= 1; --j) {
            totalPairs += leftDepths[i] * rightDepths[j];
          }
        }
        
        return totalPairs;
    }
}
