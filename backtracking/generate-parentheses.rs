/*
  22. Generate Parentheses
  
  Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
  
  Example 1:
  Input: n = 3
  Output: ["((()))","(()())","(())()","()(())","()()()"]
  
  Example 2:
  Input: n = 1
  Output: ["()"]
*/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut res = Vec::new();
        let mut s = String::with_capacity(n * 2);
        generate(&mut res, &mut s, 0, 0, n);
        res
    }
}

fn generate(
    res: &mut Vec<String>,
    s: &mut String,
    l: usize,
    r: usize,
    n: usize
) {
    if r == n {
        res.push(s.clone());
        return;
    }

    if l < n {
        s.push('(');
        generate(res, s, l + 1, r, n);
        s.pop();
    }
    if r < l {
        s.push(')');
        generate(res, s, l, r + 1, n);
        s.pop();
    }
}
