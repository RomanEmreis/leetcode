/*
  1291. Sequential Digits
  
  An integer has sequential digits if and only if each digit in the number is one more than the previous digit.
  
  Return a sorted list of all the integers in the range [low, high] inclusive that have sequential digits.
  
  Example 1:
  Input: low = 100, high = 300
  Output: [123,234]
  
  Example 2:
  Input: low = 1000, high = 13000
  Output: [1234,2345,3456,4567,5678,6789,12345]
*/
const NUMS: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut q = std::collections::VecDeque::from(NUMS);
        while let Some(num) = q.pop_front() {
            if num > high {
                return res;
            }
            if low <= num && num <= high {
                res.push(num);
            }
            let d = num % 10;
            if d < 9 {
                q.push_back(num * 10 + d + 1);
            }
        }
        res
    }
}
