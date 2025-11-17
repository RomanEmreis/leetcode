/*
  1437. Check If All 1's Are at Least Length K Places Away
  
  Given a binary array nums and an integer k, return true if all 1's are at least k places away from each other, otherwise return false.
  
  Example 1:
  Input: nums = [1,0,0,0,1,0,0,1], k = 2
  Output: true
  Explanation: Each of the 1s are at least 2 places away from each other.
  
  Example 2:
  Input: nums = [1,0,0,1,0,1], k = 2
  Output: false
  Explanation: The second 1 and third 1 are only one apart from each other.
*/
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut dist = k;
        for &n in &nums {
            if n == 0 {
                dist += 1;
                continue;
            }
            if dist < k {
                return false;
            }
            dist = 0;
        }
        true
    }
}
