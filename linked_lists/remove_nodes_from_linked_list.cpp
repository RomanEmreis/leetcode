/*
  You are given the head of a linked list.
  Remove every node which has a node with a greater value anywhere to the right side of it.
  Return the head of the modified linked list.

  Input: head = [5,2,13,3,8]
  Output: [13,8]
  Explanation: The nodes that should be removed are 5, 2 and 3.
  - Node 13 is to the right of node 5.
  - Node 13 is to the right of node 2.
  - Node 8 is to the right of node 3.
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
    ListNode* removeNodes(ListNode* head) {
        if (!head->next) return head;

        stack<ListNode*> s;
        s.push(head);

        ListNode* current = head->next;
        while (current) {
            s.push(current);
            current = current->next;
        }

        ListNode* maximum = nullptr;
        while (!s.empty()) {
            ListNode* node = s.top();
            s.pop();

            if (!maximum || node->val >= maximum->val) {
                node->next = maximum;
                maximum = node;
            }
        }

        return maximum;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
