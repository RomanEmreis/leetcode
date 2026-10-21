/*
  3863. Minimum Operations to Sort a String
  
  You are given a string s consisting of lowercase English letters.
  In one operation, you can select any substring of s that is not the entire string and sort it in non-descending alphabetical order.
  
  Return the minimum number of operations required to make s sorted in non-descending order. If it is not possible, return -1.
   
  Example 1:
  Input: s = "dog"
  Output: 1
  Explanation:​​​​​​​
      Sort substring "og" to "go".
      Now, s = "dgo", which is sorted in ascending order. Thus, the answer is 1.
  
  Example 2:
  Input: s = "card"
  Output: 2
  Explanation:
      Sort substring "car" to "acr", so s = "acrd".
      Sort substring "rd" to "dr", making s = "acdr", which is sorted in ascending order. Thus, the answer is 2.
  
  Example 3:
  Input: s = "gf"
  Output: -1
  Explanation:
      It is impossible to sort s under the given constraints. Thus, the answer is -1.
*/
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();

        if b.is_sorted() {
            return 0;
        }

        let first = b[0];
        let last = b[n - 1];

        if b.iter().all(|&x| x >= first) || b.iter().all(|&x| x <= last) {
            return 1;
        }

        if b[1..].iter().any(|&x| x >= first) || b[..n - 1].iter().any(|&x| x <= last) {
            return 2;
        }

        if n == 2 { -1 } else { 3 }
    }
}
