/*
  1733. Minimum Number of People to Teach
  
  On a social network consisting of m users and some friendships between users, two users can communicate with each other if they know a common language.
  You are given an integer n, an array languages, and an array friendships where:
    There are n languages numbered 1 through n,
    languages[i] is the set of languages the i​​​​​​th​​​​ user knows, and
    friendships[i] = [u​​​​​​i​​​, v​​​​​​i] denotes a friendship between the users u​​​​​​​​​​​i​​​​​ and vi.
  
  You can choose one language and teach it to some users so that all friends can communicate with each other. Return the minimum number of users you need to teach.
  
  Note that friendships are not transitive, meaning if x is a friend of y and y is a friend of z, this doesn't guarantee that x is a friend of z.
  
  Example 1:
  Input: n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
  Output: 1
  Explanation: You can either teach user 1 the second language or user 2 the first language.
  
  Example 2:
  Input: n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
  Output: 2
  Explanation: Teach the third language to users 1 and 3, yielding two users to teach.
*/
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = languages.len();

        let chunks = (n + 63) / 64;

        let mut lang_bits: Vec<Vec<u64>> = Vec::with_capacity(m + 1);
        lang_bits.push(vec![0; chunks]);
        for langs in languages {
            let mut bits = vec![0u64; chunks];
            for l in langs {
                let idx = (l as usize - 1) / 64;
                let off = (l as usize - 1) % 64;
                bits[idx] |= 1u64 << off;
            }
            lang_bits.push(bits);
        }

        fn can_talk(a: &[u64], b: &[u64]) -> bool {
            a.iter().zip(b).any(|(x, y)| x & y != 0)
        }

        let mut need_teach = Vec::new();
        let mut mark = vec![false; m + 1];
        for f in friendships {
            let u = f[0] as usize;
            let v = f[1] as usize;
            if !can_talk(&lang_bits[u], &lang_bits[v]) {
                if !mark[u] {
                    mark[u] = true;
                    need_teach.push(u);
                }
                if !mark[v] {
                    mark[v] = true;
                    need_teach.push(v);
                }
            }
        }

        if need_teach.is_empty() {
            return 0;
        }

        let mut freq = vec![0; n];
        for &u in &need_teach {
            for (chunk_idx, &chunk) in lang_bits[u].iter().enumerate() {
                if chunk == 0 { continue; }
                let mut mask = chunk;
                while mask != 0 {
                    let bit = mask.trailing_zeros() as usize;
                    freq[chunk_idx * 64 + bit] += 1;
                    mask &= mask - 1;
                }
            }
        }

        let req = need_teach.len() as i32;
        let best_known = freq
            .into_iter()
            .max()
            .unwrap_or(0) as i32;
        req - best_known
    }
}
