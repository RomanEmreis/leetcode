/*
  You are given an array nums of positive integers and a positive integer k.
  A subset of nums is beautiful if it does not contain two integers with an absolute difference equal to k.

  Return the number of non-empty beautiful subsets of the array nums.

  A subset of nums is an array that can be obtained by deleting some (possibly none) elements from nums. Two subsets are different if and only if the chosen indices to delete are different.

  Example:
    Input: nums = [2,4,6], k = 2
    Output: 4
    Explanation: The beautiful subsets of the array nums are: [2], [4], [6], [2, 6].
    It can be proved that there are only 4 beautiful subsets in the array [2,4,6].
*/
func beautifulSubsets(nums []int, k int) int {
    n, result := len(nums), 0;
    if n <= 1 {
        return n;
    }

    subset := make([]int, 0);

    var dfs func(index int);
    dfs = func(index int) {
        if index >= n {
            result++;
            return;
        }

        if !slices.Contains(subset, nums[index] + k) && !slices.Contains(subset, nums[index] - k) {
            subset = append(subset, nums[index]);
            dfs(index + 1);
            subset = subset[:len(subset) - 1];
            dfs(index + 1);
        } else {
            dfs(index + 1);
        }
    }

    dfs(0);

    return result - 1;
}
