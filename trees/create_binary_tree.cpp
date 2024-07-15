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
    TreeNode* createBinaryTree(vector<vector<int>>& descriptions) {
        unordered_map<int, TreeNode*> tree;
        unordered_set<int> children;

        for (const vector<int>& node : descriptions) {
            children.insert(node[1]);
        }

        int root = -1;
        for (const vector<int>& node : descriptions) {
            if (!children.contains(node[0])) {
                root = node[0];
                break;
            }
        }

        for (const vector<int>& node : descriptions) {
            if (!tree.contains(node[0])) tree[node[0]] = new TreeNode(node[0]);
            if (!tree.contains(node[1])) tree[node[1]] = new TreeNode(node[1]);

            if (node[2] == 1) tree[node[0]]->left = tree[node[1]];
            else tree[node[0]]->right = tree[node[1]];
        }

        return tree[root];
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
