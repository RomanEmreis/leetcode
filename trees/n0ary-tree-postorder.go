/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

func postorder(root *Node) []int {
    result := make([]int, 0);
    traverse(root, &result);
    return result;
}

func traverse(root *Node, result *[]int) {
    if root == nil {
        return;
    }
    for _, c := range root.Children {
        traverse(c, result);
    }
    *result = append(*result, root.Val);
}
