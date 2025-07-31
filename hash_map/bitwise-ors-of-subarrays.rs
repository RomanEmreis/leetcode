/*
  898. Bitwise ORs of Subarrays
  
  Given an integer array arr, return the number of distinct bitwise ORs of all the non-empty subarrays of arr.
  The bitwise OR of a subarray is the bitwise OR of each integer in the subarray. The bitwise OR of a subarray of one integer is that integer.
  
  A subarray is a contiguous non-empty sequence of elements within an array.
  
  Example 1:
  Input: arr = [0]
  Output: 1
  Explanation: There is only one possible result: 0.
  
  Example 2:
  Input: arr = [1,1,2]
  Output: 3
  Explanation: The possible subarrays are [1], [1], [2], [1, 1], [1, 2], [1, 1, 2].
  These yield the results 1, 1, 2, 1, 3, 3.
  There are 3 unique values, so the answer is 3.
  
  Example 3:
  Input: arr = [1,2,4]
  Output: 6
  Explanation: The possible results are 1, 2, 3, 4, 6, and 7.
*/
use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut sub = Vec::with_capacity(arr.len());
        let mut l = 0;
        arr.into_iter().for_each(|n| {
            let r = sub.len();
            sub.push(n);
            for i in l..r {
                let or = sub[i] | n;
                if *sub.last().unwrap() != or {
                    sub.push(or);
                }
            }
            l = r;
        });
        sub.into_iter()
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
