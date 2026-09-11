/*
  3483. Unique 3-Digit Even Numbers
  
  You are given an array of digits called digits. Your task is to determine the number of distinct three-digit even numbers that can be formed using these digits.
  Note: Each copy of a digit can only be used once per number, and there may not be leading zeros.
  
  Example 1:
  Input: digits = [1,2,3,4]
  Output: 12
  Explanation: The 12 distinct 3-digit even numbers that can be formed are 124, 132, 134, 142, 214, 234, 312, 314, 324, 342, 412, and 432. 
  Note that 222 cannot be formed because there is only 1 copy of the digit 2.
  
  Example 2:
  Input: digits = [0,2,2]
  Output: 2
  Explanation: The only 3-digit even numbers that can be formed are 202 and 220. Note that the digit 2 can be used twice because it appears twice in the array.
  
  Example 3:
  Input: digits = [6,6,6]
  Output: 1
  Explanation: Only 666 can be formed.
  
  Example 4:
  Input: digits = [1,3,5]
  Output: 0
  Explanation: No even 3-digit numbers can be formed.
*/
impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut freq = digits
            .into_iter()
            .fold([0; 10], |mut f, n| {
                f[n as usize] += 1;
                f
            });

        let units = [0, 2, 4, 6, 8];
        let mut res = 0;

        for h in 1..=9 {
            if freq[h] == 0 {
                continue;
            }

            freq[h] -= 1;

            for t in 0..=9 {
                if freq[t] == 0 {
                    continue;
                }

                freq[t] -= 1;

                for u in units {
                    if freq[u] > 0 {
                        res += 1;
                    }
                }

                freq[t] += 1;
            }

            freq[h] += 1;
        }

        res
    }
}
