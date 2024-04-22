class Solution {
public:
    vector<int> deckRevealedIncreasing(vector<int>& deck) {
        int n = deck.size();
        if (n == 1) return deck;

        sort(deck.begin(), deck.end());

        vector<int> result(n);
        queue<int> indexes;

        for (int i = 0; i < n; ++i) indexes.push(i);

        for (const int& card : deck) {
            result[indexes.front()] = card;
            indexes.pop();

            if (!indexes.empty()) {
                indexes.push(indexes.front());
                indexes.pop();
            }
        }

        return result;
    }
};
