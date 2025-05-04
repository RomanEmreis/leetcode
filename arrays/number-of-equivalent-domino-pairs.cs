/*
  1128. Number of Equivalent Domino Pairs
  
  Given a list of dominoes, dominoes[i] = [a, b] is equivalent to dominoes[j] = [c, d] if and only if either (a == c and b == d), or (a == d and b == c) - that is, one domino can be rotated to be equal to another domino.
  
  Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length, and dominoes[i] is equivalent to dominoes[j].
  
  Example 1:
  Input: dominoes = [[1,2],[2,1],[3,4],[5,6]]
  Output: 1
  
  Example 2:
  Input: dominoes = [[1,2],[1,2],[1,1],[1,2],[2,2]]
  Output: 3
*/
public class Solution {
    public int NumEquivDominoPairs(int[][] dominoes) {
        int result = 0;
        Span<int> count = stackalloc int[100];
        foreach (int[] d in dominoes) {
            int x = d[0] < d[1] ? d[0] * 10 + d[1] : d[1] * 10 + d[0];
            result += count[x]++;
        }
        return result;
    }
}
