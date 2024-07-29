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
class Solution {
public:
    int numTeams(vector<int>& rating) {
        size_t n = rating.size();
        int result = 0;

        for (size_t j = 1; j < n - 1; ++j) {
            int l_left = 0, l_right = 0;
            int m_left = 0, m_right = 0;

            for (size_t i = 0; i < j; ++i) {
                if (rating[i] < rating[j]) ++l_left;
                else if (rating[i] > rating[j]) ++m_left;
            }

            for (size_t k = j + 1; k < n; ++k) {
                if (rating[k] < rating[j]) ++l_right;
                else if (rating[k] > rating[j]) ++m_right;
            }

            result += l_left * m_right + m_left * l_right;
        }

        return result;
    }
};
