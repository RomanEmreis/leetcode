/*
  1288. Remove Covered Intervals
  
  Given an array intervals where intervals[i] = [li, ri] represent the interval [li, ri), remove all intervals that are covered by another interval in the list.
  The interval [a, b) is covered by the interval [c, d) if and only if c <= a and b <= d.
  
  Return the number of remaining intervals.
  
  Example 1:
  Input: intervals = [[1,4],[3,6],[2,8]]
  Output: 2
  Explanation: Interval [3,6] is covered by [2,8], therefore it is removed.
  
  Example 2:
  Input: intervals = [[1,4],[2,3]]
  Output: 1
*/
impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            b[1].cmp(&a[1])
        });
        
        let (res, _) = intervals.into_iter().fold((0, 0), |(res, prev), interval| {
            if prev < interval[1] {
                (res + 1, interval[1])
            } else {
                (res, prev)
            }
        });
        
        res
    }
}
