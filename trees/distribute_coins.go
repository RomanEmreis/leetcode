/*
  You are given the root of a binary tree with n nodes where each node in the tree has node.val coins. There are n coins in total throughout the whole tree.
  In one move, we may choose two adjacent nodes and move one coin from one node to another. A move may be from parent to child, or from child to parent.

  Return the minimum number of moves required to make every node have exactly one coin.

  Example 1:
    Input: root = [3,0,0]
    Output: 2
    Explanation: From the root of the tree, we move one coin to its left child, and one coin to its right child.
*/
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func distributeCoins(root *TreeNode) int {
    return distributeCoins1(root, nil);
}

func distributeCoins1(root *TreeNode, parent *TreeNode) int {
    if root == nil {
        return 0;
    }

    result := distributeCoins1(root.Left, root) + distributeCoins1(root.Right, root);
    reminder := root.Val - 1;

    if parent != nil {
        parent.Val += reminder;
    }

    result += int(math.Abs(float64(reminder)));

    return result
}
