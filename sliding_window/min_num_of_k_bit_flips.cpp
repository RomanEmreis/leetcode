/*
  You are given a binary array nums and an integer k.
  A k-bit flip is choosing a subarray of length k from nums and simultaneously changing every 0 in the subarray to 1, and every 1 in the subarray to 0.

  Return the minimum number of k-bit flips required so that there is no 0 in the array. If it is not possible, return -1.

  A subarray is a contiguous part of an array.

  Example:
    Input: nums = [0,1,0], k = 1
    Output: 2
    Explanation: Flip nums[0], then flip nums[2].
*/
class Solution {
public:
    int minKBitFlips(vector<int>& nums, int k) {
        int result = 0, count = 0, n = nums.size();

        for (int i = 0; i < n; ++i) {
            if (i >= k && nums[i - k] > 1) {
                nums[i - k] -= 2;
                --count;
            }
            if (count % 2 == nums[i] % 2) {
                if (i + k > n) return -1;
                nums[i] += 2;
                ++count;
                ++result;
            }
        }

        return result;
    }
};

auto init = []() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    return 'c';
}();
