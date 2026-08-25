/*
  3718. Smallest Missing Multiple of K
  
  Given an integer array nums and an integer k, return the smallest positive multiple of k that is missing from nums.
  
  A multiple of k is any positive integer divisible by k.
   
  Example 1:
  Input: nums = [8,2,3,4,6], k = 2
  Output: 10
  Explanation:
  The multiples of k = 2 are 2, 4, 6, 8, 10, 12... and the smallest multiple missing from nums is 10.
  
  Example 2:
  Input: nums = [1,4,7,10,15], k = 5
  Output: 5
  Explanation:
  The multiples of k = 5 are 5, 10, 15, 20... and the smallest multiple missing from nums is 5.
*/
impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let freq = nums
            .into_iter()
            .fold([false; 101], |mut f, n| {
                f[n as usize] = true;
                f
            });

        (k..=100)
            .step_by(k as usize)
            .find(|&i| !freq[i as usize])
            .unwrap_or((100 / k + 1) * k) 
    }
}
