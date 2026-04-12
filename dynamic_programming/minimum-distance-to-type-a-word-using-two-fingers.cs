/*
  1320. Minimum Distance to Type a Word Using Two Fingers
  
  You have a keyboard layout as shown above in the X-Y plane, where each English uppercase letter is located at some coordinate.
      For example, the letter 'A' is located at coordinate (0, 0), the letter 'B' is located at coordinate (0, 1), the letter 'P' is located at coordinate (2, 3) and the letter 'Z' is located at coordinate (4, 1).
  
  Given the string word, return the minimum total distance to type such string using only two fingers.
  
  The distance between coordinates (x1, y1) and (x2, y2) is |x1 - x2| + |y1 - y2|.
  
  Note that the initial positions of your two fingers are considered free so do not count towards your total distance, also your two fingers do not have to start at the first letter or the first two letters.
  
  Example 1:
  Input: word = "CAKE"
  Output: 3
  Explanation: Using two fingers, one optimal way to type "CAKE" is: 
  Finger 1 on letter 'C' -> cost = 0 
  Finger 1 on letter 'A' -> cost = Distance from letter 'C' to letter 'A' = 2 
  Finger 2 on letter 'K' -> cost = 0 
  Finger 2 on letter 'E' -> cost = Distance from letter 'K' to letter 'E' = 1 
  Total distance = 3
  
  Example 2:
  Input: word = "HAPPY"
  Output: 6
  Explanation: Using two fingers, one optimal way to type "HAPPY" is:
  Finger 1 on letter 'H' -> cost = 0
  Finger 1 on letter 'A' -> cost = Distance from letter 'H' to letter 'A' = 2
  Finger 2 on letter 'P' -> cost = 0
  Finger 2 on letter 'P' -> cost = Distance from letter 'P' to letter 'P' = 0
  Finger 1 on letter 'Y' -> cost = Distance from letter 'A' to letter 'Y' = 4
  Total distance = 6
*/
public class Solution {
    public int MinimumDistance(string word) {
        const int Free = 26;

        Span<int> dp = stackalloc int[27];
        dp.Fill(int.MaxValue);
        dp[Free] = 0;
        
        int n = word.Length;

        for (int i = 0; i < n - 1; ++i) {
            int curr = word[i] - 'A';
            int next = word[i + 1] - 'A';

            Span<int> newDp = stackalloc int[27];
            newDp.Fill(int.MaxValue);

            for (int j = 0; j < 27; ++j) {
                if (dp[j] == int.MaxValue)
                    continue;
                
                int cost = dp[j];
                int moveActive = cost + Dist(curr, next);
                if (moveActive < newDp[j])
                    newDp[j] = moveActive;

                int movePassive = cost + (j == Free ? 0 : Dist(j, next));
                if (movePassive < newDp[curr])
                    newDp[curr] = movePassive;
            }

            dp = newDp;
        }

        int res = int.MaxValue;
        foreach (int d in dp)
            if (res > d)
                res = d;

        return res;
    }

    private static int Dist(int a, int b) {
        var (ra, ca) = (a / 6, a % 6);
        var (rb, cb) = (b / 6, b % 6);

        return Math.Abs(ra - rb) + Math.Abs(ca - cb);
    }
}
