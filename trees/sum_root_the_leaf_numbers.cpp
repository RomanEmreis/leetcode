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
public:
    int sumNumbers(TreeNode* root) {
        int result = 0;
        stack<pair<TreeNode*, int>> s;
        s.push({root,0});

        while (!s.empty()) {
            pair<TreeNode*, int> p = s.top();
            s.pop();

            TreeNode* node = p.first;
            if (node) {
                int current = p.second * 10 + node->val;
                if (!node->left && !node->right) result += current;
                else {
                    s.push({node->left, current});
                    s.push({node->right, current});
                }
            }
        }

        return result;
    }
};
