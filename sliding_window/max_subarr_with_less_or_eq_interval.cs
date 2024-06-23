/*
  Given an array of integers nums and an integer limit, 
  return the size of the longest non-empty subarray such that the absolute difference between any two elements of this subarray is less than or equal to limit.

  Example:
    Input: nums = [8,2,4,7], limit = 4
    Output: 2 
    Explanation: All subarrays are: 
    [8] with maximum absolute diff |8-8| = 0 <= 4.
    [8,2] with maximum absolute diff |8-2| = 6 > 4. 
    [8,2,4] with maximum absolute diff |8-2| = 6 > 4.
    [8,2,4,7] with maximum absolute diff |8-2| = 6 > 4.
    [2] with maximum absolute diff |2-2| = 0 <= 4.
    [2,4] with maximum absolute diff |2-4| = 2 <= 4.
    [2,4,7] with maximum absolute diff |2-7| = 5 > 4.
    [4] with maximum absolute diff |4-4| = 0 <= 4.
    [4,7] with maximum absolute diff |4-7| = 3 <= 4.
    [7] with maximum absolute diff |7-7| = 0 <= 4. 
  Therefore, the size of the longest subarray is 2.
*/
public class Solution {
    public int LongestSubarray(int[] nums, int limit) {
        if(nums.Length == 1) return 1;
        
        LinkedList<int> minQ = [];
        LinkedList<int> maxQ = []; 
        
        int i = 0, result = 1;
        for (int j = 0; j < nums.Length; ++j) {
            while(minQ.Count > 0 && nums[minQ.Last.Value] > nums[j]) minQ.RemoveLast();
            while(maxQ.Count > 0 && nums[maxQ.Last.Value] < nums[j]) maxQ.RemoveLast();
            
            minQ.AddLast(j);
            maxQ.AddLast(j);
            
            while(nums[maxQ.First.Value] - nums[minQ.First.Value] > limit) {
                if(maxQ.First.Value == i) maxQ.RemoveFirst();
                if(minQ.First.Value == i) minQ.RemoveFirst();
                i++;
            }
            
            result = Math.Max(result, j - i + 1);
            if (result == nums.Length) return result;
        }
        
        return result;
    }
}
