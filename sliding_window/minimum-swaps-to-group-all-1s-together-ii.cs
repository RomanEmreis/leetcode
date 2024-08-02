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
public class Solution {
    public int MinSwaps(int[] nums) {
        int totalOnes = 0;
        foreach (int n in nums) if (n == 1) ++totalOnes;

        if (totalOnes == 0 || totalOnes == nums.Length) return 0;

        int maxOnes = 0;
        for (int i = 0; i < totalOnes; ++i) {
            if (nums[i] == 1) ++maxOnes;
        }

        int currOnes = maxOnes;
        for (int i = 1; i < nums.Length; ++i) {
            if (nums[i - 1] == 1) --currOnes;
            if (nums[(i + totalOnes - 1) % nums.Length] == 1) ++currOnes;

            maxOnes = Math.Max(maxOnes, currOnes);
        }

        return totalOnes - maxOnes;
    }
}
