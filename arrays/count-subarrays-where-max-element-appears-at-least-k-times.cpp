/*
  2962. Count Subarrays Where Max Element Appears at Least K Times
  
  You are given an integer array nums and a positive integer k.
  Return the number of subarrays where the maximum element of nums appears at least k times in that subarray.
  
  A subarray is a contiguous sequence of elements within an array.
  
  Example 1:
  Input: nums = [1,3,2,3,3], k = 2
  Output: 6
  Explanation: The subarrays that contain the element 3 at least 2 times are: [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
  
  Example 2:
  Input: nums = [1,4,2,1], k = 3
  Output: 0
  Explanation: No subarray contains the element 4 at least 3 times.
*/
class Solution {
public:
    long long countSubarrays(vector<int>& nums, int k) {
        int l = 0, r = 0, n = nums.size();
        long long result = 0;

        int mx = *max_element(begin(nums), end(nums));
        while (r < n) {
            if (nums[r++] == mx) --k;
            while (k == 0) if (nums[l++] == mx) ++k;

            result += l;
        }

        return result;
    }
};
