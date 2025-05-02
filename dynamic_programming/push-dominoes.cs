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
public class Solution {
    public string PushDominoes(string dominoes) {
        int n = dominoes.Length;
        Span<int> time = stackalloc int[n];
        time.Fill(-1);

        List<char>[] force = new List<char>[n];
        Queue<int> q = [];

        for (int i = 0; i < n; ++i) {
            if (dominoes[i] == '.') {
                force[i] = [];
                continue;
            }

            q.Enqueue(i);
            time[i] = 0;
            force[i] = [dominoes[i]];
        }

        Span<char> result = stackalloc char[n];
        result.Fill('.');

        while (q.TryDequeue(out int i)) {
            if (force[i].Count != 1) continue;

            char f = force[i][0];
            result[i] = f;

            int j = f == 'L' ? i - 1 : i + 1;
            if (j < 0 || j >= n) continue;
            int t = time[i];
            if (time[j] == -1) {
                q.Enqueue(j);
                time[j] = t + 1;
                force[j].Add(f);
            } else if (time[j] == t + 1) {
                force[j].Add(f);
            }
        }

        return new string(result);
    }
}
