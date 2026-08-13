/*
  2213. Longest Substring of One Repeating Character
  
  You are given a 0-indexed string s. You are also given a 0-indexed string queryCharacters of length k and a 0-indexed array of integer indices queryIndices of length k, both of which are used to describe k queries.
  The ith query updates the character in s at index queryIndices[i] to the character queryCharacters[i].
  
  Return an array lengths of length k where lengths[i] is the length of the longest substring of s consisting of only one repeating character after the ith query is performed.
  
  Example 1:
  Input: s = "babacc", queryCharacters = "bcb", queryIndices = [1,3,3]
  Output: [3,3,4]
  Explanation: 
  - 1st query updates s = "bbbacc". The longest substring consisting of one repeating character is "bbb" with length 3.
  - 2nd query updates s = "bbbccc". 
    The longest substring consisting of one repeating character can be "bbb" or "ccc" with length 3.
  - 3rd query updates s = "bbbbcc". The longest substring consisting of one repeating character is "bbbb" with length 4.
  Thus, we return [3,3,4].
  
  Example 2:
  Input: s = "abyzz", queryCharacters = "aa", queryIndices = [2,1]
  Output: [2,3]
  Explanation:
  - 1st query updates s = "abazz". The longest substring consisting of one repeating character is "zz" with length 2.
  - 2nd query updates s = "aaazz". The longest substring consisting of one repeating character is "aaa" with length 3.
  Thus, we return [2,3].
*/
impl Solution {
    pub fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
        const WIDTH: u32 = 17;
        const LENGTH_MASK: u64 = (1 << WIDTH) - 1;

        const PREFIX_SHIFT: u32 = 0;
        const SUFFIX_SHIFT: u32 = 17;
        const BEST_SHIFT: u32 = 34;
        const LEFT_CHAR_SHIFT: u32 = 51;
        const RIGHT_CHAR_SHIFT: u32 = 56;
        const UNIFORM_SHIFT: u32 = 61;

        #[inline(always)]
        fn leaf(ch: u8) -> u64 {
            let encoded = (ch - b'a' + 1) as u64;

            1
                | (1 << SUFFIX_SHIFT)
                | (1 << BEST_SHIFT)
                | (encoded << LEFT_CHAR_SHIFT)
                | (encoded << RIGHT_CHAR_SHIFT)
                | (1 << UNIFORM_SHIFT)
        }

        #[inline(always)]
        fn prefix(node: u64) -> u64 {
            (node >> PREFIX_SHIFT) & LENGTH_MASK
        }

        #[inline(always)]
        fn suffix(node: u64) -> u64 {
            (node >> SUFFIX_SHIFT) & LENGTH_MASK
        }

        #[inline(always)]
        fn best(node: u64) -> u64 {
            (node >> BEST_SHIFT) & LENGTH_MASK
        }

        #[inline(always)]
        fn left_char(node: u64) -> u64 {
            (node >> LEFT_CHAR_SHIFT) & 31
        }

        #[inline(always)]
        fn right_char(node: u64) -> u64 {
            (node >> RIGHT_CHAR_SHIFT) & 31
        }

        #[inline(always)]
        fn uniform(node: u64) -> bool {
            node & (1 << UNIFORM_SHIFT) != 0
        }

        #[inline(always)]
        fn merge(left: u64, right: u64) -> u64 {
            if left == 0 {
                return right;
            }

            if right == 0 {
                return left;
            }

            let left_prefix = prefix(left);
            let left_suffix = suffix(left);
            let left_best = best(left);

            let right_prefix = prefix(right);
            let right_suffix = suffix(right);
            let right_best = best(right);

            let connected =
                right_char(left) == left_char(right);

            let result_prefix =
                if connected && uniform(left) {
                    left_prefix + right_prefix
                } else {
                    left_prefix
                };

            let result_suffix =
                if connected && uniform(right) {
                    right_suffix + left_suffix
                } else {
                    right_suffix
                };

            let crossing = if connected {
                left_suffix + right_prefix
            } else {
                0
            };

            let result_best =
                left_best.max(right_best).max(crossing);

            let result_uniform =
                connected && uniform(left) && uniform(right);

            result_prefix
                | (result_suffix << SUFFIX_SHIFT)
                | (result_best << BEST_SHIFT)
                | (left_char(left) << LEFT_CHAR_SHIFT)
                | (right_char(right) << RIGHT_CHAR_SHIFT)
                | ((result_uniform as u64) << UNIFORM_SHIFT)
        }

        let n = s.len();
        let base = n.next_power_of_two();
        let mut tree = vec![0u64; base * 2];

        for (index, ch) in s.bytes().enumerate() {
            tree[base + index] = leaf(ch);
        }

        for node in (1..base).rev() {
            tree[node] = merge(
                tree[node << 1],
                tree[node << 1 | 1],
            );
        }

        let mut res = Vec::with_capacity(query_indices.len());

        for (position, ch) in query_indices
            .into_iter()
            .zip(query_characters.bytes())
        {
            let mut node = base + position as usize;
            let encoded = (ch - b'a' + 1) as u64;

            if left_char(tree[node]) != encoded {
                tree[node] = leaf(ch);
                node >>= 1;

                while node != 0 {
                    tree[node] = merge(
                        tree[node << 1],
                        tree[node << 1 | 1],
                    );

                    node >>= 1;
                }
            }

            res.push(best(tree[1]) as i32);
        }

        res
    }
}
