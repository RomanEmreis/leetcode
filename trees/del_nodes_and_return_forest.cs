/*
  Given the root of a binary tree, each node in the tree has a distinct value.
  After deleting all nodes with a value in to_delete, we are left with a forest (a disjoint union of trees).

  Return the roots of the trees in the remaining forest. You may return the result in any order.

  Example 1:
    Input: root = [1,2,3,4,5,6,7], to_delete = [3,5]
    Output: [[1,2,null,4],[6],[7]]

  Example 2:
    Input: root = [1,2,4,null,3], to_delete = [3]
    Output: [[1,2,4]]
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
    public IList<TreeNode> DelNodes(TreeNode root, int[] to_delete) {
        List<TreeNode> result = [];
        HashSet<int> toDelete = to_delete.ToHashSet();
        Stack<(TreeNode, TreeNode, bool)> st = [];
        st.Push((root, null, false));

        while (st.TryPop(out var item)) {
            var (node, parent, isLeft) = item;
            if (node == null) continue;

            if (toDelete.Contains(node.val)) {
                if (parent != null && isLeft) parent.left = null;
                else if (parent != null) parent.right = null;

                st.Push((node.left, null, true));
                st.Push((node.right, null, false));
            } else {
                if (parent == null) result.Add(node);

                st.Push((node.left, node, true));
                st.Push((node.right, node, false));
            }
        }

        return result;
    }
}
