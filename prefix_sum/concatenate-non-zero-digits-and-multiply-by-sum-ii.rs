/*
3756. Concatenate Non-Zero Digits and Multiply by Sum II

You are given a string s of length m consisting of digits. You are also given a 2D integer array queries, where queries[i] = [li, ri].
For each queries[i], extract the s[li..ri]. Then, perform the following:
    Form a new integer x by concatenating all the non-zero digits from the substring in their original order. If there are no non-zero digits, x = 0.
    Let sum be the sum of digits in x. The answer is x * sum.

Return an array of integers answer where answer[i] is the answer to the ith query.

Since the answers may be very large, return them modulo 109 + 7.

Example 1:
Input: s = "10203004", queries = [[0,7],[1,3],[4,6]]
Output: [12340, 4, 9]
Explanation:
    s[0..7] = "10203004"
        x = 1234
        sum = 1 + 2 + 3 + 4 = 10
        Therefore, answer is 1234 * 10 = 12340.
    s[1..3] = "020"
        x = 2
        sum = 2
        Therefore, the answer is 2 * 2 = 4.
    s[4..6] = "300"
        x = 3
        sum = 3
        Therefore, the answer is 3 * 3 = 9.

Example 2:
Input: s = "1000", queries = [[0,3],[1,1]]
Output: [1, 0]
Explanation:
    s[0..3] = "1000"
        x = 1
        sum = 1
        Therefore, the answer is 1 * 1 = 1.
    s[1..1] = "0"
        x = 0
        sum = 0
        Therefore, the answer is 0 * 0 = 0.

Example 3:
Input: s = "9876543210", queries = [[0,9]]
Output: [444444137]
Explanation:
    s[0..9] = "9876543210"
        x = 987654321
        sum = 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1 = 45
        Therefore, the answer is 987654321 * 45 = 44444444445.
        We return 44444444445 modulo (109 + 7) = 444444137.
*/
use std::sync::OnceLock;

const MX: usize = 100001;
const MOD: i64 = 1000000007;

static POW10: OnceLock<Vec<i64>> = OnceLock::new();

#[derive(Clone, Copy, Default)]
struct Pref {
    p: i64,
    sum: i32,
    cnt: i32,
}

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let b = s.as_bytes();
        let n = b.len();

        let mut pref = vec![Pref::default(); n + 1];
        for i in 1..=n {
            let d = i64::from(b[i - 1] - b'0');
            let prev = pref[i - 1];
            pref[i] = if d > 0 {
                Pref {
                    p: (prev.p * 10 + d) % MOD,
                    sum: prev.sum + d as i32,
                    cnt: prev.cnt + 1,
                }
            } else {
                Pref { sum: prev.sum, ..prev }
            };
        }

        let pow = pow10();

        queries
            .into_iter()
            .map(|q| {
                let (lo, hi) = (pref[q[0] as usize], pref[q[1] as usize + 1]);
                let n0 = (hi.cnt - lo.cnt) as usize;
                let sd = i64::from(hi.sum - lo.sum);
                let num = (hi.p - lo.p * pow[n0] % MOD + MOD) % MOD;
                (num * sd % MOD) as i32
            })
            .collect()
    }
}

#[inline]
fn pow10() -> &'static Vec<i64> {
    POW10.get_or_init(|| {
        let mut p = vec![1i64; MX];
        for i in 1..MX {
            p[i] = (p[i - 1] * 10) % MOD;
        }
        p
    })
}
