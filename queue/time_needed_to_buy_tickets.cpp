class Solution {
public:
    int timeRequiredToBuy(vector<int>& tickets, int k) {
        int n = tickets.size();
        if (n == 1) return tickets[0];

        int count = 0;
        while (tickets[k] > 0) {
            for (int& t : tickets) {
                if (t > 0 && tickets[k] > 0) {
                    --t;
                    ++count;
                }
            }
        }

        return count;
    }
};
