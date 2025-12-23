/*
  2054. Two Best Non-Overlapping Events
  
  You are given a 0-indexed 2D integer array of events where events[i] = [startTimei, endTimei, valuei]. 
  The ith event starts at startTimei and ends at endTimei, and if you attend this event, you will receive a value of valuei.
  You can choose at most two non-overlapping events to attend such that the sum of their values is maximized.
  
  Return this maximum sum.
  
  Note that the start time and end time is inclusive: that is, you cannot attend two events where one of them starts and the other ends at the same time.
  More specifically, if you attend an event with end time t, the next event must start at or after t + 1.
  
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
impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let n = events.len();

        let mut dp = vec![0; n + 1];
        let mut res = 0;

        for i in (0..=n - 1).rev() {
            dp[i] = dp[i + 1].max(events[i][2]);
        }

        for e in events.iter() {
            let end = e[1];
            let mut val = e[2];
            let mut l = 0;
            let mut r = n;
            while l < r {
                let m = (l + r) >> 1;
                if events[m][0] > end {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            if l < n {
                val += dp[l];
            }
            res = res.max(val);
        }

        res
    }
}
