/*
  Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.
  A good subarray is a subarray where:
    its length is at least two, and
    the sum of the elements of the subarray is a multiple of k.

  Note that:
    A subarray is a contiguous part of the array.
    An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k. 

  Example 1:
    Input: nums = [23,2,4,6,7], k = 6
    Output: true
    Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.
*/
class Solution {
public:
    bool checkSubarraySum(vector<int>& nums, int k) {
        int l = 0, r = 1;
        int sum = nums[l], n = nums.size();

        while (r < n) {
            sum += nums[r];
            if (sum % k == 0) return true;
            if (nums[r] == nums[r - 1] && nums[r] == 0) return true;

            int x = sum;
            l = 0;
            while ((r - l) > 1 && x >= k) {
                x -= nums[l++];
                if (x % k == 0) return true;
            }
            ++r;
        }

        return false;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
