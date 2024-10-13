/*
  You have k lists of sorted integers in non-decreasing order. Find the smallest range that includes at least one number from each of the k lists.
  
  We define the range [a, b] is smaller than range [c, d] if b - a < d - c or a < c if b - a == d - c.
  
  Example 1:
    Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
    Output: [20,24]
    Explanation: 
    List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
    List 2: [0, 9, 12, 20], 20 is in range [20,24].
    List 3: [5, 18, 22, 30], 22 is in range [20,24].
*/
public class Solution {
    public int[] SmallestRange(IList<IList<int>> nums) {
        PriorityQueue<(int val, int i, int j), int> pq = new();
        int maxInRange = int.MinValue;

        for (int i = 0; i < nums.Count; ++i) {
            int val = nums[i][0];
            pq.Enqueue((val, i, 0), val);
            if (maxInRange < val) maxInRange = val;
        }

        int start = 0, end = int.MaxValue;

        while (pq.Count == nums.Count) {
            var (val, i, j) = pq.Dequeue();

            if (maxInRange - val < end - start || 
            (maxInRange - val == end - start && val < start)) {
                start = val;
                end = maxInRange;
            }

            if (j + 1 < nums[i].Count) {
                int next = nums[i][j + 1];
                pq.Enqueue((next, i, j + 1), next);
                if (maxInRange < next) maxInRange = next;
            }
        }

        return [start, end];
    }
}
