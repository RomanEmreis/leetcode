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
    vector<TreeNode*> nodes;

    void inOrder(TreeNode* node) {
        if (!node) return;

        inOrder(node->left);
        nodes.push_back(node);
        inOrder(node->right);
    }

    TreeNode* balance(int l, int r) {
        if (l > r) return nullptr;

        int mid = (l + r) / 2;
        TreeNode* node = nodes[mid];

        node->left = balance(l, mid - 1);
        node->right = balance(mid + 1, r);

        return node;
    }

public:
    TreeNode* balanceBST(TreeNode* root) {
        inOrder(root);
        return balance(0, nodes.size() - 1);
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
