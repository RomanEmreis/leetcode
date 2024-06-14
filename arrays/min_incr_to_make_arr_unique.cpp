/*
  You are given an integer array nums. In one move, you can pick an index i where 0 <= i < nums.length and increment nums[i] by 1.
  Return the minimum number of moves to make every value in nums unique.

  The test cases are generated so that the answer fits in a 32-bit integer.

  Example:
    Input: nums = [1,2,2]
    Output: 1
    Explanation: After 1 move, the array could be [1, 2, 3].
*/
class Solution {
public:
    int minIncrementForUnique(vector<int>& nums) {
        int result = 0, n = nums.size();

        sort(nums.begin(), nums.end());
        for (int i = 1; i < n; ++i) {
            if (nums[i] <= nums[i - 1]) {
                result += nums[i - 1] - nums[i] + 1;
                nums[i] = nums[i - 1] + 1;
            }
        }

        return result;
    }
};
