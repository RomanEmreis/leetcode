/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func postorderTraversal(root *TreeNode) []int {
    result := make([]int, 0);
    postorder(root, &result);
    return result;
}

func postorder(root *TreeNode, result *[]int) {
    if root == nil {
        return;
    }

    postorder(root.Left, result);
    postorder(root.Right, result);

    *result = append(*result, root.Val)
}
