/*
  You are given an integer array nums. In one move, you can pick an index i where 0 <= i < nums.length and increment nums[i] by 1.
  Return the minimum number of moves to make every value in nums unique.

  The test cases are generated so that the answer fits in a 32-bit integer.

  Example:
    Input: nums = [1,2,2]
    Output: 1
    Explanation: After 1 move, the array could be [1, 2, 3].
*/
func minIncrementForUnique(nums []int) int {
    slices.Sort(nums);

    result, n := 0, len(nums);
    for i := 1; i < n; i++ {
        if nums[i] <= nums[i - 1] {
            result += nums[i - 1] - nums[i] + 1;
            nums[i] = nums[i - 1] + 1;
        }
    }

    return result;
}
