/*
  There are n soldiers standing in a line. Each soldier is assigned a unique rating value.
  You have to form a team of 3 soldiers amongst them under the following rules:

  Choose 3 soldiers with index (i, j, k) with rating (rating[i], rating[j], rating[k]).
  A team is valid if: (rating[i] < rating[j] < rating[k]) or (rating[i] > rating[j] > rating[k]) where (0 <= i < j < k < n).
  
  Return the number of teams you can form given the conditions. (soldiers can be part of multiple teams).

  Example:
    Input: rating = [2,5,3,4,1]
    Output: 3
    Explanation: We can form three teams given the conditions. (2,3,4), (5,4,1), (5,3,1). 
*/
public class Solution {
    public int NumTeams(int[] rating) {
        int result = 0;

        for (int j = 1; j < rating.Length - 1; ++j) {
            int lessLeft = 0, lessRight = 0;
            int moreLeft = 0, moreRight = 0;

            for (int i = 0; i < j; ++i) {
                if (rating[i] < rating[j]) ++lessLeft;
                else if (rating[i] > rating[j]) ++moreLeft;
            }

            for (int k = j + 1; k < rating.Length; ++k) {
                if (rating[k] < rating[j]) ++lessRight;
                else if (rating[k] > rating[j]) ++moreRight;
            } 

            result += lessLeft * moreRight + moreLeft * lessRight;
        }

        return result;
    }
}
