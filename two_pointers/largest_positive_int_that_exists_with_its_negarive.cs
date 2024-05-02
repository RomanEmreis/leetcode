public class Solution {
    public int FindMaxK(int[] nums) {
        if (nums.Length == 1) return -1;

        Array.Sort(nums);

        int l = 0, r = nums.Length - 1, result = -1;
        while (l < r) {
            int sum = nums[l] + nums[r];
            if (sum == 0) {
                result = Math.Max(result, nums[r]);
                --r;
                ++l;
            } else if (sum > 0) {
                --r;
            } else {
                ++l;
            }
        }

        return result;
    }
}
