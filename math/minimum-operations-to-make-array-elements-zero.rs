/*
  3495. Minimum Operations to Make Array Elements Zero
  
  You are given a 2D array queries, where queries[i] is of the form [l, r]. Each queries[i] defines an array of integers nums consisting of elements ranging from l to r, both inclusive.
  In one operation, you can:
      Select two integers a and b from the array.
      Replace them with floor(a / 4) and floor(b / 4).
  
  Your task is to determine the minimum number of operations required to reduce all elements of the array to zero for each query. Return the sum of the results for all queries.
  
  Example 1:
  Input: queries = [[1,2],[2,4]]
  Output: 3
  Explanation:
  For queries[0]:
      The initial array is nums = [1, 2].
      In the first operation, select nums[0] and nums[1]. The array becomes [0, 0].
      The minimum number of operations required is 1.
  For queries[1]:
      The initial array is nums = [2, 3, 4].
      In the first operation, select nums[0] and nums[2]. The array becomes [0, 3, 1].
      In the second operation, select nums[1] and nums[2]. The array becomes [0, 0, 0].
      The minimum number of operations required is 2.
  The output is 1 + 2 = 3.
  
  Example 2:
  Input: queries = [[2,6]]
  Output: 4
  Explanation:
  For queries[0]:
      The initial array is nums = [2, 3, 4, 5, 6].
      In the first operation, select nums[0] and nums[3]. The array becomes [0, 3, 4, 1, 6].
      In the second operation, select nums[2] and nums[4]. The array becomes [0, 3, 1, 1, 1].
      In the third operation, select nums[1] and nums[2]. The array becomes [0, 0, 0, 1, 1].
      In the fourth operation, select nums[3] and nums[4]. The array becomes [0, 0, 0, 0, 0].
      The minimum number of operations required is 4.
  The output is 4.
*/
impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        fn get_ops(n: i32) -> i64 {
            let mut res = 0;
            let mut ops = 0;
            let mut pow = 1;
            while pow <= n {
                let l = pow;
                let r = n.min(4 * pow - 1);
                ops += 1;
                res += (r - l + 1) as i64 * ops;
                pow *= 4;
            }
            res
        }
        let mut res = 0;
        for q in queries.iter() {
            let l = q[0];
            let r = q[1];
            res += (get_ops(r) - get_ops(l - 1) + 1) / 2;
        }
        res
    }
}
