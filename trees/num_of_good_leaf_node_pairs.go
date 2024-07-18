/*
  You are given the root of a binary tree and an integer distance. A pair of two different leaf nodes of a binary tree is said to be good if the length of the shortest path between them is less than or equal to distance.
  Return the number of good leaf node pairs in the tree.

  Example:
    Input: root = [1,2,3,null,4], distance = 3
    Output: 1
    Explanation: The leaf nodes of the tree are 3 and 4 and the length of the shortest path between them is 3. This is the only good pair.
*/
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func countPairs(root *TreeNode, distance int) int {
    depths := make([]int, distance + 1);
    return dfs(root, distance, &depths);
}

func dfs(root *TreeNode, distance int, depths *[]int) int {
    if root == nil {
        return 0;
    }

    if root.Left == nil && root.Right == nil {
        (*depths)[1] = 1;
        return 0;
    }

    leftDepths := make([]int, distance + 1);
    rightDepths := make([]int, distance + 1);

    leftPairs := dfs(root.Left, distance, &leftDepths);
    rightPairs := dfs(root.Right, distance, &rightDepths);

    for i := 1; i <= distance; i++ {
        (*depths)[i] = leftDepths[i - 1] + rightDepths[i - 1];
    }

    totalPairs := leftPairs + rightPairs;

    for i := 1; i < distance; i++ {
        for j := distance - i; j >= 1; j-- {
            totalPairs += leftDepths[i] * rightDepths[j];
        }
    }

    return totalPairs;
}
