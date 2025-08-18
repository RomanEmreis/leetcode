/*
  679. 24 Game
  
  You are given an integer array cards of length 4. You have four cards, each containing a number in the range [1, 9]. 
  You should arrange the numbers on these cards in a mathematical expression using the operators ['+', '-', '*', '/'] and the parentheses '(' and ')' to get the value 24.
  
  You are restricted with the following rules:
  * The division operator '/' represents real division, not integer division.
    For example, 4 / (1 - 2 / 3) = 4 / (1 / 3) = 12.
  * Every operation done is between two numbers. In particular, we cannot use '-' as a unary operator.
    For example, if cards = [1, 1, 1, 1], the expression "-1 - 1 - 1 - 1" is not allowed.
  * You cannot concatenate numbers together
    For example, if cards = [1, 2, 1, 2], the expression "12 + 12" is not valid.
  
  Return true if you can get such expression that evaluates to 24, and false otherwise.
  
  Example 1:
  Input: cards = [4,1,8,7]
  Output: true
  Explanation: (8-4) * (7-1) = 24
  
  Example 2:
  Input: cards = [1,2,1,2]
  Output: false
*/
impl Solution {
  const E: f32 = 10e-5;

  pub fn judge_point24(cards: Vec<i32>) -> bool {
      let mut vec = Vec::with_capacity(4);
      for card in cards {
          vec.push(card as f32);
      }
      return Self::check(&vec);
  }

  fn check(vec: &Vec<f32>) -> bool {
    let n = vec.len();
    if n == 1 {
        return (vec[0] - 24.0).abs() <= Self::E;
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
              continue;
            }
            let mut res: Vec<f32> = Vec::with_capacity(6);
            for k in 0..n {
                if k != i && k != j {
                    res.push(vec[k]);
                }
            }
            for op in 0..4 {
                if (op == 0 || op == 2) && i > j {
                    continue;
                }
                if op == 3 && vec[j] == 0.0 {
                    continue;
                }
                match op {
                    0 => res.push(vec[i] + vec[j]),
                    1 => res.push(vec[i] - vec[j]),
                    2 => res.push(vec[i] * vec[j]),
                    3 => res.push(vec[i] / vec[j]),
                    _ => {}
                }
                if Self::check(&res) {
                    return true;
                }
                res.pop();
            }
        }
    }
    false
  }
}
