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
    TreeNode* traverse(TreeNode* root, unordered_set<int>& toDelete, vector<TreeNode*>& result){
        if (root == nullptr) return nullptr;

        TreeNode* leftNode = traverse(root->left, toDelete, result);
        TreeNode* rightNode = traverse(root->right, toDelete, result);

        root->left = leftNode;
        root->right = rightNode;

        if (toDelete.count(root->val)) {
            if (root->left) result.push_back(root->left);
            if (root->right) result.push_back(root->right);

            delete root;
            
            return nullptr;
        }

        return root;
    }
public:
    vector<TreeNode*> delNodes(TreeNode* root, vector<int>& to_delete) {
        vector<TreeNode*> result;
        unordered_set<int> toDelete(to_delete.begin(), to_delete.end());

        root = traverse(root, toDelete, result);
        if (root) result.push_back(root);
        
        return result;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
