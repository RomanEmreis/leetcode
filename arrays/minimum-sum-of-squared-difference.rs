/*
  2333. Minimum Sum of Squared Difference
  
  You are given two positive 0-indexed integer arrays nums1 and nums2, both of length n.
  The sum of squared difference of arrays nums1 and nums2 is defined as the sum of (nums1[i] - nums2[i])2 for each 0 <= i < n.
  You are also given two positive integers k1 and k2. You can modify any of the elements of nums1 by +1 or -1 at most k1 times. 
  Similarly, you can modify any of the elements of nums2 by +1 or -1 at most k2 times.
  
  Return the minimum sum of squared difference after modifying array nums1 at most k1 times and modifying array nums2 at most k2 times.
  
  Note: You are allowed to modify the array elements to become negative integers.
   
  Example 1:
  Input: nums1 = [1,2,3,4], nums2 = [2,10,20,19], k1 = 0, k2 = 0
  Output: 579
  Explanation: The elements in nums1 and nums2 cannot be modified because k1 = 0 and k2 = 0. 
  The sum of square difference will be: (1 - 2)2 + (2 - 10)2 + (3 - 20)2 + (4 - 19)2 = 579.
  
  Example 2:
  Input: nums1 = [1,4,10,12], nums2 = [5,8,6,9], k1 = 1, k2 = 1
  Output: 43
  Explanation: One way to obtain the minimum sum of square difference is: 
  - Increase nums1[0] once.
  - Increase nums2[2] once.
  The minimum of the sum of square difference will be: 
  (2 - 5)2 + (4 - 8)2 + (10 - 7)2 + (12 - 9)2 = 43.
  Note that, there are other ways to obtain the minimum of the sum of square difference, but there is no way to obtain a sum smaller than 43.
*/
impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        const MAX_DIFFERENCE: usize = 100_000;

        let mut frequency = vec![0i32; MAX_DIFFERENCE + 1];
        let mut highest = 0usize;
        let mut total_difference = 0i64;

        for (&left, &right) in nums1.iter().zip(&nums2) {
            let difference = (i64::from(left) - i64::from(right)).abs() as usize;

            frequency[difference] += 1;
            highest = highest.max(difference);
            total_difference += difference as i64;
        }

        let mut operations = i64::from(k1) + i64::from(k2);

        if operations >= total_difference {
            return 0;
        }

        for difference in (1..=highest).rev() {
            let available = i64::from(frequency[difference]);

            if available == 0 {
                continue;
            }

            let moved = operations.min(available);

            frequency[difference] -= moved as i32;
            frequency[difference - 1] += moved as i32;
            operations -= moved;

            if operations == 0 {
                break;
            }
        }

        frequency
            .iter()
            .enumerate()
            .map(|(difference, &count)| {
                let difference = difference as i64;

                i64::from(count) * difference * difference
            })
            .sum()
    }
}
