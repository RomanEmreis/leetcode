/*
  A critical point in a linked list is defined as either a local maxima or a local minima.
  A node is a local maxima if the current node has a value strictly greater than the previous node and the next node.
  A node is a local minima if the current node has a value strictly smaller than the previous node and the next node.

  Note that a node can only be a local maxima/minima if there exists both a previous node and a next node.

  Given a linked list head, return an array of length 2 containing [minDistance, maxDistance] 
  where minDistance is the minimum distance between any two distinct critical points and maxDistance is the maximum distance between any two distinct critical points.
  If there are fewer than two critical points, return [-1, -1].

  Example 1:
    Input: head = [3,1]
    Output: [-1,-1]
    Explanation: There are no critical points in [3,1].

  Example 2:
    Input: head = [5,3,1,2,5,1,2]
    Output: [1,3]
    Explanation: There are three critical points:
    - [5,3,1,2,5,1,2]: The third node is a local minima because 1 is less than 3 and 2.
    - [5,3,1,2,5,1,2]: The fifth node is a local maxima because 5 is greater than 2 and 1.
    - [5,3,1,2,5,1,2]: The sixth node is a local minima because 1 is less than 5 and 2.
    The minimum distance is between the fifth and the sixth node. minDistance = 6 - 5 = 1.
    The maximum distance is between the third and the sixth node. maxDistance = 6 - 3 = 3.
*/
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    vector<int> nodesBetweenCriticalPoints(ListNode* head) {
        if (!head || !head->next) return {-1,-1};

        ListNode* curr = head->next;
        ListNode* prev = head;

        vector<int> cp;
        int i = 2, diff = INT_MAX;

        while (curr->next) {
            if ((curr->val > prev->val && curr->val > curr->next->val) ||
                (curr->val < prev->val && curr->val < curr->next->val)) {
                if (cp.size() > 0) diff = min(diff, i - cp.back());
                cp.push_back(i);
            }

            prev = curr;
            curr = curr->next;
            ++i;
        }

        if (cp.size() <= 1) return {-1,-1};

        return {diff, cp.back() - cp.front()};
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
