/*
  Given a binary tree root and an integer target, delete all the leaf nodes with value target.
  Note that once you delete a leaf node with value target, if its parent node becomes a leaf node and has the value target, it should also be deleted (you need to continue doing that until you cannot).

  Example 1:
    Input: root = [1,2,3,2,null,2,4], target = 2
    Output: [1,null,3,null,4]
    Explanation: Leaf nodes in green with value (target = 2) are removed (Picture in left). 
    After removing, new nodes become leaf nodes with value (target = 2) (Picture in center).
*/
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func removeLeafNodes(root *TreeNode, target int) *TreeNode {
    if root == nil {
        return nil;
    }

    root.Left = removeLeafNodes(root.Left, target);
    root.Right = removeLeafNodes(root.Right, target);

    if root.Left == nil && root.Right == nil && root.Val == target {
        return nil;
    } else {
        return root;
    }
}
