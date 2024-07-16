/*
  You are given the root of a binary tree with n nodes. Each node is uniquely assigned a value from 1 to n. You are also given an integer startValue representing the value of the start node s, and a different integer destValue representing the value of the destination node t.

  Find the shortest path starting from node s and ending at node t. Generate step-by-step directions of such path as a string consisting of only the uppercase letters 'L', 'R', and 'U'. Each letter indicates a specific direction:
  'L' means to go from a node to its left child node.
  'R' means to go from a node to its right child node.
  'U' means to go from a node to its parent node.
  
  Return the step-by-step directions of the shortest path from node s to node t.

  Example:
    Input: root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
    Output: "UURL"
    Explanation: The shortest path is: 3 → 1 → 5 → 2 → 6.
*/
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func getDirections(root *TreeNode, startValue int, destValue int) string {
    var pathStart, pathTarget string
    find(root, startValue, &pathStart)
    find(root, destValue, &pathTarget)
    for len(pathStart) > 0 && len(pathTarget) > 0 && pathStart[len(pathStart)-1] == pathTarget[len(pathTarget)-1] {
        pathStart = pathStart[:len(pathStart)-1]
        pathTarget = pathTarget[:len(pathTarget)-1]
    }

    stepsUp := strings.Repeat("U", len(pathStart))
    stepsDown := reverse(pathTarget)
    return stepsUp + stepsDown
}

func find(node *TreeNode, val int, path *string) bool {
    if node == nil {
        return false
    }
    if node.Val == val {
        return true
    }
    if find(node.Left, val, path) {
        *path = *path + "L"
        return true
    } else if find(node.Right, val, path) {
        *path = *path + "R"
        return true
    }
    if len(*path) > 0 {
        *path = (*path)[:len(*path)-1]
    }
    return false
}

func reverse(s string) string {
    runes := []rune(s)
    for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
        runes[i], runes[j] = runes[j], runes[i]
    }
    return string(runes)
}
