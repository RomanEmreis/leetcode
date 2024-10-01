/*
  Given an array of integers arr of even length n and an integer k.
  
  We want to divide the array into exactly n / 2 pairs such that the sum of each pair is divisible by k.
  
  Return true If you can find a way to do that or false otherwise.
  
  Example 1:
    Input: arr = [1,2,3,4,5,10,6,7,8,9], k = 5
    Output: true
    Explanation: Pairs are (1,9),(2,8),(3,7),(4,6) and (5,10).
*/
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut freq = vec![0; k as usize];
        for n in arr {
            let rem = ((n % k) + k) % k;
            freq[rem as usize] += 1;
        }

        if freq[0] % 2 != 0 {
            return false;
        }

        let k = k as usize;

        for i in 1..=(freq.len() / 2) {
            if freq[i] != freq[k - i] {
                return false;
            }
        }

        true
    }
}
