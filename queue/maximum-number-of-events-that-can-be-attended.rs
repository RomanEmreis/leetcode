/*
  1353. Maximum Number of Events That Can Be Attended
  
  You are given an array of events where events[i] = [startDayi, endDayi]. Every event i starts at startDayi and ends at endDayi.
  You can attend an event i at any day d where startTimei <= d <= endTimei. You can only attend one event at any time d.
  
  Return the maximum number of events you can attend.
  
  Example 1:
  Input: events = [[1,2],[2,3],[3,4]]
  Output: 3
  Explanation: You can attend all the three events.
  One way to attend them all is as shown.
  Attend the first event on day 1.
  Attend the second event on day 2.
  Attend the third event on day 3.
  
  Example 2:
  Input: events= [[1,2],[2,3],[3,4],[1,2]]
  Output: 4
*/
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by(|x, y| x[0].cmp(&y[0]));

        let n = events.len();
        let mut result = 0;
        let mut day = 0;

        let mut min_heap = BinaryHeap::with_capacity(n);
        let mut i = 0;
        while !min_heap.is_empty() || i < n {
            if min_heap.is_empty() {
                day = events[i][0];
            }
            while i < n && events[i][0] == day {
                min_heap.push(Reverse(events[i][1]));
                i += 1;
            }
            
            let _ = min_heap.pop();
            result += 1;
            day += 1;

            while let Some(e) = min_heap.peek() {
                if e.0 < day {
                    let _ = min_heap.pop();
                } else {
                    break;
                }
            }
        }
        result
    }
}
