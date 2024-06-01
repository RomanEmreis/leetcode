/*
  Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.
  You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.

  Example:
    Input: nums = [1,2,1,3,2,5]
    Output: [3,5]
    Explanation:  [5, 3] is also a valid answer.
*/
public class Solution {
    public int[] SingleNumber(int[] nums) {
        Span<int> result = stackalloc int[2];
        
        int xor = 0;
        foreach (int num in nums) xor ^= num;

        int lowestBit = xor & -xor;
        foreach (int num in nums) {
            if ((lowestBit & num) == 0) result[0] ^= num;
            else result[1] ^= num;
        }

        return result.ToArray();
    }
}
