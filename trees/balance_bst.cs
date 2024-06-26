/*
  Given the root of a binary search tree, return a balanced binary search tree with the same node values. If there is more than one answer, return any of them.
  A binary search tree is balanced if the depth of the two subtrees of every node never differs by more than 1.

  Example 1:
    Input: root = [1,null,2,null,3,null,4,null,null]
    Output: [2,1,3,null,null,null,4]
    Explanation: This is not the only correct answer, [3,1,4,null,2] is also correct.
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
    private readonly List<TreeNode> nodes = [];

    public TreeNode BalanceBST(TreeNode root) {
        FillInOrder(root);
        root = CreateBalancedBST(0, nodes.Count - 1);
        return root;
    }

    private void FillInOrder(TreeNode node) {
        if (node == null) return;
        
        FillInOrder(node.left);
        nodes.Add(node);
        FillInOrder(node.right);
    }

    private TreeNode CreateBalancedBST(int start, int end) {
        if (start > end) return null;  

        int mid = (start + end) / 2;  
        TreeNode node = nodes[mid];
  
        node.left = CreateBalancedBST(start, mid - 1);  
        node.right = CreateBalancedBST(mid + 1, end);  
  
        return node;  
    }  
}
