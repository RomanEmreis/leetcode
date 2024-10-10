/*
  A ramp in an integer array nums is a pair (i, j) for which i < j and nums[i] <= nums[j]. The width of such a ramp is j - i.
  Given an integer array nums, return the maximum width of a ramp in nums. If there is no ramp in nums, return 0.
  
  Example 1:
    Input: nums = [6,0,8,2,1,5]
    Output: 4
    Explanation: The maximum width ramp is achieved at (i, j) = (1, 5): nums[1] = 0 and nums[5] = 5.
*/
public class Solution {
    public int MaxWidthRamp(int[] nums) {
        Stack<int> st = [];
        for (int i = 0; i < nums.Length; ++i) {
            if (st.Count == 0 || nums[i] < nums[st.Peek()]) st.Push(i);
        }

        int result = 0;
        for (int i = nums.Length - 1; i > result; --i) {
            while (st.Count > 0 && nums[i] >= nums[st.Peek()]) {
                result = Math.Max(result, i - st.Pop());
            }
        }

        return result;
    }
}
