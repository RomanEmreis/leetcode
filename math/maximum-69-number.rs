/*
  1323. Maximum 69 Number
  
  Example 1:
  Input: num = 9669
  Output: 9969
  Explanation: 
  Changing the first digit results in 6669.
  Changing the second digit results in 9969.
  Changing the third digit results in 9699.
  Changing the fourth digit results in 9666.
  The maximum number is 9969.
  
  Example 2:
  Input: num = 9996
  Output: 9999
  Explanation: Changing the last digit 6 to 9 results in the maximum number.
  
  Example 3:
  Input: num = 9999
  Output: 9999
  Explanation: It is better not to apply any change.
*/
impl Solution {
    pub fn maximum69_number(mut num: i32) -> i32 {
        let n = num.ilog10() as usize + 1;
        let mut nums = vec![0; n];
        let mut i = n - 1;
        while num > 0 {
            nums[i] = num % 10;
            num /= 10;
            i -= 1;
        }
        for i in 0..n {
            if nums[i] == 6 {
                nums[i] = 9;
                break;
            }
        }
        nums.iter().fold(0, |acc, &x| acc * 10 + x)
    }
}
