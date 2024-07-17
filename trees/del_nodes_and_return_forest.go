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
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func delNodes(root *TreeNode, to_delete []int) []*TreeNode {
    result := make([]*TreeNode, 0);
    toDelete := make(map[int]struct{});

    for _, n := range to_delete {
        toDelete[n] = struct{}{}
    }

    traverse(root, nil, false, &toDelete, &result);

    return result;
}

func traverse(node, parent *TreeNode, isLeft bool, toDelete *map[int]struct{}, result *[]*TreeNode) {
    if  node == nil {
        return;
    }
    
    if _, ok := (*toDelete)[node.Val]; ok {
        if parent != nil && isLeft {
            parent.Left = nil;
        } else if parent != nil {
            parent.Right = nil;
        }

        traverse(node.Left, nil, true, toDelete, result);
        traverse(node.Right, nil, false, toDelete, result);
    } else {
        if parent == nil {
            (*result) = append((*result), node);
        }
        traverse(node.Left, node, true, toDelete, result);
        traverse(node.Right, node, false, toDelete, result);
    }
}
