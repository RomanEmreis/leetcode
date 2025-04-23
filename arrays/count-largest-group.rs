/*
  1399. Count Largest Group
  
  You are given an integer n.
  Each number from 1 to n is grouped according to the sum of its digits.
  Return the number of groups that have the largest size.
  
  Example 1:
  Input: n = 13
  Output: 4
  Explanation: There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
  [1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9].
  There are 4 groups with largest size.
  
  Example 2:
  Input: n = 2
  Output: 2
  Explanation: There are 2 groups [1], [2] of size 1.
*/
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut count = [0; 37];
        for i in 1..=n {
            let mut d = i;
            let mut sum = 0;
            while d > 0 {
                sum += d % 10;
                d /= 10;
            }

            count[sum as usize] += 1;
        }

        let larger = count.iter()
            .max()
            .unwrap();
            
        count.iter()
            .filter(|&c| c == larger)
            .count() as i32
    }
}
