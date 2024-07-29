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
func numTeams(rating []int) int {
    n, result := len(rating), 0;

    for j := 1; j < n - 1; j++ {
        lessLeft, lessRight := 0, 0;
        moreLeft, moreRight := 0, 0;

        for i := 0; i < j; i++ {
            if rating[i] < rating[j] {
                lessLeft++;
            } else if rating[i] > rating[j] {
                moreLeft++;
            }a
        }

        for k := j + 1; k < n; k++ {
            if rating[k] < rating[j] {
                lessRight++;
            } else if rating[k] > rating[j] {
                moreRight++;
            }
        }

        result += lessLeft * moreRight + moreLeft * lessRight;
    }

    return result;
}
