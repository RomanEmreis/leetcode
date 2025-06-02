/*
  135. Candy
  
  There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.
  
  You are giving candies to these children subjected to the following requirements:
      Each child must have at least one candy.
      Children with a higher rating get more candies than their neighbors.
  
  Return the minimum number of candies you need to have to distribute the candies to the children.
  
  Example 1:
  Input: ratings = [1,0,2]
  Output: 5
  Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
  
  Example 2:
  Input: ratings = [1,2,2]
  Output: 4
  Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
  The third child gets 1 candy because it satisfies the above two conditions.
*/
public class Solution {
    public int Candy(int[] ratings) {
        int n = ratings.Length;

        Spana<int> l = stackalloc int[n];
        Span<int> r = stackalloc int[n];

        l.Fill(1);
        r.Fill(1);

        for (int i = 1; i < n; ++i) {
            if (ratings[i] > ratings[i - 1]) {
                l[i] = l[i - 1] + 1;
            }
        }

        for (int i = n - 2; i >= 0; --i) {
            if (ratings[i] > ratings[i + 1]) {
                r[i] = r[i + 1] + 1;
            }
        }

        int result = 0;
        for (int i = 0; i < n; ++i) {
            result += Math.Max(l[i], r[i]);
        }
        return result;
    }
}
