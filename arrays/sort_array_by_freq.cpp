/*
  Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.
  Return the sorted array.

  Example:
    Input: nums = [1,1,2,2,2,3]
    Output: [3,1,1,2,2,2]
    Explanation: '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.
*/
class Solution {
public:
    vector<int> frequencySort(vector<int>& nums) {
        unordered_map<int, int> m;
        for (const int& n : nums) {
            ++m[n];
        }

        sort(nums.begin(), nums.end(), [&](int x, int y) -> bool {
            return m[x] == m[y] ? x > y : m[x] < m[y];
        });

        return nums;
    }
};
