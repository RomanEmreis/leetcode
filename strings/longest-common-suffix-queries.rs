/*
  3093. Longest Common Suffix Queries
  
  You are given two arrays of strings wordsContainer and wordsQuery.
  For each wordsQuery[i], you need to find a string from wordsContainer that has the longest common suffix with wordsQuery[i].
  If there are two or more strings in wordsContainer that share the longest common suffix, find the string that is the smallest in length. 
  If there are two or more such strings that have the same smallest length, find the one that occurred earlier in wordsContainer.
  
  Return an array of integers ans, where ans[i] is the index of the string in wordsContainer that has the longest common suffix with wordsQuery[i].
  
  Example 1:
  Input: wordsContainer = ["abcd","bcd","xbcd"], wordsQuery = ["cd","bcd","xyz"]
  Output: [1,1,1]
  Explanation:
  Let's look at each wordsQuery[i] separately:
      For wordsQuery[0] = "cd", strings from wordsContainer that share the longest common suffix "cd" are at indices 0, 1, and 2. Among these, the answer is the string at index 1 because it has the shortest length of 3.
      For wordsQuery[1] = "bcd", strings from wordsContainer that share the longest common suffix "bcd" are at indices 0, 1, and 2. Among these, the answer is the string at index 1 because it has the shortest length of 3.
      For wordsQuery[2] = "xyz", there is no string from wordsContainer that shares a common suffix. Hence the longest common suffix is "", that is shared with strings at index 0, 1, and 2.
      Among these, the answer is the string at index 1 because it has the shortest length of 3.
  
  Example 2:
  Input: wordsContainer = ["abcdefgh","poiuygh","ghghgh"], wordsQuery = ["gh","acbfgh","acbfegh"]
  Output: [2,0,2]
  Explanation:
  Let's look at each wordsQuery[i] separately:
      For wordsQuery[0] = "gh", strings from wordsContainer that share the longest common suffix "gh" are at indices 0, 1, and 2. Among these, the answer is the string at index 2 because it has the shortest length of 6.
      For wordsQuery[1] = "acbfgh", only the string at index 0 shares the longest common suffix "fgh". Hence it is the answer, even though the string at index 2 is shorter.
      For wordsQuery[2] = "acbfegh", strings from wordsContainer that share the longest common suffix "gh" are at indices 0, 1, and 2. Among these, the answer is the string at index 2 because it has the shortest length of 6.
*/
impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let total: usize = words_container.iter().map(|w| w.len()).sum::<usize>() + 1;

        let mut children: Vec<[u32; 26]> = Vec::with_capacity(total);
        let mut best: Vec<(u32, i32)> = Vec::with_capacity(total);
        children.push([0; 26]);
        best.push((u32::MAX, -1));

        for (wi, w) in words_container.iter().enumerate() {
            let b = w.as_bytes();
            let wlen = b.len() as u32;
            let wi = wi as i32;
            let mut node = 0usize;

            unsafe {
                let r = best.get_unchecked_mut(0);
                if r.0 > wlen { *r = (wlen, wi); }
            }

            for &ch in b.iter().rev() {
                let c = (ch - b'a') as usize;
                let mut next = unsafe { *children.get_unchecked(node).get_unchecked(c) };
                if next == 0 {
                    next = children.len() as u32;
                    children.push([0; 26]);
                    best.push((wlen, wi));
                    unsafe { *children.get_unchecked_mut(node).get_unchecked_mut(c) = next; }
                    node = next as usize;
                    continue;
                }
                node = next as usize;
                unsafe {
                    let bn = best.get_unchecked_mut(node);
                    if bn.0 > wlen { *bn = (wlen, wi); }
                }
            }
        }

        let mut res = Vec::with_capacity(words_query.len());
        for q in &words_query {
            let mut node = 0usize;
            for &ch in q.as_bytes().iter().rev() {
                let next = unsafe { *children.get_unchecked(node).get_unchecked((ch - b'a') as usize) };
                if next == 0 { break; }
                node = next as usize;
            }
            res.push(unsafe { best.get_unchecked(node).1 });
        }
        res
    }
}
