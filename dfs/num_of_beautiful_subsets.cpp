/*
  You are given an array nums of positive integers and a positive integer k.
  A subset of nums is beautiful if it does not contain two integers with an absolute difference equal to k.

  Return the number of non-empty beautiful subsets of the array nums.

  A subset of nums is an array that can be obtained by deleting some (possibly none) elements from nums. Two subsets are different if and only if the chosen indices to delete are different.

  Example:
    Input: nums = [2,4,6], k = 2
    Output: 4
    Explanation: The beautiful subsets of the array nums are: [2], [4], [6], [2, 6].
    It can be proved that there are only 4 beautiful subsets in the array [2,4,6].
*/
class Solution {
public:
    int beautifulSubsets(vector<int>& nums, int k) {
        int n = nums.size();
        if (n <= 1) return n;

        int result = 0;
        vector<int> subsets;

        function<void(int)> dfs;
        dfs = [&](int index) {
            if (index >= n) {
                ++result;
                return;
            }

            if (find(subsets.begin(), subsets.end(), nums[index] + k) == subsets.end() &&
                find(subsets.begin(), subsets.end(), nums[index] - k) == subsets.end()) {
                subsets.push_back(nums[index]);
                dfs(index + 1);
                subsets.pop_back();
                dfs(index + 1);
            } else {
                dfs(index + 1);
            }
        };

        dfs(0); 

        return result - 1;
    }
};

auto init = []() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    return 'c';
}();
