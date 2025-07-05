/*
  1394. Find Lucky Integer in an Array
  
  Given an array of integers arr, a lucky integer is an integer that has a frequency in the array equal to its value.
  
  Return the largest lucky integer in the array. If there is no lucky integer return -1.
  
  Example 1:
  Input: arr = [2,2,3,4]
  Output: 2
  Explanation: The only lucky number in the array is 2 because frequency[2] == 2.
  
  Example 2:
  Input: arr = [1,2,2,3,3,3]
  Output: 3
  Explanation: 1, 2 and 3 are all lucky numbers, return the largest of them.
  
  Example 3:
  Input: arr = [2,2,2,3,3]
  Output: -1
  Explanation: There are no lucky numbers in the array.
*/
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = [0; 501];
        for num in arr {
            freq[num as usize] += 1;
        }
        for i in (1..=500).rev() {
            let j = i as i32;
            if freq[i] == j {
                return j;
            }
        }
        -1
    }
}
