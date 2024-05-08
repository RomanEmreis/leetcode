/*
  You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.
  The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:

  The 1st place athlete's rank is "Gold Medal".
  The 2nd place athlete's rank is "Silver Medal".
  The 3rd place athlete's rank is "Bronze Medal".
  For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").
  Return an array answer of size n where answer[i] is the rank of the ith athlete.

  Example:
    Input: score = [5,4,3,2,1]
    Output: ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
    Explanation: The placements are [1st, 2nd, 3rd, 4th, 5th].
*/
class Solution {
private:
    const string 
        gold = "Gold Medal",
        silver = "Silver Medal",
        bronze = "Bronze Medal";

public:
    vector<string> findRelativeRanks(vector<int>& score) {
        int n = score.size();
        if (n == 1) return {gold};

        auto cmp = [](const pair<int, int>& x, const pair<int, int>& y) { return x.second < y.second; };
        priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(cmp)> q(cmp);

        for (int i = 0; i < n; ++i) q.push({i, score[i]});

        vector<string> result(n);

        int place = 1;
        while (!q.empty()) {
            auto [index, score] = q.top();
            q.pop();

            if (place == 1) result[index] = gold;
            else if (place == 2) result[index] = silver;
            else if (place == 3) result[index] = bronze;
            else result[index] = to_string(place);

            ++place;
        }

        return result;
    }
};
