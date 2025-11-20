/*
  757. Set Intersection Size At Least Two
  
  You are given a 2D integer array intervals where intervals[i] = [starti, endi] represents all the integers from starti to endi inclusively.
  A containing set is an array nums where each interval from intervals has at least two integers in nums.
      For example, if intervals = [[1,3], [3,7], [8,9]], then [1,2,4,7,8,9] and [2,3,4,8,9] are containing sets.
  
  Return the minimum possible size of a containing set.
  
  Example 1:
  Input: intervals = [[1,3],[3,7],[8,9]]
  Output: 5
  Explanation: let nums = [2, 3, 4, 8, 9].
  It can be shown that there cannot be any containing array of size 4.
  
  Example 2:
  Input: intervals = [[1,3],[1,4],[2,5],[3,5]]
  Output: 3
  Explanation: let nums = [2, 3, 4].
  It can be shown that there cannot be any containing array of size 2.
  
  Example 3:
  Input: intervals = [[1,2],[2,3],[2,4],[4,5]]
  Output: 5
  Explanation: let nums = [1, 2, 3, 4, 5].
  It can be shown that there cannot be any containing array of size 4.
*/
impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| if a[1] == b[1] {
            b[0].cmp(&a[0])
        } else {
            a[1].cmp(&b[1])
        });

        let mut res = 0;
        let mut max_1 = -1;
        let mut max_2 = -1;
        
        for int in intervals.iter() {
            let s = int[0];
            let e = int[1];

            if max_1 >= s && max_2 >= s {
                continue;
            }

            if max_1 >= s {
                max_2 = max_1;
                max_1 = e;
                res += 1;
            } else {
                max_1 = e;
                max_2 = e - 1;
                res += 2; 
            }
        }

        res
    }
}
