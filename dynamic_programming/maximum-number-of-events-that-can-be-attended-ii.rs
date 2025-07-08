/*
  1751. Maximum Number of Events That Can Be Attended II
  
  You are given an array of events where events[i] = [startDayi, endDayi, valuei]. The ith event starts at startDayi and ends at endDayi, 
  and if you attend this event, you will receive a value of valuei. You are also given an integer k which represents the maximum number of events you can attend.
  You can only attend one event at a time. If you choose to attend an event, you must attend the entire event. Note that the end day is inclusive: 
  that is, you cannot attend two events where one of them starts and the other ends on the same day.
  
  Return the maximum sum of values that you can receive by attending events.
  
  Example 1:
  Input: events = [[1,2,4],[3,4,3],[2,3,1]], k = 2
  Output: 7
  Explanation: Choose the green events, 0 and 1 (0-indexed) for a total value of 4 + 3 = 7.
  
  Example 2:
  Input: events = [[1,2,4],[3,4,3],[2,3,10]], k = 2
  Output: 10
  Explanation: Choose event 2 for a total value of 10.
  Notice that you cannot attend any other event as they overlap, and that you do not have to attend k events.
  
  Example 3:
  Input: events = [[1,1,1],[2,2,2],[3,3,3],[4,4,4]], k = 3
  Output: 9
  Explanation: Although the events do not overlap, you can only attend 3 events. Pick the highest valued three.
*/
impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        events.sort_by_key(|x| x[1]);

        let k = k as usize;
        let n = events.len();
        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 1..=n {
            let start = events[i - 1][0];
            let value = events[i - 1][2];
            let p = Self::binary_search(&events, start, i - 1);
            for j in 1..=k {
                dp[i][j] = dp[i - 1][j].max(dp[p][j - 1] + value);
            }
        }

        dp[n][k]
    }

    fn binary_search(events: &Vec<Vec<i32>>, x: i32, hi: usize) -> usize {
        let mut l = 0;
        let mut r = hi;
        while l < r {
            let m = (l + r) >> 1;
            if events[m][1] >= x {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}
