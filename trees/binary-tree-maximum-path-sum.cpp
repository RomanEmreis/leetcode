/*
  A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.
  The path sum of a path is the sum of the node's values in the path.

  Given the root of a binary tree, return the maximum path sum of any non-empty path.

  Example 1:
    Input: root = [1,2,3]
    Output: 6
    Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.

  Example 2:
    Input: root = [-10,9,20,null,null,15,7]
    Output: 42
    Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
*/
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
private:
    int result = -1001;

    int maxPathSumImpl(TreeNode* root) {
        if (!root) return -1001;

        int max_left_sum = max(maxPathSumImpl(root->left), 0);
        int max_right_sum = max(maxPathSumImpl(root->right), 0);

        result = max(result, max_left_sum + max_right_sum + root->val);

        return max(max_left_sum, max_right_sum) + root->val;
    }
public:
    int maxPathSum(TreeNode* root) {
        maxPathSumImpl(root);
        return result;
    }
};
