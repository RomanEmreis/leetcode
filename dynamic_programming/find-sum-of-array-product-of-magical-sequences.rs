/*
  3539. Find Sum of Array Product of Magical Sequences
  
  You are given two integers, m and k, and an integer array nums.
  A sequence of integers seq is called magical if:
      seq has a size of m.
      0 <= seq[i] < nums.length
      The binary representation of 2seq[0] + 2seq[1] + ... + 2seq[m - 1] has k set bits.
  
  The array product of this sequence is defined as prod(seq) = (nums[seq[0]] * nums[seq[1]] * ... * nums[seq[m - 1]]).
  
  Return the sum of the array products for all valid magical sequences.
  
  Since the answer may be large, return it modulo 109 + 7.
  A set bit refers to a bit in the binary representation of a number that has a value of 1.
  
  Example 1:
  Input: m = 5, k = 5, nums = [1,10,100,10000,1000000]
  Output: 991600007
  Explanation:
  All permutations of [0, 1, 2, 3, 4] are magical sequences, each with an array product of 1013.
  
  Example 2:
  Input: m = 2, k = 2, nums = [5,4,3,2,1]
  Output: 170
  Explanation:
  The magical sequences are [0, 1], [0, 2], [0, 3], [0, 4], [1, 0], [1, 2], [1, 3], [1, 4], [2, 0], [2, 1], [2, 3], [2, 4], [3, 0], [3, 1], [3, 2], [3, 4], [4, 0], [4, 1], [4, 2], and [4, 3].
  
  Example 3:
  Input: m = 1, k = 1, nums = [28]
  Output: 28
  Explanation:
  The only magical sequence is [0].
*/
impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let m = m as usize;
        let k = k as usize;
        let comb = get_comb(m, m);
        let mut mem = vec![vec![vec![vec![-1; m + 1]; nums.len() + 1]; k + 1]; m + 1];
        dp(m, k, 0, 0, &nums, &comb, &mut mem)
    }
}

const MOD: i64 = 1_000_000_007;

fn dp(
    m: usize, 
    k: usize, 
    i: usize, 
    carry: usize, 
    nums: &Vec<i32>, 
    comb: &Vec<Vec<i64>>, 
    mem: &mut Vec<Vec<Vec<Vec<i32>>>>
) -> i32 {
    let pop_count = carry.count_ones() as usize;
    if m + pop_count < k {
        return 0;
    }
    if m == 0 {
        return if k == pop_count { 1 } else { 0 };
    }
    if i == nums.len() {
        return 0;
    }
    if mem[m][k][i][carry] != -1 {
        return mem[m][k][i][carry];
    }

    let mut res = 0;
    for count in 0..=m {
        let contribution = comb[m][count] * mod_pow(nums[i] as i64, count as i64) % MOD;
        let new_carry = carry + count;
        res = ((res as i64 
            + dp(m - count, k - (new_carry % 2), i + 1, new_carry / 2, nums, comb, mem) as i64
            * contribution)
        % MOD) as i32;
    }
    mem[m][k][i][carry as usize] = res;
    res
}

#[inline]
fn get_comb(n: usize, k: usize) -> Vec<Vec<i64>> {
    let mut comb = vec![vec![0; n + 1]; k + 1];
    for i in 0..=n {
        comb[i][0] = 1;
    }
    for i in 1..=n {
        for j in 1..=k {
            comb[i][j] = comb[i - 1][j] + comb[i - 1][j - 1];
        }
    }
    comb
}

fn mod_pow(x: i64, n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    (if n % 2 == 1 {
        x * mod_pow(x % MOD, n - 1)
    } else {
        mod_pow(x * x % MOD, n / 2)
    }) % MOD
}
