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
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
type Tree struct {
	nodes []*TreeNode
}

func balanceBST(root *TreeNode) *TreeNode {
    tree := &Tree{};
    tree.inOrder(root);

    return tree.balance(0, len(tree.nodes) - 1);
}

func (t *Tree) inOrder(node *TreeNode) {
    if node == nil {
        return;
    }

    t.inOrder(node.Left);
    t.nodes = append(t.nodes, node);
    t.inOrder(node.Right);
}

func (t *Tree) balance(l, r int) *TreeNode {
    if l > r {
        return nil;
    }

    mid := (l + r) / 2;
    node := t.nodes[mid];

    node.Left = t.balance(l, mid - 1);
    node.Right = t.balance(mid + 1, r);

    return node;
}
