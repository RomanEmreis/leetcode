/*
  You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.
  For each index i, names[i] and heights[i] denote the name and height of the ith person.
  Return names sorted in descending order by the people's heights.

  Example 1:
    Input: names = ["Mary","John","Emma"], heights = [180,165,170]
    Output: ["Mary","Emma","John"]
    Explanation: Mary is the tallest, followed by Emma and John.
  
  Example 2:
    Input: names = ["Alice","Bob","Bob"], heights = [155,185,150]
    Output: ["Bob","Alice","Bob"]
    Explanation: The first Bob is the tallest, followed by Alice and the second Bob.
*/
class Solution {
private:
    static struct {
        bool operator()(const pair<string, int>& x, const pair<string, int>& y) const { 
            return x.second > y.second;
        }
    } c;

public:
    vector<string> sortPeople(vector<string>& names, vector<int>& heights) {
        int n = names.size();
        vector<pair<string, int>> people(n);

        for (int i = 0; i < n; ++i) {
            people[i] = {names[i], heights[i]};
        }

        sort(people.begin(), people.end(), c);

        vector<string> result(n);
        for (int i = 0; i < n; ++i) {
            result[i] = people[i].first;
        }

        return result;
    }
};

static auto _ = [](){
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return nullptr;
}();
