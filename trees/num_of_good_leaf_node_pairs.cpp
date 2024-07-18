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
    static int dfs(TreeNode* root, int& distance, vector<int>& depths) {
        if (!root) return 0;
        if (!root->left && !root->right) {
            depths[1] = 1;
            return 0;
        }

        vector<int> l_depths(distance + 1);
        vector<int> r_depths(distance + 1);

        int l_pairs = dfs(root->left, distance, l_depths);
        int r_pairs = dfs(root->right, distance, r_depths);

        for (int i = 1; i <= distance; ++i) {
            depths[i] = l_depths[i - 1] + r_depths[i - 1];
        }

        int total = l_pairs + r_pairs;

        for (int i = 1; i < distance; ++i) {
            for (int j = distance - i; j >= 1; --j) {
                total += l_depths[i] * r_depths[j];
            }
        }

        return total;
    }
public:
    int countPairs(TreeNode* root, int distance) {
        vector<int> depths(distance + 1);
        return dfs(root, distance, depths);
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
