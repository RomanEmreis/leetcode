/*
  Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
  We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.

  You must solve this problem without using the library's sort function.
*/
public class Solution {
    public void SortColors(int[] nums) {
        Span<int> count = [0,0,0];

        foreach (int num in nums) ++count[num];

        for (int i = 0, idx = 0; i < count.Length; ++i) {
            for (int j = 0; j < count[i]; ++j) {
                nums[idx++] = i;
            }
        }
    }
}
