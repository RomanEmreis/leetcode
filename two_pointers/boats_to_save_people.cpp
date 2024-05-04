/*
  You are given an array people where people[i] is the weight of the ith person, and an infinite number of boats where each boat can carry a maximum weight of limit.
  Each boat carries at most two people at the same time, provided the sum of the weight of those people is at most limit.

  Return the minimum number of boats to carry every given person.
*/
class Solution {
public:
    int numRescueBoats(vector<int>& people, int limit) {
        int n = people.size();
        if (n == 1) return n;

        sort(people.begin(), people.end());

        int l = 0, r = n - 1, result = 0;
        while (l <= r) {
            if (people[l] + people[r] <= limit) ++l;
            --r;
            ++result;
        }

        return result;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
