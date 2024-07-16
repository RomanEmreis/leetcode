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
    static bool find(const TreeNode* node, const int& val, string& path) {
        if (node->val == val) return true;

        if (node->left && find(node->left, val, path)) path.push_back('L');
        else if (node->right && find(node->right, val, path)) path.push_back('R');

        return !path.empty();
    }

public:
    string getDirections(TreeNode* root, int startValue, int destValue) {
        string s, t;

        find(root, startValue, s);
        find(root, destValue, t);

        while (!s.empty() && !t.empty() && s.back() == t.back()) {
            s.pop_back();
            t.pop_back();
        }

        return string(s.size(), 'U') + string(rbegin(t), rend(t));
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
