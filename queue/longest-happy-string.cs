/*
  A string s is called happy if it satisfies the following conditions:
  
  s only contains the letters 'a', 'b', and 'c'.
  s does not contain any of "aaa", "bbb", or "ccc" as a substring.
  s contains at most a occurrences of the letter 'a'.
  s contains at most b occurrences of the letter 'b'.
  s contains at most c occurrences of the letter 'c'.
  Given three integers a, b, and c, return the longest possible happy string. If there are multiple longest happy strings, return any of them. If there is no such string, return the empty string "".
  
  A substring is a contiguous sequence of characters within a string.
  
  Example 1:
    Input: a = 1, b = 1, c = 7
    Output: "ccaccbcc"
    Explanation: "ccbccacc" would also be a correct answer.

  Example 2:
    Input: a = 7, b = 1, c = 0
    Output: "aabaa"
    Explanation: It is the only correct answer in this case.
*/
public class Solution {
    public string LongestDiverseString(int a, int b, int c) {
        PriorityQueue<(char, int), int> pq = new();
        if (a > 0) pq.Enqueue(('a', a), -a);
        if (b > 0) pq.Enqueue(('b', b), -b);
        if (c > 0) pq.Enqueue(('c', c), -c);

        StringBuilder sb = new();
        
        while (pq.Count > 0) {
            var (ch, n) = pq.Dequeue();
            int length = sb.Length;

            if (length >= 2 && sb[^1] == ch && sb[^2] == ch) {
                if (pq.Count == 0) break;

                var (nextCh, nextN) = pq.Dequeue();
                sb.Append(nextCh);
                --nextN;

                if (nextN > 0) pq.Enqueue((nextCh, nextN), -nextN);

                pq.Enqueue((ch, n), -n);
            } else {
                sb.Append(ch);
                --n;

                if (n > 0) pq.Enqueue((ch, n), -n);
            }
        }

        return sb.ToString();
    }
}
