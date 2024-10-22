/*
  You are given the root of a binary tree and a positive integer k.
  The level sum in the tree is the sum of the values of the nodes that are on the same level.
  
  Return the kth largest level sum in the tree (not necessarily distinct). If there are fewer than k levels in the tree, return -1.
  
  Note that two nodes are on the same level if they have the same distance from the root.

  Example 1:
    Input: root = [5,8,9,2,1,3,7,4,6], k = 2
    Output: 13
    Explanation: The level sums are the following:
    - Level 1: 5.
    - Level 2: 8 + 9 = 17.
    - Level 3: 2 + 1 + 3 + 7 = 13.
    - Level 4: 4 + 6 = 10.
    The 2nd largest level sum is 13.
  
  Example 2:
    Input: root = [1,2,null,3], k = 1
    Output: 3
    Explanation: The largest level sum is 3.
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
    public long KthLargestLevelSum(TreeNode root, int k) {
        Dictionary<int, long> sums = [];

        Calculate(1, k, root, sums);

        var levelSums = sums.Values.ToList();
        levelSums.Sort();

        return levelSums.Count >= k ? levelSums[^k] : -1;
    }

    private static void Calculate(int level, int k, TreeNode node, Dictionary<int, long> sums) {
        if (node is null) return;

        if (sums.ContainsKey(level)) sums[level] += node.val;
        else sums[level] = node.val;

        Calculate(level + 1, k, node.left, sums);
        Calculate(level + 1, k, node.right, sums);
    }
}
