/*
  The distance of a pair of integers a and b is defined as the absolute difference between a and b.
  Given an integer array nums and an integer k, return the kth smallest distance among all the pairs nums[i] and nums[j] where 0 <= i < j < nums.length.

  Example 1:
    Input: nums = [1,3,1], k = 1
    Output: 0
    Explanation: Here are all the pairs:
    (1,3) -> 2
    (1,1) -> 0
    (3,1) -> 2
    Then the 1st smallest distance pair is (1,1), and its distance is 0.
*/
class Solution {
private:
    inline int count(vector<int>& nums, int k) const {
        int left = 0, count = 0;
        for (size_t right = 1; right < nums.size(); ++right) {
            while (nums[right] - nums[left] > k) ++left;
            count += right - left;
        }
        return count;
    }
public:
    int smallestDistancePair(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());

        int low = 0, high = nums[nums.size() - 1] - nums[0];
        while (low < high) {
            int mid = low + (high - low) / 2;
            int c = count(nums, mid);
            if (c >= k) high = mid;
            else low = mid + 1;
        }
        return low;
    }
};
