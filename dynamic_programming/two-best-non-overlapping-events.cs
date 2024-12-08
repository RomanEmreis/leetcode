/*
  2054. Two Best Non-Overlapping Events
  
  You are given a 0-indexed 2D integer array of events where events[i] = [startTimei, endTimei, valuei]. The ith event starts at startTimei and ends at endTimei,
  and if you attend this event, you will receive a value of valuei. You can choose at most two non-overlapping events to attend such that the sum of their values is maximized.
  Return this maximum sum.
  
  Note that the start time and end time is inclusive: that is, you cannot attend two events where one of them starts and the other ends at the same time. More specifically, if you attend an event with end time t, the next event must start at or after t + 1.
  
  Example 1:
  Input: events = [[1,3,2],[4,5,2],[2,4,3]]
  Output: 4
  Explanation: Choose the green events, 0 and 1 for a sum of 2 + 2 = 4.
  
  Example 2:
  Example 1 Diagram
  Input: events = [[1,3,2],[4,5,2],[1,5,5]]
  Output: 5
  Explanation: Choose event 2 for a sum of 5.
  
  Example 3:
  Input: events = [[1,5,3],[1,5,1],[6,6,5]]
  Output: 8
  Explanation: Choose events 0 and 2 for a sum of 3 + 5 = 8.
*/
public class Solution {
    public int MaxTwoEvents(int[][] events) {
        int n = events.Length;
        Array.Sort(events, (a, b) => a[0].CompareTo(b[0]));

        Span<int> dp = stackalloc int[n + 1];
        for (int i = n - 1; i >= 0; --i) {
            dp[i] = Math.Max(dp[i + 1], events[i][2]);
        }

        int result = 0;
        foreach (int[] evnt in events) {
            int value = evnt[2];
            int l = 0, r = n;
            while (l < r) {
                int m = (l + r) >> 1;
                if (events[m][0] > evnt[1]) r = m;
                else l = m + 1;
            }
            if (l < n) value += dp[l];
            result = Math.Max(result, value);
        }
        return result;
    }
}
