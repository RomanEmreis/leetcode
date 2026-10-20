/*
  3886. Sum of Sortable Integers
  
  You are given an integer array nums of length n.
  An integer k is called sortable if k divides n and you can sort nums in non-decreasing order by sequentially performing the following operations:
      Partition nums into consecutive subarrays of length k.
      Cyclically rotate each subarray independently any number of times to the left or to the right.
  
  Return an integer denoting the sum of all possible sortable integers k.
  
  Example 1:
  Input: nums = [3,1,2]
  Output: 3
  Explanation:​​​​​​​
      For n = 3, possible divisors are 1 and 3.
      For k = 1: each subarray has one element. No rotation can sort the array.
      For k = 3: the single subarray [3, 1, 2] can be rotated once to produce [1, 2, 3], which is sorted.
      Only k = 3 is sortable. Hence, the answer is 3.
  
  Example 2:
  Input: nums = [7,6,5]
  Output: 0
  Explanation:
      For n = 3, possible divisors are 1 and 3.
      For k = 1: each subarray has one element. No rotation can sort the array.
      For k = 3: the single subarray [7, 6, 5] cannot be rotated into non-decreasing order.
      No k is sortable. Hence, the answer is 0.
  
  Example 3:
  Input: nums = [5,8]
  Output: 3
  Explanation:​​​​​​​
      For n = 2, possible divisors are 1 and 2.
      Since [5, 8] is already sorted, every divisor is sortable. Hence, the answer is 1 + 2 = 3.
*/
impl Solution {
    fn is_sortable(nums: &[i32], desc: &[u32], k: usize) -> bool {
        let n = nums.len();
        if desc.len() > n / k {
            return false;
        }

        let mut prev_max = i32::MIN;
        let mut idx = 0;

        for s in (0..n).step_by(k) {
            let e = s + k;

            let mut first = 0usize;
            let mut cnt = 0u8;
            while idx < desc.len() && (desc[idx] as usize) < e - 1 {
                if cnt == 0 {
                    first = desc[idx] as usize;
                }
                cnt += 1;
                if cnt > 1 {
                    return false;
                }
                idx += 1;
            }

            if idx < desc.len() && desc[idx] as usize == e - 1 {
                return false;
            }

            let (mn, mx) = if cnt == 0 {
                (nums[s], nums[e - 1])
            } else {
                if nums[e - 1] > nums[s] {
                    return false;
                }
                (nums[first + 1], nums[first])
            };

            if prev_max > mn {
                return false;
            }
            prev_max = mx;
        }
        true
    }

    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut p = 1usize;
        {
            let mut d = 2;
            while d * d <= n {
                if n % d == 0 {
                    p = d;
                    break;
                }
                d += 1;
            }
            if p == 1 && n > 1 {
                p = n;
            }
        }

        let total = nums.windows(2).filter(|w| w[0] > w[1]).count();
        if total > n / p {
            return 0;
        }

        let mut desc: Vec<u32> = Vec::with_capacity(total);
        for i in 0..n.saturating_sub(1) {
            if nums[i] > nums[i + 1] {
                desc.push(i as u32);
            }
        }

        let mut res = 0i32;
        let mut div = 1usize;
        while div * div <= n {
            if n % div == 0 {
                if Self::is_sortable(&nums, &desc, div) {
                    res += div as i32;
                }
                let paired = n / div;
                if paired != div && Self::is_sortable(&nums, &desc, paired) {
                    res += paired as i32;
                }
            }
            div += 1;
        }
        res
    }
}
