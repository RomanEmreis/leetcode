/*
  You are given a 2D integer array descriptions where descriptions[i] = [parenti, childi, isLefti] indicates that parenti is the parent of childi in a binary tree of unique values. Furthermore,

  If isLefti == 1, then childi is the left child of parenti.
  If isLefti == 0, then childi is the right child of parenti.
  Construct the binary tree described by descriptions and return its root.

  The test cases will be generated such that the binary tree is valid.

  Example:
    Input: descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
    Output: [50,20,80,15,17,19]
    Explanation: The root node is the node with value 50 since it has no parent.
    The resulting binary tree is shown in the diagram.
*/
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func createBinaryTree(descriptions [][]int) *TreeNode {
    tree := make(map[int]*TreeNode)
    roots := make(map[int]bool)

    for _, node := range descriptions {
        roots[node[0]] = true
    }

    for _, node := range descriptions {
        if _, ok := roots[node[1]]; ok {
            delete(roots, node[1])
        }
    }

    var root int
    for i := range roots {
        root = i
        break
    }

    for _, node := range descriptions {
        if _, ok := tree[node[0]]; !ok {
            tree[node[0]] = &TreeNode{Val: node[0]}
        }
        if _, ok := tree[node[1]]; !ok {
            tree[node[1]] = &TreeNode{Val: node[1]}
        }

        if node[2] == 1 {
            tree[node[0]].Left = tree[node[1]]
        } else {
            tree[node[0]].Right = tree[node[1]]
        }
    }

    return tree[root]
}
