/*
  2040. Kth Smallest Product of Two Sorted Arrays
  
  Given two sorted 0-indexed integer arrays nums1 and nums2 as well as an integer k, return the kth (1-based) smallest product of nums1[i] * nums2[j] where 0 <= i < nums1.length and 0 <= j < nums2.length.
  
  Example 1:
  Input: nums1 = [2,5], nums2 = [3,4], k = 2
  Output: 8
  Explanation: The 2 smallest products are:
  - nums1[0] * nums2[0] = 2 * 3 = 6
  - nums1[0] * nums2[1] = 2 * 4 = 8
  The 2nd smallest product is 8.
  
  Example 2:
  Input: nums1 = [-4,-2,0,3], nums2 = [2,4], k = 6
  Output: 0
  Explanation: The 6 smallest products are:
  - nums1[0] * nums2[1] = (-4) * 4 = -16
  - nums1[0] * nums2[0] = (-4) * 2 = -8
  - nums1[1] * nums2[1] = (-2) * 4 = -8
  - nums1[1] * nums2[0] = (-2) * 2 = -4
  - nums1[2] * nums2[0] = 0 * 2 = 0
  - nums1[2] * nums2[1] = 0 * 4 = 0
  The 6th smallest product is 0.
  
  Example 3:
  Input: nums1 = [-2,-1,0,1,2], nums2 = [-3,-1,2,4,5], k = 3
  Output: -6
  Explanation: The 3 smallest products are:
  - nums1[0] * nums2[4] = (-2) * 5 = -10
  - nums1[0] * nums2[3] = (-2) * 4 = -8
  - nums1[4] * nums2[0] = 2 * (-3) = -6
  The 3rd smallest product is -6.
*/
impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i64) -> i64 {
        let split1 = nums1.iter().position(|&x| x >= 0).unwrap_or(nums1.len());
        let split2 = nums2.iter().position(|&x| x >= 0).unwrap_or(nums2.len());

        let neg1 = &Vec::from_iter(nums1[0..split1].iter().rev().map(|x| -*x))[0..];
        let pos1 = &nums1[split1..];
        let neg2 = &Vec::from_iter(nums2[0..split2].iter().rev().map(|x| -*x))[0..];
        let pos2 = &nums2[split2..];

        let neg_counter = neg1.len() as i64 * pos2.len() as i64 + neg2.len() as i64 * pos1.len() as i64;
        let mut sign = 1;
        let (neg2, pos2) = if k > neg_counter {
            k -= neg_counter;
            (neg2, pos2)
        } else {
            k = neg_counter - k + 1;
            sign = -1;
            (pos2, neg2)
        };

        let mut l = 0;
        let mut r = 10_000_000_000;

        while l < r {
            let mid = (l + r) >> 1;
            if Self::count(neg1, neg2, mid) + Self::count(pos1, pos2, mid) >= k {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l * sign
    }

    fn count(a: &[i32], b: &[i32], val: i64) -> i64 {
        let mut result = 0;
        let mut index = b.len() as i64 - 1;
        for &number in a.iter() {
            while index >= 0 && number as i64 * b[index as usize] as i64 > val {
                index -= 1;
            }
            result += index + 1;
        }
        result
    }
}
