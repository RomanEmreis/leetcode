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
public class Solution {
    public int SmallestDistancePair(int[] nums, int k) {
        Array.Sort(nums);

        int low = 0;
        int high = nums[^1] - nums[0];

        while (low < high) {
            int mid = low + (high - low) / 2;
            int count = Count(nums, mid);

            if (count >= k) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        return low;
    }

    private static int Count(int[] nums, int k) {
        int count = 0;
        int left = 0;

        for (int right = 1; right < nums.Length; ++right) {
            while (nums[right] - nums[left] > k) ++left;
            count += right - left;
        }

        return count;
    }
}
