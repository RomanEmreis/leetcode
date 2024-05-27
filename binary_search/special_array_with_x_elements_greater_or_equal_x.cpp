/*
  You are given an array nums of non-negative integers. nums is considered special if there exists a number x such that there are exactly x numbers in nums that are greater than or equal to x.
  Notice that x does not have to be an element in nums.

  Return x if the array is special, otherwise, return -1. It can be proven that if nums is special, the value for x is unique.

  Example:
    Input: nums = [3,5]
    Output: 2
    Explanation: There are 2 values (3 and 5) that are greater than or equal to 2.
*/
class Solution {
public:
    int specialArray(vector<int>& nums) {
        sort(nums.begin(), nums.end(), greater<int>());
        int n = nums.size();
        
        if (nums[n - 1] > n) return n;
        if (nums[0] == 0) return -1;

        int l = 0, r = n - 1, m = 0;
        while (l <= r) {
            m = l + (r - l) / 2;
            if (nums[m] > m) l = m + 1;
            else if (nums[m] < m) r = m - 1;
            else return -1;
        }
        return nums[m] > m ? m + 1 : m;
    }
};
