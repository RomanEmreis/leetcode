/*
  3318. Find X-Sum of All K-Long Subarrays I
  
  You are given an array nums of n integers and two integers k and x.
  The x-sum of an array is calculated by the following procedure:
      Count the occurrences of all elements in the array.
      Keep only the occurrences of the top x most frequent elements. If two elements have the same number of occurrences, the element with the bigger value is considered more frequent.
      Calculate the sum of the resulting array.
  
  Note that if an array has less than x distinct elements, its x-sum is the sum of the array.
  
  Return an integer array answer of length n - k + 1 where answer[i] is the x-sum of the nums[i..i + k - 1].
  
  Example 1:
  Input: nums = [1,1,2,2,3,4,2,3], k = 6, x = 2
  Output: [6,10,12]
  Explanation:
      For subarray [1, 1, 2, 2, 3, 4], only elements 1 and 2 will be kept in the resulting array. Hence, answer[0] = 1 + 1 + 2 + 2.
      For subarray [1, 2, 2, 3, 4, 2], only elements 2 and 4 will be kept in the resulting array. Hence, answer[1] = 2 + 2 + 2 + 4. Note that 4 is kept in the array since it is bigger than 3 and 1 which occur the same number of times.
      For subarray [2, 2, 3, 4, 2, 3], only elements 2 and 3 are kept in the resulting array. Hence, answer[2] = 2 + 2 + 2 + 3 + 3.
  
  Example 2:
  Input: nums = [3,8,7,8,7,5], k = 2, x = 2
  Output: [11,15,15,15,12]
  Explanation:
  Since k == x, answer[i] is equal to the sum of the subarray nums[i..i + k - 1].
*/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let x = x as usize;
        let mut res = Vec::new();
        for i in 0..=nums.len() - k {
            let mut freq = [0i32; 51];
            for &n in &nums[i..i + k] {
                freq[n as usize] += 1;
            }

            let mut pq = BinaryHeap::new();

            for (j, &count) in freq.iter().enumerate() {
                if count == 0 {
                    continue;
                }
                pq.push(Reverse((count, j as i32)));
                if pq.len() > x {
                    pq.pop();
                }
            }
            let mut sum = 0;
            while let Some(Reverse((count, num))) = pq.pop() {
                sum += num * count;
            }

            res.push(sum);
        }

        res
    }
}
