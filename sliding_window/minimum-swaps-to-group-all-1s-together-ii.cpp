/*
  A swap is defined as taking two distinct positions in an array and swapping the values in them.
  A circular array is defined as an array where we consider the first element and the last element to be adjacent.

  Given a binary circular array nums, return the minimum number of swaps required to group all 1's present in the array together at any location.

  Example:
    Input: nums = [0,1,0,1,1,0,0]
    Output: 1
    Explanation: Here are a few of the ways to group all the 1's together:
    [0,0,1,1,1,0,0] using 1 swap.
    [0,1,1,1,0,0,0] using 1 swap.
    [1,1,0,0,0,0,1] using 2 swaps (using the circular property of the array).
    There is no way to group all 1's together with 0 swaps.
    Thus, the minimum number of swaps required is 1.
*/
static const int init = []{
    ios_base::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);
    return 0;
}();

class Solution {
public:
    int minSwaps(vector<int>& nums) {
        int n = nums.size(), total = 0;
        for (const int& num : nums) if (num == 1) ++total;

        if (total == 0 || total == n) return 0;

        int max_ones = 0;
        for (size_t i = 0; i < total; ++i) if (nums[i] == 1) ++max_ones;

        int curr_ones = max_ones;
        for (size_t i = 1; i < n; ++i) {
            if (nums[i - 1] == 1) --curr_ones;
            if (nums[(i + total - 1) % n] == 1) ++curr_ones;

            max_ones = max(max_ones, curr_ones);
        }

        return total - max_ones;
    }
};
