/*
  Given a binary tree root and a linked list with head as the first node. 
  Return True if all the elements in the linked list starting from the head correspond to some downward path connected in the binary tree otherwise return False.
  
  In this context downward path means a path that starts at some node and goes downwards.

  Example 1:
    Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
    Output: true
    Explanation: Nodes in blue form a subpath in the binary Tree. 
*/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isSubPath(head *ListNode, root *TreeNode) bool {
    if root == nil {
        return false;
    }

    return dfs(head, root) || isSubPath(head, root.Left) || isSubPath(head, root.Right);
}

func dfs(head *ListNode, root *TreeNode) bool {
    if head == nil {
        return true;
    }
    if root == nil || root.Val != head.Val {
        return false;
    }

    return dfs(head.Next, root.Left) || dfs(head.Next, root.Right);
}
