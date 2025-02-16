/*
  1718. Construct the Lexicographically Largest Valid Sequence
  
  Given an integer n, find a sequence that satisfies all of the following:
      The integer 1 occurs once in the sequence.
      Each integer between 2 and n occurs twice in the sequence.
      For every integer i between 2 and n, the distance between the two occurrences of i is exactly i.
  
  The distance between two numbers on the sequence, a[i] and a[j], is the absolute difference of their indices, |j - i|.
  Return the lexicographically largest sequence. It is guaranteed that under the given constraints, there is always a solution.
  
  A sequence a is lexicographically larger than a sequence b (of the same length) if in the first position where a and b differ, sequence a has a number greater than the corresponding number in b. For example, [0,1,9,0] is lexicographically larger than [0,1,5,6] because the first position they differ is at the third number, and 9 is greater than 5.
  
  Example 1:
  Input: n = 3
  Output: [3,1,2,3,2]
  Explanation: [2,3,2,1,3] is also a valid sequence, but [3,1,2,3,2] is the lexicographically largest valid sequence.
  
  Example 2:
  Input: n = 5
  Output: [5,3,1,4,3,5,2,4,2]
*/
public class Solution {
    public int[] ConstructDistancedSequence(int n) {
        Span<int> count = stackalloc int[n * 2];
        Span<int> path = stackalloc int[n * 2];
        count.Fill(2);
        count[1] = 1;

        Dfs(1, ref count, ref path);

        int[] result = new int[n * 2 - 1];
        for (int i = 0; i < result.Length; ++i) {
            result[i] = path[i + 1];
        }
        return result;

        bool Dfs(int x, ref Span<int> count, ref Span<int> path) {
            if (x == n * 2) return true;
            if (path[x] > 0) return Dfs(x + 1, ref count, ref path);

            for (int i = n; i > 1; --i) {
                if (count[i] > 0 && x + i < n * 2 && path[x + i] == 0) {
                    count[i] = 0;
                    path[x] = i;
                    path[x + i] = i;

                    if (Dfs(x + 1, ref count, ref path)) return true;

                    count[i] = 2;
                    path[x] = 0;
                    path[x + i] =  0;
                }
            }

            if (count[1] > 0) {
                path[x] = 1;
                count[1] = 0;
                
                if (Dfs(x + 1, ref count, ref path)) return true;

                count[1] = 1;
                path[x] = 0;
            }
            return false;
        }
    }
}
