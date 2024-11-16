/*
  You are given an array of integers nums of length n and a positive integer k.
  The power of an array is defined as:
  Its maximum element if all of its elements are consecutive and sorted in ascending order.
  -1 otherwise.
  
  You need to find the power of all subarrays of nums of size k.
  
  Return an integer array results of size n - k + 1, where results[i] is the power of nums[i..(i + k - 1)].
  
  Example 1:
  Input: nums = [1,2,3,4,3,2,5], k = 3
  Output: [3,4,-1,-1,-1]
  Explanation:
  There are 5 subarrays of nums of size 3:
  [1, 2, 3] with the maximum element 3.
  [2, 3, 4] with the maximum element 4.
  [3, 4, 3] whose elements are not consecutive.
  [4, 3, 2] whose elements are not sorted.
  [3, 2, 5] whose elements are not consecutive.
*/
public class Solution {
    public int[] ResultsArray(int[] nums, int k) {
        int[] result = new int[nums.Length - k + 1];

        for (int i = 0, j = k - 1, w = 0; j < nums.Length; ++i, ++j, ++w) {
            int l = i;
            bool isSorted = true;
            while (l < j) {
                isSorted = nums[l++] < nums[l] && (nums[l] - nums[l - 1] == 1);
                if (!isSorted) break;
            }

            result[w] = isSorted ? nums[j] : -1;
        }

        return result; 
    }
}
