/*
  Given an integer array nums of unique elements, return all possible subsets (the power set).
  The solution set must not contain duplicate subsets. Return the solution in any order.

  Example:
    Input: nums = [1,2,3]
    Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
*/
class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        int n = nums.size();
        int pn = 1 << n;
        vector<vector<int>> result(pn);

        for (int i = 0; i < pn; ++i) {
            for (int j = 0; j < n; ++j) {
                if (i & 1 << j) result[i].push_back(nums[j]);
            }
        }

        return result;
    }
};
