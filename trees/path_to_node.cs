/*
  You are given the root of a binary tree with n nodes. Each node is uniquely assigned a value from 1 to n. You are also given an integer startValue representing the value of the start node s, and a different integer destValue representing the value of the destination node t.

  Find the shortest path starting from node s and ending at node t. Generate step-by-step directions of such path as a string consisting of only the uppercase letters 'L', 'R', and 'U'. Each letter indicates a specific direction:
  'L' means to go from a node to its left child node.
  'R' means to go from a node to its right child node.
  'U' means to go from a node to its parent node.
  
  Return the step-by-step directions of the shortest path from node s to node t.

  Example:
    Input: root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
    Output: "UURL"
    Explanation: The shortest path is: 3 → 1 → 5 → 2 → 6.
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
    public string GetDirections(TreeNode root, int startValue, int destValue) {
        List<char> s = [], t = [];
        Find(root, startValue, s);
        Find(root, destValue, t);

        while (s.Count > 0 && t.Count > 0 && s[^1] == t[^1]) {
            s.RemoveAt(s.Count - 1);
            t.RemoveAt(t.Count - 1);
        }

        t.Reverse();

        return new string('U',s.Count) + new string(t.ToArray());        
    }

    private static bool Find(TreeNode node, int val, List<char> path) {
        if (node.val == val) return true;

        if (node.left != null && Find(node.left, val, path)) path.Add('L');
        else if (node.right != null && Find(node.right, val, path)) path.Add('R');

        return path.Count > 0;
    }
}
