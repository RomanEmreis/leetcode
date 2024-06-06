/*
  Alice has some number of cards and she wants to rearrange the cards into groups so that each group is of size groupSize, and consists of groupSize consecutive cards.
  Given an integer array hand where hand[i] is the value written on the ith card and an integer groupSize, return true if she can rearrange the cards, or false otherwise.

  Example 1:
    Input: hand = [1,2,3,6,2,3,4,7,8], groupSize = 3
    Output: true
    Explanation: Alice's hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]

  Example 2:
    Input: hand = [1,2,3,4,5], groupSize = 4
    Output: false
    Explanation: Alice's hand can not be rearranged into groups of 4.
*/
class Solution {
public:
    bool isNStraightHand(vector<int>& hand, int groupSize) {
        int n = hand.size();
        if (n % groupSize != 0) return false;

        sort(hand.begin(), hand.end());

        for (int i = 0; i < n; ++i) {
            if (hand[i] >= 0) {
                int count = 1, next = hand[i] + 1, j = i + 1;
                hand[i] = -1;
                while (j < n && count < groupSize) {
                    if (hand[j] == next) {
                        next = hand[j] + 1;
                        hand[j] = -1;
                        ++count;
                    }
                    ++j;
                }

                if (count != groupSize) return false;
            }
        }

        return true;
    }
};
