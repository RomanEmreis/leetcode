/*
  2438. Range Product Queries of Powers
  
  Given a positive integer n, there exists a 0-indexed array called powers, composed of the minimum number of powers of 2 that sum to n. The array is sorted in non-decreasing order, and there is only one way to form the array.
  You are also given a 0-indexed 2D integer array queries, where queries[i] = [lefti, righti]. Each queries[i] represents a query where you have to find the product of all powers[j] with lefti <= j <= righti.
  
  Return an array answers, equal in length to queries, where answers[i] is the answer to the ith query. Since the answer to the ith query may be too large, each answers[i] should be returned modulo 109 + 7.
  
  Example 1:
  Input: n = 15, queries = [[0,1],[2,2],[0,3]]
  Output: [2,4,64]
  Explanation:
  For n = 15, powers = [1,2,4,8]. It can be shown that powers cannot be a smaller size.
  Answer to 1st query: powers[0] * powers[1] = 1 * 2 = 2.
  Answer to 2nd query: powers[2] = 4.
  Answer to 3rd query: powers[0] * powers[1] * powers[2] * powers[3] = 1 * 2 * 4 * 8 = 64.
  Each answer modulo 109 + 7 yields the same answer, so [2,4,64] is returned.
  
  Example 2:
  Input: n = 2, queries = [[0,0]]
  Output: [2]
  Explanation:
  For n = 2, powers = [2].
  The answer to the only query is powers[0] = 2. The answer modulo 109 + 7 is the same, so [2] is returned.
*/
impl Solution {
    pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut bins = Vec::new();
        let mut n = n;
        let mut rep = 1;
        while n > 0 {
            if n % 2 == 1 {
                bins.push(rep);
            }
            n /= 2;
            rep *= 2;
        }

        const MOD: i64 = 1_000_000_007;
        let m = bins.len();
        let mut results = vec![vec![0; m]; m];
        for i in 0..m {
            let mut cur: i64 = 1;
            for j in i..m {
                cur = (cur * bins[j]) % MOD;
                results[i][j] = cur as i32;
            }
        }

        queries
            .iter()
            .map(|q| results[q[0] as usize][q[1] as usize])
            .collect()
    }
}
