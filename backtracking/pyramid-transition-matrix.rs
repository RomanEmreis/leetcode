/*
  756. Pyramid Transition Matrix
  
  You are stacking blocks to form a pyramid. Each block has a color, which is represented by a single letter.
  Each row of blocks contains one less block than the row beneath it and is centered on top.
  
  To make the pyramid aesthetically pleasing, there are only specific triangular patterns that are allowed. 
  A triangular pattern consists of a single block stacked on top of two blocks. 
  The patterns are given as a list of three-letter strings allowed, where the first two characters of a pattern represent 
  the left and right bottom blocks respectively, and the third character is the top block.
      For example, "ABC" represents a triangular pattern with a 'C' block stacked on top of an 'A' (left) and 'B' (right) block.
      Note that this is different from "BAC" where 'B' is on the left bottom and 'A' is on the right bottom.
  
  You start with a bottom row of blocks bottom, given as a single string, that you must use as the base of the pyramid.
  
  Given bottom and allowed, return true if you can build the pyramid all the way to 
  the top such that every triangular pattern in the pyramid is in allowed, or false otherwise.
  
  Example 1:
  Input: bottom = "BCD", allowed = ["BCC","CDE","CEA","FFF"]
  Output: true
  Explanation: The allowed triangular patterns are shown on the right.
  Starting from the bottom (level 3), we can build "CE" on level 2 and then build "A" on level 1.
  There are three triangular patterns in the pyramid, which are "BCC", "CDE", and "CEA". All are allowed.
  
  Example 2:
  Input: bottom = "AAAA", allowed = ["AAB","AAC","BCD","BBE","DEF"]
  Output: false
  Explanation: The allowed triangular patterns are shown on the right.
  Starting from the bottom (level 4), there are multiple ways to build level 3, but trying all the possibilites, 
  you will get always stuck before building level 1.
*/
use std::collections::HashSet;

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut top = vec![Vec::new(); 64];

        for a in allowed.into_iter() {
            let b = a.as_bytes();
            let prefix = ((b[1] - b'A') * 8 + b[0] - b'A') as usize;
            let block = (b[2] - b'A') as usize;

            top[prefix].push(block);
        }

        let n = bottom.len();
        let b = bottom.as_bytes();
        let mut value = 0;
        for i in 0..n {
            value += ((b[i] - b'A') as usize) << 3 * i;
        }

        let mut cache = HashSet::new();

        build(value, 0, 1, n, &mut cache, &top)
    }
}

fn build(
    mut value: usize, 
    mut next: usize, 
    mut index: usize, 
    mut len: usize, 
    cache: &mut HashSet<(usize, usize)>, 
    top: &[Vec<usize>]) -> bool {
    if len == 2 {
        return top[value].len() > 0
    }

    if index == len - 1 && cache.contains(&(value, len)) {
        return false;
    }

    let value_0 = value;
    let len_0 = len;
    let index_0 = index;

    if index == len {
        index = 1;
        len -= 1;
        value = next;
        next = 0;
    }

    let nexts = &top[(value >> (index - 1) * 3) & 63];
    for n in nexts.iter() {
        if index > 1 && top[(next >> (index - 2) * 3) & 7 | (n << 3)].len() == 0 {
            continue;
        }
        if build(value, next | (n << (index - 1) * 3), index + 1, len, cache, top) {
            return true;
        }
    }

    if index_0 == len_0 - 1 {
        cache.insert((value_0, len_0));
    }

    false
}
