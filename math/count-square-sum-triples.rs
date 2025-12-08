/*
  1925. Count Square Sum Triples
  
  A square triple (a,b,c) is a triple where a, b, and c are integers and a2 + b2 = c2.
  
  Given an integer n, return the number of square triples such that 1 <= a, b, c <= n.
  
  Example 1:
  Input: n = 5
  Output: 2
  Explanation: The square triples are (3,4,5) and (4,3,5).
  
  Example 2:
  Input: n = 10
  Output: 4
  Explanation: The square triples are (3,4,5), (4,3,5), (6,8,10), and (8,6,10).
*/
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut res = 0;
        let mut u = 3;

        while u * u < n * 2 {
            let mut v = 1;
            while v < u && u * u + v * v <= n * 2 {
                if gcd(u, v) == 1 {
                    res += n * 2 / (u * u + v * v);
                }
                v += 2;
            }
            u += 2;
        }

        res * 2
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        (a, b) = (b % a, a);
    }
    b
}
