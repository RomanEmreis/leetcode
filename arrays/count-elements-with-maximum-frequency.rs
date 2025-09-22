/*
  3005. Count Elements With Maximum Frequency
  
  You are given an array nums consisting of positive integers.
  Return the total frequencies of elements in nums such that those elements all have the maximum frequency.
  The frequency of an element is the number of occurrences of that element in the array.
  
  Example 1:
  Input: nums = [1,2,2,3,1,4]
  Output: 4
  Explanation: The elements 1 and 2 have a frequency of 2 which is the maximum frequency in the array.
  So the number of elements in the array with maximum frequency is 4.
  
  Example 2:
  Input: nums = [1,2,3,4,5]
  Output: 5
  Explanation: All elements of the array have a frequency of 1 which is the maximum.
  So the number of elements in the array with maximum frequency is 5.
*/
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freq = [0; 101];
        nums.iter().for_each(|&n| freq[n as usize] += 1);
        
        let mut cnt = 0;
        let mut max = 0;
        freq.iter().for_each(|&f| {
            if f > max {
                max = f;
                cnt = f;
            } else if f == max {
                cnt += f;
            }
        });
        cnt
    }
}
