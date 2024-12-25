/*
  515. Find Largest Value in Each Tree Row
  
  Given the root of a binary tree, return an array of the largest value in each row of the tree (0-indexed).
   
  Example 1:
  Input: root = [1,3,2,5,3,null,9]
  Output: [1,3,9]
  
  Example 2:
  Input: root = [1,2,3]
  Output: [1,3]
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
    public IList<int> LargestValues(TreeNode root) {
        if (root is null) return [];
        List<int> result = [];
        Queue<TreeNode> q = [];
        q.Enqueue(root);
        while (q.Count > 0) {
            int x = int.MinValue;
            for (int i = q.Count; i > 0; --i) {
                var node = q.Dequeue();
                if (node.val > x) x = node.val;
                if (node.left is not null) q.Enqueue(node.left);
                if (node.right is not null) q.Enqueue(node.right);
            }
            result.Add(x);
        }
        return result;
    }
}
