/*
  3362. Zero Array Transformation III
  
  You are given an integer array nums of length n and a 2D array queries where queries[i] = [li, ri].
  Each queries[i] represents the following action on nums:
  
  Decrement the value at each index in the range [li, ri] in nums by at most 1.
  The amount by which the value is decremented can be chosen independently for each index.
  A Zero Array is an array with all its elements equal to 0.
  
  Return the maximum number of elements that can be removed from queries, such that nums can still be converted to a zero array using the remaining queries. If it is not possible to convert nums to a zero array, return -1.
  
  Example 1:
  Input: nums = [2,0,2], queries = [[0,2],[0,2],[1,1]]
  Output: 1
  Explanation:
  After removing queries[2], nums can still be converted to a zero array.
  Using queries[0], decrement nums[0] and nums[2] by 1 and nums[1] by 0.
  Using queries[1], decrement nums[0] and nums[2] by 1 and nums[1] by 0.
  
  Example 2:
  Input: nums = [1,1,1,1], queries = [[1,3],[0,2],[1,3],[1,2]]
  Output: 2
  Explanation:
  We can remove queries[2] and queries[3].
  
  Example 3:
  Input: nums = [1,2,3,4], queries = [[0,3]]
  Output: -1
  Explanation:
  nums cannot be converted to a zero array even after using all the queries.
*/
public class Solution {
    public int MaxRemoval(int[] nums, int[][] queries) {
        int idx = 0;
        PriorityQueue<int, int> available = new();
        PriorityQueue<int, int> running = new();

        Array.Sort(queries, (a, b) => a[0].CompareTo(b[0]));

        for (int i = 0; i < nums.Length; ++i) {
            while (idx < queries.Length && queries[idx][0] <= i) {
                int x = queries[idx++][1];
                available.Enqueue(x, -x);
            }
            while (running.Count > 0 && running.Peek() < i) {
                _ = running.Dequeue();
            }
            while (nums[i] > running.Count) {
                if (available.Count == 0 || available.Peek() < i) return -1;
                int x = available.Dequeue();
                running.Enqueue(x, x);
            }
        }

        return available.Count;
    }
}
