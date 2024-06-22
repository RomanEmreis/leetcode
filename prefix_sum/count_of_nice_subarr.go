/*
  Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.
  Return the number of nice sub-arrays.

  Example:
    Input: nums = [1,1,2,1,1], k = 3
    Output: 2
    Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
*/
func numberOfSubarrays(nums []int, k int) int {
    n := len(nums);
    result, sum := 0, 0;

    m := make([]int, n + 1);
    m[0]++;

    for i := 0; i < n; i++ {
        sum += nums[i] & 1;
        if sum - k >= 0 {
            result += m[sum - k];
        }
        m[sum]++;
    }

    return result;
}
