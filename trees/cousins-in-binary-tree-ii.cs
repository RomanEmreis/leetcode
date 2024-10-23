/*
  Given the root of a binary tree, replace the value of each node in the tree with the sum of all its cousins' values.
  Two nodes of a binary tree are cousins if they have the same depth with different parents.
  
  Return the root of the modified tree.
  
  Note that the depth of a node is the number of edges in the path from the root node to it.
  
  Example 1:
    Input: root = [5,4,9,1,10,null,7]
    Output: [0,0,0,7,7,null,11]
    Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
    - Node with value 5 does not have any cousins so its sum is 0.
    - Node with value 4 does not have any cousins so its sum is 0.
    - Node with value 9 does not have any cousins so its sum is 0.
    - Node with value 1 has a cousin with value 7 so its sum is 7.
    - Node with value 10 has a cousin with value 7 so its sum is 7.
    - Node with value 7 has cousins with values 1 and 10 so its sum is 11.
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
    private readonly List<int> s = [];
    public TreeNode ReplaceValueInTree(TreeNode root) {
        Dfs1(root, 0);
        root.val = 0;
        Dfs2(root, 0);

        return root;
    }

    private void Dfs1(TreeNode root, int level) {
        if (root is null) return;

        if (s.Count <= level) s.Add(0);

        s[level] += root.val;

        Dfs1(root.left, level + 1); 
        Dfs1(root.right, level + 1); 
    }

    private void Dfs2(TreeNode root, int level) {
        int l = root.left?.val ?? 0;
        int r = root.right?.val ?? 0;
        int sum = l + r;

        ++level;

        if (root.left is not null) {
            root.left.val = s[level] - sum;
            Dfs2(root.left, level);
        }

        if (root.right is not null) {
            root.right.val = s[level] - sum;
            Dfs2(root.right, level);
        }
    }
}
