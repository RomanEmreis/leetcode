/*
  838. Push Dominoes
  
  There are n dominoes in a line, and we place each domino vertically upright. In the beginning, we simultaneously push some of the dominoes either to the left or to the right.
  After each second, each domino that is falling to the left pushes the adjacent domino on the left. Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.
  
  When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.
  For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.
  
  You are given a string dominoes representing the initial state where:
  dominoes[i] = 'L', if the ith domino has been pushed to the left,
  dominoes[i] = 'R', if the ith domino has been pushed to the right, and
  dominoes[i] = '.', if the ith domino has not been pushed.
  
  Return a string representing the final state.
  
  Example 1:
  Input: dominoes = "RR.L"
  Output: "RR.L"
  Explanation: The first domino expends no additional force on the second domino.
  
  Example 2:
  Input: dominoes = ".L.R...LR..L.."
  Output: "LL.RR.LLRRLL.."
*/
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut result = Vec::from_iter(dominoes.chars());

        let mut prev = 'L';
        let mut start = 0;

        for (index, domino) in dominoes.char_indices() {
            if domino == '.' {
                continue;
            }

            if domino == prev {
                while start != index {
                    result[start] = domino;
                    start += 1;
                }
            } else if domino == 'L' && prev == 'R' {
                let mut end = index - 1;
                while start < end {
                    result[start] = 'R';
                    result[end] = 'L';
                    start += 1;
                    end -= 1;
                }
            }
            start = index + 1;
            prev = domino;
        }

        if prev == 'R' {
            while start != dominoes.len() {
                result[start] = 'R';
                start += 1;
            }
        }

        result.iter().collect()
    }
}
